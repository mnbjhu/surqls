use tower_lsp::lsp_types::CompletionItem;

use crate::{
    ast::statement::transform::Transform,
    declarations::type_::Type,
    features::completions::has_completions::{HasCompletionItems, HasCompletionItemsForType},
};

impl HasCompletionItems for Transform {
    fn get_completion_items(
        &self,
        scope: &crate::declarations::scoped_item::ScopedItems,
        position: tower_lsp::lsp_types::Position,
        rope: &ropey::Rope,
    ) -> Vec<CompletionItem> {
        let expr = match self {
            Transform::Where(Some((e, _))) => e,
            Transform::Limit(Some((e, _))) => e,
            Transform::Skip(Some((e, _))) => e,
            _ => return vec![],
        };
        expr.get_completion_items_for_type(scope, position, rope, &Type::Any)
    }
}
