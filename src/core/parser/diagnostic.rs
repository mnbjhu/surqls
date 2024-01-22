use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use super::delcarations::ScopedItems;

pub trait HasDiagnostic {
    fn diagnostics(&self, rope: &Rope, scope: &mut ScopedItems) -> Vec<Diagnostic>;
}
