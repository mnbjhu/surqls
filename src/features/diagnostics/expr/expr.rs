use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    core::{
        parser::{
            delcarations::{ScopedItems, Type},
            expr::{parser::Expression, types::Typed},
        },
        span::Spanned,
    },
    features::diagnostics::diagnostic::{HasDiagnostic, HasDiagnosticsForType},
    ls::util::range::span_to_range,
};

impl HasDiagnostic for Spanned<Expression> {
    fn diagnostics(&self, rope: &Rope, scope: &mut ScopedItems) -> Vec<Diagnostic> {
        self.diagnostics_for_type(rope, &Type::Any, scope)
    }
}

impl HasDiagnosticsForType for Spanned<Expression> {
    fn diagnostics_for_type(
        &self,
        rope: &Rope,
        type_: &Type,
        scope: &mut ScopedItems,
    ) -> Vec<Diagnostic> {
        match &self {
            (Expression::Object(obj), s) => {
                (obj, s.clone()).diagnostics_for_type(rope, type_, scope)
            }
            _ => {
                let found = &self.0.get_type();
                if !&type_.is_assignable_to(found) {
                    vec![Diagnostic {
                        range: span_to_range(&self.1, rope).unwrap(),
                        severity: Some(tower_lsp::lsp_types::DiagnosticSeverity::ERROR),
                        message: format!(
                            "Expected type {}, found type {}",
                            type_,
                            self.0.get_type()
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
