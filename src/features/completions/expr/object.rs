use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Position};

use crate::{
    ast::expr::object::ObjectEntry,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::completions::has_completions::HasCompletionItemsForType,
    util::{range::span_to_range, span::Spanned},
};

impl HasCompletionItemsForType for Vec<Spanned<ObjectEntry>> {
    fn get_completion_items_for_type(
        &self,
        scope: &mut ScopedItems,
        position: Position,
        rope: &ropey::Rope,
        type_: &Type,
    ) -> Vec<CompletionItem> {
        let mut items = vec![];
        match type_ {
            Type::Object(obj) => {
                for (entry, span) in self {
                    let range = span_to_range(&span, rope).unwrap();
                    if range.start <= position && position <= range.end {
                        let ObjectEntry { key, value } = entry;
                        let key_range = span_to_range(&key.1, rope).unwrap();
                        let current = &self
                            .into_iter()
                            .map(|x| x.0.key.0.to_string())
                            .collect::<Vec<_>>();
                        let mut missing = obj.fields.clone().into_iter().collect::<Vec<_>>();
                        missing.retain(|x| !current.contains(&x.name));
                        if key_range.start <= position && position <= key_range.end {
                            let cmps = missing
                                .iter()
                                .map(|f| CompletionItem {
                                    label: f.name.clone(),
                                    kind: Some(CompletionItemKind::FIELD),
                                    detail: Some(format!("{}", f.ty)),
                                    ..Default::default()
                                })
                                .collect::<Vec<_>>();
                            items.extend(cmps);
                        }
                        if let Some(value) = value {
                            let value_range = span_to_range(&value.1, rope).unwrap();
                            if value_range.start <= position && position <= value_range.end {
                                items.extend(
                                    value.0.get_completion_items_for_type(
                                        scope,
                                        position,
                                        rope,
                                        obj.get_field(&key.0.to_string())
                                            .map(|x| &x.ty)
                                            .unwrap_or(&Type::Any),
                                    ),
                                );
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        items
    }
}
