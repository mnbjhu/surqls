use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, Position};

use crate::{
    ast::{expr::types::Typed, statement::crud::select::SelectStatement},
    declarations::{field::Field, scoped_item::ScopedItems, type_::Type},
    features::completions::{
        has_completions::HasCompletionItems, table_name::get_completion_items_for_table_name,
    },
    util::range::span_to_range,
};

impl HasCompletionItems for SelectStatement {
    fn get_completion_items(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
    ) -> Vec<CompletionItem> {
        let mut scope = scope.clone();
        if let Some(from) = &self.from {
            if let Some(table) = scope.table_definitions.get(&from.0) {
                scope.scoped_table = table.clone();
            }
            let name_range = span_to_range(&from.1, rope).unwrap();
            if name_range.start <= position && position <= name_range.end {
                return get_completion_items_for_table_name(&scope);
            }
        };
        for projection in &self.projections {
            let projection_range = span_to_range(&projection.1, rope).unwrap();
            if let Some(alias) = &projection.0.alias {
                scope.scoped_table.fields.retain(|f| f.name != alias.0);
                let ty = projection.0.expr.0.get_type(&scope);
                scope.scoped_table.fields.push(Field {
                    name: alias.0.clone(),
                    is_required: match ty {
                        Type::Option(_) => false,
                        _ => true,
                    },
                    ty,
                });
            }
            if projection_range.start <= position && position <= projection_range.end {
                return projection.0.get_completion_items(&scope, position, rope);
            }
        }
        for transform in &self.transforms {
            let transform_range = span_to_range(&transform.1, rope).unwrap();
            if transform_range.start <= position && position <= transform_range.end {
                return transform.0.get_completion_items(&scope, position, rope);
            }
        }
        return vec![];
    }
}
