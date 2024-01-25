use ropey::Rope;
use tower_lsp::{
    lsp_types::{CompletionItem, Position},
    Client,
};

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
        _: &Client,
    ) -> Vec<CompletionItem> {
        if let Some(table) = &self.table {
            let name_range = span_to_range(&table.1, rope).unwrap();
            if name_range.start <= position && position <= name_range.end {
                return get_completion_items_for_table_name(scope, &table.0);
            }
        };
        return vec![];
    }
}
