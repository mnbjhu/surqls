use ropey::Rope;
use tower_lsp::{
    lsp_types::{CompletionItem, Position},
    Client,
};

use crate::{
    ast::projection::Projection,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::completions::has_completions::{HasCompletionItems, HasCompletionItemsForType},
    util::range::span_to_range,
};

impl HasCompletionItems for Projection {
    fn get_completion_items(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
        _: &Client,
    ) -> Vec<CompletionItem> {
        let expression_range = span_to_range(&self.expr.1, rope).unwrap();
        if expression_range.start <= position && position <= expression_range.end {
            return self
                .expr
                .0
                .get_completion_items_for_type(scope, position, rope, &Type::Any);
        }

        return vec![];
    }
}
