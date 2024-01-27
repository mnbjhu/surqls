use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    ast::statement::let_::LetStatement,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::{HasDiagnostic, HasDiagnosticsForType},
    util::span::Spanned,
};

impl HasDiagnostic for Spanned<&LetStatement> {
    fn diagnostics(&self, rope: &Rope, scope: &ScopedItems) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if let Some(expr) = &self.0.value {
            diagnostics.extend(expr.diagnostics_for_type(rope, &Type::Any, scope));
        }
        diagnostics
    }
}
