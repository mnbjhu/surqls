use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::core::parser::delcarations::ScopedItems;

pub fn get_completion_items_for_table_name(
    scope: &mut ScopedItems,
    name: &str,
) -> Vec<CompletionItem> {
    scope
        .table_definitions
        .iter()
        .map(|(name, ty)| CompletionItem {
            label: name.to_string(),
            detail: Some(ty.to_string()),
            kind: Some(CompletionItemKind::STRUCT),
            ..Default::default()
        })
        .collect()
}
