use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::HasDiagnosticsForType,
    util::{range::span_to_range, span::Spanned},
};

impl HasDiagnosticsForType for Spanned<&String> {
    fn diagnostics_for_type(&self, rope: &Rope, _: &Type, scope: &ScopedItems) -> Vec<Diagnostic> {
        match scope.scoped_table.get_field(&self.0) {
            Some(_) => {
                vec![]
            }
            None => vec![Diagnostic {
                range: span_to_range(&self.1, rope).unwrap(),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!("Unknown field '{}'", self.0),
                ..Default::default()
            }],
        }
    }
}
