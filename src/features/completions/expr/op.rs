use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, Position};

use crate::{
    ast::expr::{op::BinaryOperator, parser::Expression},
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::completions::has_completions::HasCompletionItemsForType,
    util::{range::span_to_range, span::Spanned},
};

pub fn get_completions_for_binary(
    scope: &ScopedItems,
    position: Position,
    rope: &Rope,
    type_: &Type,
    left: &Box<Spanned<Expression>>,
    op: &Spanned<BinaryOperator>,
    right: &Box<Spanned<Expression>>,
) -> Vec<CompletionItem> {
    let left_range = span_to_range(&left.1, rope).unwrap();
    if left_range.start <= position && position <= left_range.end {
        return left
            .as_ref()
            .0
            .get_completion_items_for_type(scope, position, rope, type_);
    }
    let right_range = span_to_range(&right.1, rope).unwrap();
    if right_range.start <= position && position <= right_range.end {
        return right
            .as_ref()
            .0
            .get_completion_items_for_type(scope, position, rope, type_);
    }
    vec![]
}
