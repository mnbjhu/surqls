use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

pub trait HasDiagnostic {
    fn diagnostics(&self, rope: &Rope) -> Vec<Diagnostic>;
}
