use ropey::Rope;
use tower_lsp::lsp_types::DocumentSymbol;

pub trait Symbol {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol;
}
