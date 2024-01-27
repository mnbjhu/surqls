use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, Position};

use crate::{
    ast::statement::statement::Statement,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::completions::has_completions::{HasCompletionItems, HasCompletionItemsForType},
    util::range::span_to_range,
};

impl HasCompletionItems for Statement {
    fn get_completion_items(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
    ) -> Vec<CompletionItem> {
        match &self {
            Statement::Create(create) => create.get_completion_items(scope, position, rope),
            Statement::Update(update) => update.get_completion_items(scope, position, rope),
            Statement::Delete(delete) => delete.get_completion_items(scope, position, rope),
            Statement::Select(select) => select.get_completion_items(scope, position, rope),
            Statement::Return(expr) => {
                let range = span_to_range(&expr.1, rope).unwrap();
                if range.start <= position && position <= range.end {
                    return expr
                        .0
                        .get_completion_items_for_type(scope, position, rope, &Type::Any);
                }
                vec![]
            }
            Statement::Define(define) => define.0.get_completion_items(scope, position, rope),
            Statement::Let(let_) => let_.get_completion_items(scope, position, rope),
            Statement::Invalid => vec![],
        }
    }
}
