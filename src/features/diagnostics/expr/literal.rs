use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    ast::expr::literal::Literal,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::HasDiagnosticsForType,
    util::{range::span_to_range, span::Spanned},
};

impl HasDiagnosticsForType for Spanned<&Literal> {
    fn diagnostics_for_type(&self, rope: &Rope, _: &Type, scope: &ScopedItems) -> Vec<Diagnostic> {
        match self.0 {
            Literal::DateTime(s) => match chrono::DateTime::parse_from_rfc3339(s) {
                Ok(_) => vec![],
                Err(_) => vec![Diagnostic {
                    range: span_to_range(&self.1, rope).unwrap(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!("Invalid datetime '{}'", s),
                    ..Default::default()
                }],
            },
            _ => vec![],
        }
    }
}
