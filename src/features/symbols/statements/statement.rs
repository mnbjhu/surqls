use ropey::Rope;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    ast::statement::statement::Statement,
    features::symbols::Symbol,
    util::{range::span_to_range, span::Spanned},
};

impl Symbol for &Spanned<Statement> {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol {
        match &self.0 {
            Statement::Create(create) => (create, self.1).get_document_symbol(rope),
            Statement::Update(update) => (update, self.1).get_document_symbol(rope),
            Statement::Delete(delete) => (delete, self.1).get_document_symbol(rope),
            Statement::Select(select) => (select, self.1).get_document_symbol(rope),
            Statement::Return(_) => DocumentSymbol {
                name: "return".to_string(),
                kind: SymbolKind::FUNCTION,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
            Statement::Define(_) => DocumentSymbol {
                name: "define".to_string(),
                kind: SymbolKind::FUNCTION,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
            Statement::Invalid => DocumentSymbol {
                name: "invalid".to_string(),
                kind: SymbolKind::NULL,
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
