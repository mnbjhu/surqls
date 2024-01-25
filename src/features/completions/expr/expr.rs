use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, Position};

use crate::{
    ast::expr::parser::Expression,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::completions::has_completions::HasCompletionItemsForType,
};

impl HasCompletionItemsForType for Expression {
    fn get_completion_items_for_type(
        &self,
        scope: &mut ScopedItems,
        position: Position,
        rope: &Rope,
        type_: &Type,
    ) -> Vec<CompletionItem> {
        match self {
            Expression::Object(obj) => {
                obj.get_completion_items_for_type(scope, position, rope, type_)
            }
            _ => vec![],
        }
    }
}
