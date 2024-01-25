use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    ast::statement::statement::Statement, declarations::scoped_item::ScopedItems,
    features::diagnostics::diagnostic::HasDiagnostic, util::span::Spanned,
};

impl HasDiagnostic for Spanned<Statement> {
    fn diagnostics(&self, rope: &Rope, scope: &mut ScopedItems) -> Vec<Diagnostic> {
        match &self.0 {
            Statement::Create(create) => (create, self.1).diagnostics(rope, scope),
            Statement::Update(update) => (update, self.1).diagnostics(rope, scope),
            Statement::Return(expr) => expr.diagnostics(rope, scope),
            Statement::Define(_) => vec![],
        }
    }
}
