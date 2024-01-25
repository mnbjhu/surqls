use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    ast::expr::{access::Access, parser::Expression, types::Typed},
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::{HasDiagnostic, HasDiagnosticsForType},
    util::{range::span_to_range, span::Spanned},
};

impl HasDiagnostic for Spanned<Expression> {
    fn diagnostics(&self, rope: &Rope, scope: &ScopedItems) -> Vec<Diagnostic> {
        self.diagnostics_for_type(rope, &Type::Any, scope)
    }
}

impl HasDiagnosticsForType for Spanned<Expression> {
    fn diagnostics_for_type(
        &self,
        rope: &Rope,
        type_: &Type,
        scope: &ScopedItems,
    ) -> Vec<Diagnostic> {
        match &self {
            (Expression::Object(obj), s) => {
                (obj, s.clone()).diagnostics_for_type(rope, type_, scope)
            }
            (Expression::Array(arr), s) => {
                (arr, s.clone()).diagnostics_for_type(rope, type_, scope)
            }
            (Expression::Identifier(ident), s) => {
                (ident, s.clone()).diagnostics_for_type(rope, type_, scope)
            }
            (Expression::Access { expr, access }, _) => match &access.0.as_ref() {
                Access::Property(name) => {
                    let mut diagnostics = vec![];
                    let ty = expr.0.get_type(scope);
                    if let Type::Object(obj) = ty {
                        let prop = obj.get_field(name);
                        if prop.is_none() {
                            diagnostics.push(Diagnostic {
                                range: span_to_range(&self.1, rope).unwrap(),
                                severity: Some(DiagnosticSeverity::ERROR),
                                message: format!("Property '{}' does not exist", name),
                                related_information: None,
                                ..Default::default()
                            });
                        }
                    } else {
                        diagnostics.push(Diagnostic {
                            range: span_to_range(&self.1, rope).unwrap(),
                            severity: Some(DiagnosticSeverity::ERROR),
                            message: "Expected object type".to_string(),
                            related_information: None,
                            ..Default::default()
                        });
                    }
                    diagnostics
                }
                _ => vec![],
            },
            _ => {
                let found = &self.0.get_type(scope);
                if !&type_.is_assignable_to(found) {
                    vec![Diagnostic {
                        range: span_to_range(&self.1, rope).unwrap(),
                        severity: Some(tower_lsp::lsp_types::DiagnosticSeverity::ERROR),
                        message: format!(
                            "Expected type {}, found type {}",
                            type_,
                            self.0.get_type(scope)
                        ),
                        related_information: None,
                        ..Default::default()
                    }]
                } else {
                    vec![]
                }
            }
        }
    }
}
