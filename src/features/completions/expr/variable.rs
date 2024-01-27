use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::declarations::scoped_item::ScopedItems;

pub fn get_completion_for_variable(scope: &ScopedItems) -> Vec<CompletionItem> {
    let mut completions = vec![];
    for (name, ty) in &scope.variables {
        completions.push(CompletionItem {
            label: format!("${}", name),
            kind: Some(CompletionItemKind::VARIABLE),
            detail: Some(format!("{}", ty)),
            ..Default::default()
        });
    }
    completions
}
