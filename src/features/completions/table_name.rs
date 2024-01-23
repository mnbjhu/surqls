use ropey::Rope;
use tower_lsp::{
    lsp_types::{CompletionItem, CompletionItemKind, Position},
    Client,
};

use crate::core::parser::{delcarations::ScopedItems, table_name::TableName};

use super::has_completions::HasCompletionItems;

impl HasCompletionItems for TableName {
    fn get_completion_items(
        &self,
        scope: &mut ScopedItems,
        _: Position,
        _: &Rope,
        _: &Client,
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
}
