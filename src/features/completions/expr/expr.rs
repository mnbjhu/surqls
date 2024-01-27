use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Position};

use crate::{
    ast::expr::{access::Access, parser::Expression, types::Typed},
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::completions::has_completions::{HasCompletionItems, HasCompletionItemsForType},
    util::range::span_to_range,
};

use super::{
    ident::get_completion_for_field, op::get_completions_for_binary,
    variable::get_completion_for_variable,
};

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
            Expression::Array(arr) => {
                let expected = match type_ {
                    Type::Array(ty) => ty.as_ref(),
                    _ => &Type::Any,
                };
                for item in arr {
                    let range = span_to_range(&item.1, rope).unwrap();
                    if range.start <= position && position <= range.end {
                        return item
                            .0
                            .get_completion_items_for_type(scope, position, rope, expected);
                    }
                }
                vec![]
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
            Expression::Binary { left, op, right } => {
                get_completions_for_binary(scope, position, rope, type_, left, op, right)
            }
            Expression::Variable(_) => get_completion_for_variable(scope),
            Expression::CodeBlock(block) => block.get_completion_items(scope, position, rope),
            Expression::Call { name, args } => {
                vec![
                    CompletionItem {
                        label: "test::function".to_string(),
                        kind: Some(CompletionItemKind::FUNCTION),
                        ..Default::default()
                    },
                    CompletionItem {
                        label: "testfunction".to_string(),
                        kind: Some(CompletionItemKind::FUNCTION),
                        ..Default::default()
                    },
                ]
            }
            _ => vec![],
        }
    }
}
