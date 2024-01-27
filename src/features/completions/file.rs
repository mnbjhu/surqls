use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, Position};

use crate::{
    ast::{expr::types::Typed, parser::File, statement::statement::Statement},
    declarations::scoped_item::ScopedItems,
    util::range::span_to_range,
};

use super::has_completions::HasCompletionItems;

impl HasCompletionItems for File {
    fn get_completion_items(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
    ) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        let mut scope = scope.clone();
        for statement in self {
            let range = span_to_range(&statement.1, rope).unwrap();
            if let Statement::Let(let_) = &statement.0 {
                if range.start <= position && position <= range.end {
                    completions.extend(statement.0.get_completion_items(&scope, position, rope));
                }
                if let Some(name) = &let_.name {
                    if let Some(value) = &let_.value {
                        let ty = value.0.get_type(&scope);
                        scope.variables.insert(name.0.clone(), ty);
                    }
                }
            } else {
                if range.start <= position && position <= range.end {
                    completions.extend(statement.0.get_completion_items(&scope, position, rope));
                }
            }
        }
        completions
    }
}
