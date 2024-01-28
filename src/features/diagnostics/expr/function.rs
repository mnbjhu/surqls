use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    ast::expr::parser::Expression,
    declarations::{functions::GenericType, scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::HasDiagnosticsForType,
    util::{
        range::span_to_range,
        span::{Span, Spanned},
    },
};
pub fn get_function_call_diagnostics_for_type(
    rope: &Rope,
    type_: &Type,
    scope: &ScopedItems,
    span: &Span,
    name: &str,
    args: &Vec<Spanned<Expression>>,
) -> Vec<Diagnostic> {
    if let Some(found) = scope.functions.get(name) {
        let mut diagnostics = vec![];
        for (i, arg) in args.iter().enumerate() {
            let function_arg = &found.args.get(i);
            if let Some(function_arg) = function_arg {
                match function_arg.1 {
                    GenericType::Named(ref ty) => {
                        diagnostics.extend(arg.diagnostics_for_type(rope, ty, scope));
                    }
                    _ => {}
                }
            } else {
                diagnostics.push(Diagnostic {
                    range: span_to_range(&arg.1, rope).unwrap(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!("Function '{}' does not have {} arguments", name, i + 1),
                    ..Default::default()
                });
            }
        }
        if found.args.len() > args.len() {
            let missing = &found.args[args.len()..];
            let message = if missing.len() == 1 {
                format!(
                    "Function '{}' is missing 1 argument: '{}'",
                    name, missing[0].0
                )
            } else {
                format!(
                    "Function '{}' is missing {} arguments: {}",
                    name,
                    missing.len(),
                    missing
                        .iter()
                        .map(|x| format!("'{}'", x.0))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };

            diagnostics.push(Diagnostic {
                range: span_to_range(span, rope).unwrap(),
                severity: Some(DiagnosticSeverity::ERROR),
                message,
                ..Default::default()
            });
        }
        diagnostics
    } else {
        vec![Diagnostic {
            range: span_to_range(span, rope).unwrap(),
            severity: Some(DiagnosticSeverity::ERROR),
            message: format!("Function '{}' does not exist", name),
            ..Default::default()
        }]
    }
}
