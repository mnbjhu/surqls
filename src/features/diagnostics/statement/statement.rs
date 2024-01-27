use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    ast::statement::statement::Statement, declarations::scoped_item::ScopedItems,
    features::diagnostics::diagnostic::HasDiagnostic, util::span::Spanned,
};

impl HasDiagnostic for Spanned<Statement> {
    fn diagnostics(&self, rope: &Rope, scope: &ScopedItems) -> Vec<Diagnostic> {
        match &self.0 {
            Statement::Create(create) => (create, self.1).diagnostics(rope, scope),
            Statement::Update(update) => (update, self.1).diagnostics(rope, scope),
            Statement::Delete(delete) => (delete, self.1).diagnostics(rope, scope),
            Statement::Select(select) => (select, self.1).diagnostics(rope, scope),
            Statement::Define(define) => (&define.0, self.1).diagnostics(rope, scope),
            Statement::Return(expr) => expr.diagnostics(rope, scope),
            Statement::Let(let_) => (let_, self.1).diagnostics(rope, scope),
            Statement::Invalid => vec![],
        }
    }
}
