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
        let mut diagnostics = vec![];
        let mut scope = scope.clone();
        let ty = match &self.0.table {
            Some(table) => {
                diagnostics.extend(diagnostics_for_table_name(&table.0, &table.1, rope, &scope));
                if let Some(obj) = scope.table_definitions.get(&table.0) {
                    for field in &obj.fields {
                        scope.scoped_table.fields.retain(|f| f.name != field.name);
                        scope.scoped_table.fields.push(field.clone());
                    }
                    Type::Object(obj.clone())
                } else {
                    Type::Any
                }
            }
            None => Type::Any,
        };
        if let Some(content) = &self.0.content {
            diagnostics.extend(content.diagnostics_for_type(rope, &ty, &scope));
        };

        for transform in &self.0.transforms {
            diagnostics.extend(transform.0.diagnostics(rope, &scope));
        }
        diagnostics
    }
}
