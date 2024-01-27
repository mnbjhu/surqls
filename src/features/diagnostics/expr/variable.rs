use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    declarations::{scoped_item::ScopedItems, type_::Type},
    util::{range::span_to_range, span::Spanned},
};

pub fn get_variable_diagnostics_for_type(
    variable: &Spanned<&String>,
    rope: &Rope,
    type_: &Type,
    scope: &ScopedItems,
) -> Vec<Diagnostic> {
    match scope.variables.get(variable.0) {
        Some(_) => {
            vec![]
        }
        None => vec![Diagnostic {
            range: span_to_range(&variable.1, rope).unwrap(),
            severity: Some(DiagnosticSeverity::ERROR),
            message: format!("Unknown variable '{}'", variable.0),
            ..Default::default()
        }],
    }
}
