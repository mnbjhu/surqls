use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::declarations::scoped_item::ScopedItems;

pub fn get_completion_for_field(scope: &ScopedItems) -> Vec<CompletionItem> {
    let mut completions = vec![];
    for field in &scope.scoped_table.fields {
        completions.push(CompletionItem {
            label: field.name.clone(),
            kind: Some(CompletionItemKind::FIELD),
            detail: Some(format!("{}", field.ty)),
            ..Default::default()
        });
    }
    completions
}
