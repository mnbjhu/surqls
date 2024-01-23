use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    core::{
        parser::{
            delcarations::ScopedItems, statement::create_statement::CreateStatement,
            table_name::TableName,
        },
        span::Spanned,
    },
    features::diagnostics::diagnostic::{HasDiagnostic, HasDiagnosticsForType},
};

impl HasDiagnostic for Spanned<&CreateStatement> {
    fn diagnostics(&self, rope: &Rope, scope: &mut ScopedItems) -> Vec<Diagnostic> {
        return match &self.0.table {
            Some(table) => {
                let mut diags = table.diagnostics(rope, scope);
                if let Some(content) = &self.0.content {
                    match &table.0 {
                        TableName::Found(name, ty) => {
                            diags.extend(content.diagnostics_for_type(rope, ty, scope));
                        }
                        _ => {}
                    };
                };
                diags
            }
            None => vec![],
        };
    }
}
