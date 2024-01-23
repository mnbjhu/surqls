use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    core::{
        parser::{delcarations::ScopedItems, table_name::TableName},
        span::Spanned,
    },
    features::diagnostics::diagnostic::HasDiagnostic,
    ls::util::range::span_to_range,
};

impl HasDiagnostic for Spanned<TableName> {
    fn diagnostics(&self, rope: &Rope, scope: &mut ScopedItems) -> Vec<Diagnostic> {
        let mut not_found = Diagnostic {
            range: span_to_range(&self.1, rope).unwrap(),
            severity: Some(DiagnosticSeverity::ERROR),
            related_information: None,
            ..Default::default()
        };
        match &self.0 {
            TableName::NotFound(name) => {
                not_found.message = format!("Table '{}' not found", name);
                vec![not_found]
            }
            TableName::Found(_, _) => vec![],
        }
    }
}
