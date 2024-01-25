use ropey::Rope;
use tower_lsp::{
    lsp_types::{CompletionItem, Position},
    Client,
};

use crate::declarations::{scoped_item::ScopedItems, type_::Type};

pub trait HasCompletionItems {
    fn get_completion_items(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
        client: &Client,
    ) -> Vec<CompletionItem>;
}

pub trait HasCompletionItemsForType {
    fn get_completion_items_for_type(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
        type_: &Type,
    ) -> Vec<CompletionItem>;
}
