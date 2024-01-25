use ropey::Rope;
use tower_lsp::{
    lsp_types::{CompletionItem, Position},
    Client,
};

use crate::{
    ast::statement::statement::Statement,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::completions::has_completions::{HasCompletionItems, HasCompletionItemsForType},
};

impl HasCompletionItems for Statement {
    fn get_completion_items(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
        client: &Client,
    ) -> Vec<CompletionItem> {
        match &self {
            Statement::Create(create) => create.get_completion_items(scope, position, rope, client),
            Statement::Update(update) => update.get_completion_items(scope, position, rope, client),
            Statement::Delete(delete) => delete.get_completion_items(scope, position, rope, client),
            Statement::Select(select) => select.get_completion_items(scope, position, rope, client),
            Statement::Return(expr) => {
                expr.0
                    .get_completion_items_for_type(scope, position, rope, &Type::Any)
            }
            Statement::Define(define) => {
                define.0.get_completion_items(scope, position, rope, client)
            }
            Statement::Invalid => vec![],
        }
    }
}
