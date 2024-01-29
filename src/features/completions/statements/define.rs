use ropey::Rope;
use tower_lsp::lsp_types::{CompletionItem, Position};

use crate::{
    ast::statement::define::DefineStatement,
    declarations::scoped_item::ScopedItems,
    features::completions::{
        has_completions::HasCompletionItems, table_name::get_completion_items_for_table_name,
    },
    util::range::span_to_range,
};

impl HasCompletionItems for DefineStatement {
    fn get_completion_items(
        &self,
        scope: &ScopedItems,
        position: Position,
        rope: &Rope,
    ) -> Vec<CompletionItem> {
        match &self {
            DefineStatement::Table(table) => {
                vec![]
            }
            DefineStatement::Field(field) => {
                if let Some(table_name) = &field.0.table_name {
                let table_name_range = span_to_range(&table_name.1, rope).unwrap();
                    if table_name_range.start <= position && position <= table_name_range.end {
                        return get_completion_items_for_table_name(scope);
                    }

                    if let Some(type_) = &field.0.type_ {
                        let type_range = span_to_range(&type_.1, rope).unwrap();
                        if type_range.start <= position && position <= type_range.end {
                            return type_.0.get_completion_items(scope, position, rope);
                        }
                    }
                };
                vec![]
            }
        }
    }
}
