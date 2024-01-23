use ropey::Rope;
use tower_lsp::lsp_types::DocumentSymbol;

pub mod statements;

pub trait Symbol {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol;
}
