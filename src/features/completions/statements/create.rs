use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, Position};

use crate::{
    ast::statement::crud::create::CreateStatement,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::completions::{
        has_completions::{HasCompletionItems, HasCompletionItemsForType},
        table_name::get_completion_items_for_table_name,
    },
    util::range::span_to_range,
};

impl HasCompletionItems for CreateStatement {
    fn get_completion_items(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
    ) -> Vec<CompletionItem> {
        let mut scope = scope.clone();
        if let Some(table) = &self.table {
            let name_range = span_to_range(&table.1, rope).unwrap();
            if name_range.start <= position && position <= name_range.end {
                return get_completion_items_for_table_name(&scope);
            }
            let ty = match scope.table_definitions.get(&table.0) {
                Some(obj) => {
                    for field in &obj.fields {
                        scope.scoped_table.fields.retain(|f| f.name != field.name);
                        scope.scoped_table.fields.push(field.clone());
                    }
                    Type::Object(obj.clone())
                }
                None => Type::Any,
            };
            if let Some(content) = &self.content {
                let content_range = span_to_range(&content.1, rope).unwrap();
                if content_range.start <= position && position <= content_range.end {
                    return content
                        .0
                        .get_completion_items_for_type(&scope, position, rope, &ty);
                }
            }
        };
        for transform in &self.transforms {
            let transform_range = span_to_range(&transform.1, rope).unwrap();
            if transform_range.start <= position && position <= transform_range.end {
                return transform.0.get_completion_items(&scope, position, rope);
            }
        }
        return vec![];
    }
}
