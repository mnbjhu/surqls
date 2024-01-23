use ropey::Rope;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    core::{parser::statement::statement::Statement, span::Spanned},
    features::symbols::Symbol,
    ls::util::range::span_to_range,
};

impl Symbol for &Spanned<Statement> {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol {
        let text = rope.slice(self.1.start..self.1.end);
        match &self.0 {
            Statement::Create(create) => (create, self.1).get_document_symbol(rope),
            Statement::Invalid => DocumentSymbol {
                name: format!("Invalid Statement: '{}'", text),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },

            Statement::Return(expr) => DocumentSymbol {
                name: "return".to_string(),
                kind: SymbolKind::FUNCTION,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
        }
    }
}
