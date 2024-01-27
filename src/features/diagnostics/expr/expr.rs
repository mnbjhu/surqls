use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    ast::expr::{access::Access, parser::Expression, types::Typed},
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::{HasDiagnostic, HasDiagnosticsForType},
    util::{range::span_to_range, span::Spanned},
};

use super::{op::get_bin_op_diagnostics_for_type, variable::get_variable_diagnostics_for_type};

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
        let mut diagnostics = vec![];
        if let (Expression::Object(obj), s) = &self {
            match type_ {
                Type::Object(_) | Type::Any => {
                    diagnostics.extend((obj, s.clone()).diagnostics_for_type(rope, type_, scope));
                    return diagnostics;
                }
                Type::Option(opt) => {
                    if let Type::Object(_) = opt.as_ref() {
                        diagnostics
                            .extend((obj, s.clone()).diagnostics_for_type(rope, &opt, scope));
                        return diagnostics;
                    }
                }
                _ => {}
            }
            diagnostics.push(Diagnostic {
                range: span_to_range(&self.1, rope).unwrap(),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!(
                    "Expected type {}, found type {}",
                    type_,
                    self.0.get_type(scope)
                ),
                related_information: None,
                ..Default::default()
            });
            return diagnostics;
        }
        if let (Expression::Array(arr), s) = &self {
            return (arr, s.clone()).diagnostics_for_type(rope, type_, scope);
        }
        let actual_type = &self.0.get_type(scope);
        if !&type_.is_assignable_to(actual_type) {
            diagnostics.push(Diagnostic {
                range: span_to_range(&self.1, rope).unwrap(),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!(
                    "Expected type {}, found type {}",
                    type_,
                    self.0.get_type(scope)
                ),
                related_information: None,
                ..Default::default()
            });
        }
        match &self {
            (Expression::Identifier(ident), s) => {
                diagnostics.extend((ident, s.clone()).diagnostics_for_type(rope, type_, scope));
            }
            (Expression::Variable(var), s) => {
                diagnostics.extend(get_variable_diagnostics_for_type(
                    &(var, s.clone()),
                    rope,
                    type_,
                    scope,
                ));
            }
            (Expression::CodeBlock(block), s) => diagnostics.extend(block.diagnostics(rope, scope)),
            (Expression::Binary { left, right, op }, s) => diagnostics.extend(
                get_bin_op_diagnostics_for_type(left, right, op, rope, type_, scope),
            ),
            (Expression::Access { expr, access }, _) => match &access.0.as_ref() {
                Access::Property(name) => {
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
                            range: span_to_range(&expr.1, rope).unwrap(),
                            severity: Some(DiagnosticSeverity::ERROR),
                            message: "Expected object type".to_string(),
                            related_information: None,
                            ..Default::default()
                        });
                    }
                }
                Access::Index(index) => {
                    let ty = expr.0.get_type(scope);
                    if let Type::Array(_) = ty {
                        diagnostics.extend(index.diagnostics_for_type(rope, &Type::Int, scope));
                    } else {
                        diagnostics.push(Diagnostic {
                            range: span_to_range(&expr.1, rope).unwrap(),
                            severity: Some(DiagnosticSeverity::ERROR),
                            message: "Expected array type".to_string(),
                            related_information: None,
                            ..Default::default()
                        });
                    }
                }
            },
            (Expression::Literal(lit), s) => {
                diagnostics.extend((lit, self.1.clone()).diagnostics_for_type(rope, type_, scope));
            }
            _ => {}
        };
        diagnostics
    }
}
