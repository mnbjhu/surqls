use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::declarations::scoped_item::ScopedItems;

pub fn get_completion_items_for_table_name(scope: &ScopedItems) -> Vec<CompletionItem> {
    scope
        .table_definitions
        .iter()
        .map(|(name, _)| CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::STRUCT),
            ..Default::default()
        })
        .collect()
}
