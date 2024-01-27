use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Range};

use crate::{
    ast::expr::{op::BinaryOperator, parser::Expression, types::Typed},
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::HasDiagnosticsForType,
    util::{range::span_to_range, span::Spanned},
};

pub fn get_bin_op_diagnostics_for_type(
    left: &Spanned<Expression>,
    right: &Spanned<Expression>,
    op: &Spanned<BinaryOperator>,
    rope: &Rope,
    type_: &Type,
    scope: &ScopedItems,
) -> Vec<Diagnostic> {
    let mut diagnostics = vec![];
    diagnostics.extend(left.diagnostics_for_type(rope, &Type::Any, scope));
    diagnostics.extend(right.diagnostics_for_type(rope, &Type::Any, scope));
    let left_type = left.0.get_type(scope);
    let right_type = right.0.get_type(scope);
    let left_range = span_to_range(&left.1, rope).unwrap();
    let right_range = span_to_range(&right.1, rope).unwrap();
    match &op.0 {
        BinaryOperator::Add
        | BinaryOperator::Subtract
        | BinaryOperator::Multiply
        | BinaryOperator::Divide
        | BinaryOperator::Modulo => {
            if !left_type.is_numeric() && left_type != Type::Error {
                diagnostics.push(Diagnostic {
                    range: left_range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!("Expected numeric type, found {}", left_type.to_string()),
                    ..Default::default()
                });
            }
            if !right_type.is_numeric() && right_type != Type::Error {
                diagnostics.push(Diagnostic {
                    range: right_range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!("Expected numeric type, found {}", right_type.to_string()),
                    ..Default::default()
                });
            }
        }
        BinaryOperator::Equals
        | BinaryOperator::NotEquals
        | BinaryOperator::LessThan
        | BinaryOperator::LessThanOrEqual
        | BinaryOperator::GreaterThan => {
            if left_type == Type::Error || right_type == Type::Error {
                return diagnostics;
            }

            if !left_type.is_assignable_to(&right_type) && !left_type.is_assignable_to(&right_type)
            {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: left_range.start,
                        end: right_range.end,
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!(
                        "Cannot compare {} and {}",
                        left_type.to_string(),
                        right_type.to_string()
                    ),
                    ..Default::default()
                });
            }
        }
        _ => {}
    }
    diagnostics
}

impl Type {
    fn is_numeric(&self) -> bool {
        match self {
            Type::Int | Type::Float => true,
            _ => false,
        }
    }
}
