use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, Documentation, InsertTextFormat, MarkupContent, MarkupKind,
};

use crate::declarations::scoped_item::ScopedItems;

pub fn get_completions_for_function(scope: &ScopedItems) -> Vec<CompletionItem> {
    let mut completions = vec![];
    for (name, def) in &scope.functions {
        let mut insert_text = name.clone();
        insert_text.push('(');
        for (i, arg) in def.args.iter().enumerate() {
            if i != 0 {
                insert_text.push_str(", ");
            }
            insert_text.push_str(format!("${{{}:{}}}", i + 1, arg.0).as_str());
        }
        insert_text.push(')');
        completions.push(CompletionItem {
            label: name.clone(),
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text: Some(insert_text),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: def.doc.clone().map(|doc| {
                Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: doc,
                })
            }),
            detail: Some(format!("{}{}", name, def)),
            ..Default::default()
        });
    }
    completions
}
