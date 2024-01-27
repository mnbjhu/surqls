use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    ast::{expr::types::Typed, parser::File, statement::statement::Statement},
    declarations::scoped_item::ScopedItems,
};

use super::diagnostic::HasDiagnostic;

impl HasDiagnostic for File {
    fn diagnostics(&self, rope: &Rope, scope: &ScopedItems) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut scope = scope.clone();
        for statement in self {
            if let Statement::Let(let_) = &statement.0 {
                diagnostics.extend(statement.diagnostics(rope, &scope));
                if let Some(name) = &let_.name {
                    if let Some(value) = &let_.value {
                        let ty = value.0.get_type(&scope);
                        scope.variables.insert(name.0.clone(), ty);
                    }
                }
            } else {
                diagnostics.extend(statement.diagnostics(rope, &scope));
            }
        }
        diagnostics
    }
}
