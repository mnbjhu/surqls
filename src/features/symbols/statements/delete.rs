use ropey::Rope;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    ast::statement::crud::delete::DeleteStatement,
    features::symbols::Symbol,
    util::{range::span_to_range, span::Spanned},
};

impl Symbol for Spanned<&DeleteStatement> {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol {
        let mut children = Vec::new();
        for transform in &self.0.transforms {
            children.push(transform.get_document_symbol(rope));
        }
        DocumentSymbol {
            name: format!(
                "DELETE {}",
                self.0.table.clone().map(|x| x.0).unwrap_or("".to_string())
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
