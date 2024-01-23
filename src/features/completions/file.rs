use ropey::Rope;
use tower_lsp::{
    lsp_types::{CompletionItem, Position},
    Client,
};

use crate::{
    core::parser::{delcarations::ScopedItems, parser::File},
    ls::util::range::span_to_range,
};

use super::has_completions::HasCompletionItems;

impl HasCompletionItems for File {
    fn get_completion_items(
        &self,
        scope: &mut ScopedItems,
        position: Position,
        rope: &Rope,
        client: &Client,
    ) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        for statement in self {
            let range = span_to_range(&statement.1, rope).unwrap();
            if range.start <= position && position <= range.end {
                completions.extend(
                    statement
                        .0
                        .get_completion_items(scope, position, rope, client),
                );
            }
        }
        completions
    }
}
