use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    ast::statement::transform::Transform,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::{HasDiagnostic, HasDiagnosticsForType},
};

impl HasDiagnostic for Transform {
    fn diagnostics(&self, rope: &Rope, scope: &ScopedItems) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        match &self {
            Transform::Where(Some(limit)) => {
                diagnostics.extend(limit.diagnostics_for_type(rope, &Type::Bool, scope));
            }
            Transform::Limit(Some(limit)) => {
                diagnostics.extend(limit.diagnostics_for_type(rope, &Type::Int, scope));
            }
            Transform::Skip(Some(skip)) => {
                diagnostics.extend(skip.diagnostics_for_type(rope, &Type::Int, scope));
            }
            _ => {}
        }
        diagnostics
    }
}
