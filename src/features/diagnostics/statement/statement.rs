use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    ast::{expr::types::Typed, statement::statement::Statement},
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::HasDiagnostic,
    util::span::Spanned,
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

impl Typed for Statement {
    fn get_type(&self, scope: &ScopedItems) -> Type {
        match self {
            Statement::Create(create) => create.get_type(scope),
            Statement::Update(update) => update.get_type(scope),
            Statement::Delete(delete) => delete.get_type(scope),
            Statement::Select(select) => select.get_type(scope),
            _ => Type::Null,
        }
    }
}
