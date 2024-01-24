use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    core::{parser::delcarations::ScopedItems, span::Span},
    ls::util::range::span_to_range,
};

pub fn diagnostics_for_table_name(
    name: &str,
    span: &Span,
    rope: &Rope,
    scope: &mut ScopedItems,
) -> Vec<Diagnostic> {
    match scope.table_definitions.get(name) {
        Some(_) => {
            vec![]
        }
        None => vec![Diagnostic {
            message: format!("Table '{}' not found", name),
            range: span_to_range(span, rope).unwrap(),
            severity: Some(DiagnosticSeverity::ERROR),
            related_information: None,
            ..Default::default()
        }],
    }
}
