use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, Position};

use crate::{
    ast::statement::crud::delete::DeleteStatement,
    declarations::scoped_item::ScopedItems,
    features::completions::{
        has_completions::HasCompletionItems, table_name::get_completion_items_for_table_name,
    },
    util::range::span_to_range,
};

impl HasCompletionItems for DeleteStatement {
    fn get_completion_items(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
    ) -> Vec<CompletionItem> {
        let mut scope = scope.clone();
        if let Some(table) = &self.table {
            let name_range = span_to_range(&table.1, rope).unwrap();
            if name_range.start <= position && position <= name_range.end {
                return get_completion_items_for_table_name(&scope);
            }
            match scope.table_definitions.get(&table.0) {
                Some(obj) => {
                    for field in &obj.fields {
                        scope.scoped_table.fields.retain(|f| f.name != field.name);
                        scope.scoped_table.fields.push(field.clone());
                    }
                }
                None => {}
            };
        };
        for transform in &self.transforms {
            let transform_range = span_to_range(&transform.1, rope).unwrap();
            if transform_range.start <= position && position <= transform_range.end {
                return transform.0.get_completion_items(&scope, position, rope);
            }
        }
        return vec![];
    }
}
