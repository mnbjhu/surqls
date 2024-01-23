use ropey::Rope;
use tower_lsp::{
    lsp_types::{CompletionItem, Position},
    Client,
};

use crate::{
    core::parser::{
        delcarations::{ScopedItems, Type},
        statement::statement::Statement,
    },
    features::completions::has_completions::{HasCompletionItems, HasCompletionItemsForType},
};

impl HasCompletionItems for Statement {
    fn get_completion_items(
        &self,
        scope: &mut ScopedItems,
        position: Position,
        rope: &Rope,
        client: &Client,
    ) -> Vec<CompletionItem> {
        match &self {
            Statement::Create(create) => create.get_completion_items(scope, position, rope, client),
            Statement::Return(expr) => {
                expr.0
                    .get_completion_items_for_type(scope, position, rope, &Type::Any)
            }
            _ => vec![],
        }
    }
}
