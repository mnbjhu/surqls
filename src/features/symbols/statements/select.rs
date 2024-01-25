use ropey::Rope;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    ast::statement::crud::select::SelectStatement,
    features::symbols::Symbol,
    util::{range::span_to_range, span::Spanned},
};

impl Symbol for Spanned<&SelectStatement> {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol {
        let mut children = Vec::new();
        if let Some(from) = &self.0.from {
            children.push(DocumentSymbol {
                name: format!("FROM {}", from.0),
                kind: SymbolKind::STRUCT,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&from.1, rope).unwrap(),
                selection_range: span_to_range(&from.1, rope).unwrap(),
                children: None,
            });
        }
        for transform in &self.0.transforms {
            children.push(transform.get_document_symbol(rope));
        }
        DocumentSymbol {
            name: format!(
                "SELECT {}",
                self.0.from.clone().map(|x| x.0).unwrap_or("".to_string())
            ),
            kind: SymbolKind::STRUCT,
            tags: None,
            detail: None,
            deprecated: None,
            range: span_to_range(&self.1, rope).unwrap(),
            selection_range: span_to_range(&self.1, rope).unwrap(),
            children: Some(children),
        }
    }
}
