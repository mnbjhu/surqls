use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, Position};

use crate::{
    ast::statement::let_::LetStatement,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::completions::has_completions::{HasCompletionItems, HasCompletionItemsForType},
    util::range::span_to_range,
};

impl HasCompletionItems for LetStatement {
    fn get_completion_items(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
    ) -> Vec<CompletionItem> {
        if let Some(expr) = &self.value {
            let expr_range = span_to_range(&expr.1, rope).unwrap();
            if expr_range.start <= position && position <= expr_range.end {
                return expr
                    .0
                    .get_completion_items_for_type(&scope, position, rope, &Type::Any);
            }
        }
        return vec![];
    }
}
