use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat, Position};

use crate::{
    ast::type_::Type, declarations::scoped_item::ScopedItems,
    features::completions::table_name::get_completion_items_for_table_name,
    util::range::span_to_range,
};

use super::has_completions::HasCompletionItems;

impl HasCompletionItems for Type {
    fn get_completion_items(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
    ) -> Vec<CompletionItem> {
        let name_range = span_to_range(&self.name.1, rope).unwrap();
        if name_range.start <= position && position <= name_range.end {
            let primatives = vec![
                "any", "bool", "int", "float", "string", "date", "datetime", "duration", "object",
            ];
            let mut items = primatives
                .iter()
                .map(|name| CompletionItem {
                    label: name.to_string(),
                    kind: Some(CompletionItemKind::TYPE_PARAMETER),
                    ..Default::default()
                })
                .collect::<Vec<_>>();
            let generic = vec!["array", "option", "record"];
            items.extend(generic.iter().map(|name| CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                insert_text: Some(format!("{}<${{1:any}}>", name)),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            }));
            return items;
        }
        for arg in &self.args {
            let arg_range = span_to_range(&arg.1, rope).unwrap();
            if arg_range.start <= position && position <= arg_range.end {
                if &self.name.0 == "record" {
                    return get_completion_items_for_table_name(scope);
                }
                return arg.0.get_completion_items(scope, position, rope);
            }
        }
        vec![]
    }
}
