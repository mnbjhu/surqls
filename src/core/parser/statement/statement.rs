use ropey::Rope;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    core::{parser::symbol::Symbol, span::Spanned},
    ls::util::range::span_to_range,
};

use super::crud_statement::CrudStatement;

pub enum Statement {
    Crud(CrudStatement),
    Invalid,
}

impl Symbol for &Spanned<Statement> {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol {
        match &self.0 {
            Statement::Crud(crud) => (crud, self.1).get_document_symbol(rope),
            Statement::Invalid => DocumentSymbol {
                name: "Invalid Statement".to_string(),
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
