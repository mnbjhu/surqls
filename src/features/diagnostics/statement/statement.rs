use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    core::{
        parser::{delcarations::ScopedItems, statement::statement::Statement},
        span::Spanned,
    },
    features::diagnostics::diagnostic::HasDiagnostic,
    ls::util::range::span_to_range,
};

impl HasDiagnostic for Spanned<Statement> {
    fn diagnostics(&self, rope: &Rope, scope: &mut ScopedItems) -> Vec<Diagnostic> {
        let text = rope.slice(self.1.start..self.1.end);
        match &self.0 {
            Statement::Create(create) => (create, self.1).diagnostics(rope, scope),
            Statement::Return(expr) => expr.diagnostics(rope, scope),
            Statement::Invalid => vec![Diagnostic {
                range: span_to_range(&self.1, rope).unwrap(),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!("Invalid Statement: '{}'", text),
                ..Default::default()
            }],
            Statement::Define(_) => vec![],
        }
    }
}
