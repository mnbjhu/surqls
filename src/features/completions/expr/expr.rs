use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Position};

use crate::{
    ast::expr::{access::Access, parser::Expression, types::Typed},
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::completions::has_completions::HasCompletionItemsForType,
};

use super::ident::get_completion_for_field;

impl HasCompletionItemsForType for Expression {
    fn get_completion_items_for_type(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
        type_: &Type,
    ) -> Vec<CompletionItem> {
        match self {
            Expression::Object(obj) => {
                obj.get_completion_items_for_type(scope, position, rope, type_)
            }
            Expression::Identifier(_) => get_completion_for_field(scope),
            Expression::Access { expr, access } => match access.0.as_ref() {
                Access::Property(_) => {
                    let ty = expr.0.get_type(scope);
                    if let Type::Object(obj) = ty {
                        obj.fields
                            .into_iter()
                            .map(|x| CompletionItem {
                                label: x.name.clone(),
                                detail: Some(format!("{}", x.ty)),
                                kind: Some(CompletionItemKind::FIELD),
                                ..Default::default()
                            })
                            .collect::<Vec<_>>()
                    } else {
                        vec![]
                    }
                }
                Access::Index(_) => vec![],
            },
            _ => vec![],
        }
    }
}
