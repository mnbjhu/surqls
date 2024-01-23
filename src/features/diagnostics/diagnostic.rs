use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::core::parser::delcarations::{ScopedItems, Type};

pub trait HasDiagnosticsForType {
    fn diagnostics_for_type(
        &self,
        rope: &Rope,
        type_: &Type,
        scope: &mut ScopedItems,
    ) -> Vec<Diagnostic>;
}
pub trait HasDiagnostic {
    fn diagnostics(&self, rope: &Rope, scope: &mut ScopedItems) -> Vec<Diagnostic>;
}
