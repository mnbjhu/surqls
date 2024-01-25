use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    ast::statement::crud::delete::DeleteStatement, declarations::scoped_item::ScopedItems,
    features::diagnostics::diagnostic::HasDiagnostic, util::span::Spanned,
};

use super::table_name::diagnostics_for_table_name;

impl HasDiagnostic for Spanned<&DeleteStatement> {
    fn diagnostics(&self, rope: &Rope, scope: &ScopedItems) -> Vec<Diagnostic> {
        return match &self.0.table {
            Some(table) => diagnostics_for_table_name(&table.0, &table.1, rope, scope),
            None => vec![],
        };
    }
}
