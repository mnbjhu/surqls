use ropey::Rope;
use tower_lsp::{
    lsp_types::{CompletionItem, Position},
    Client,
};

use crate::{
    core::parser::{delcarations::ScopedItems, statement::create_statement::CreateStatement},
    features::completions::{
        has_completions::{HasCompletionItems, HasCompletionItemsForType},
        table_name::get_completion_items_for_table_name,
    },
    ls::util::range::span_to_range,
};

impl HasCompletionItems for CreateStatement {
    fn get_completion_items(
        &self,
        scope: &mut ScopedItems,
        position: Position,
        rope: &Rope,
        client: &Client,
    ) -> Vec<CompletionItem> {
        if let Some(table) = &self.table {
            let name_range = span_to_range(&table.1, rope).unwrap();
            if name_range.start <= position && position <= name_range.end {
                return get_completion_items_for_table_name(scope, &table.0);
            }
            if let Some(content) = &self.content {
                let content_range = span_to_range(&content.1, rope).unwrap();
                match scope.table_definitions.get(&table.0) {
                    Some(ty) => {
                        if content_range.start <= position && position <= content_range.end {
                            return content.0.get_completion_items_for_type(
                                scope,
                                position,
                                rope,
                                &ty.clone(),
                            );
                        }
                    }
                    None => {}
                }
            }
        };
        return vec![];
    }
}
