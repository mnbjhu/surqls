use ropey::Rope;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    ast::statement::transform::Transform,
    features::symbols::Symbol,
    util::{range::span_to_range, span::Spanned},
};

impl Symbol for &Spanned<Transform> {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol {
        let name = match &self.0 {
            Transform::Where(_) => "where",
            Transform::Limit(_) => "limit",
            Transform::Skip(_) => "skip",
            Transform::Invalid(_) => "invalid",
        };
        DocumentSymbol {
            name: name.to_string(),
            kind: SymbolKind::FUNCTION,
            tags: None,
            detail: None,
            deprecated: None,
            range: crate::util::range::span_to_range(&self.1, rope).unwrap(),
            selection_range: span_to_range(&self.1, rope).unwrap(),
            children: None,
        }
    }
}
