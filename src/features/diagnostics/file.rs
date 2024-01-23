use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::core::parser::{delcarations::ScopedItems, parser::File};

use super::diagnostic::HasDiagnostic;

impl HasDiagnostic for File {
    fn diagnostics(&self, rope: &Rope, scope: &mut ScopedItems) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for statement in self {
            diagnostics.extend(statement.diagnostics(rope, scope));
        }
        diagnostics
    }
}
