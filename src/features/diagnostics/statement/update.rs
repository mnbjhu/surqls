use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    ast::statement::crud::update::UpdateStatement,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::{HasDiagnostic, HasDiagnosticsForType},
    util::span::Spanned,
};

use super::table_name::diagnostics_for_table_name;

impl HasDiagnostic for Spanned<&UpdateStatement> {
    fn diagnostics(&self, rope: &Rope, scope: &ScopedItems) -> Vec<Diagnostic> {
        return match &self.0.table {
            Some(table) => {
                let mut diags = diagnostics_for_table_name(&table.0, &table.1, rope, scope);
                if let Some(content) = &self.0.content {
                    let def = scope.table_definitions.get(&table.0);
                    let ty = match def {
                        Some(ty) => Type::Object(ty.clone()),
                        None => Type::Any,
                    };
                    diags.extend(content.diagnostics_for_type(rope, &ty, scope));
                };
                diags
            }
            None => vec![],
        };
    }
}
