use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, Position};

use super::delcarations::ScopedItems;

pub trait HasCompletionItems {
    fn get_completion_items(
        &self,
        scope: &mut ScopedItems,
        position: Position,
        rope: &Rope,
    ) -> Vec<CompletionItem>;
}
