use std::collections::HashMap;

use chumsky::{prelude::Input, Parser};
use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::core::{
    lexer::lexer,
    parser::{
        delcarations::{ScopedItems, Type},
        diagnostic::HasDiagnostic,
        parser::{parser, File},
    },
};

use super::util::range::span_to_range;

pub fn parse_file<'rope>(text: String, rope: &'rope Rope) -> (Option<File>, Vec<Diagnostic>) {
    let token_result = lexer().parse(text.as_str());
    let (tokens, errs) = token_result.into_output_errors();
    let mut diagnostics = vec![];
    for err in errs {
        diagnostics.push(Diagnostic {
            range: span_to_range(err.span(), rope).unwrap(),
            severity: Some(DiagnosticSeverity::ERROR),
            message: err.reason().to_string(),
            ..Default::default()
        });
    }
    if let Some(tokens) = tokens {
        let mut table_definitions = HashMap::new();
        table_definitions.insert("thing".to_string(), Type::Any);
        let mut scoped_table = HashMap::new();
        scoped_table.insert("some".to_string(), Type::String);
        scoped_table.insert("other".to_string(), Type::Int);
        let parser_result = parser().parse_with_state(
            tokens
                .as_slice()
                .spanned((rope.len_chars()..rope.len_chars()).into()),
            &mut ScopedItems {
                table_definitions,
                scoped_table,
            },
        );
        let (ast, errs) = parser_result.into_output_errors();
        for err in errs {
            diagnostics.push(Diagnostic {
                range: span_to_range(err.span(), rope).unwrap(),
                severity: Some(DiagnosticSeverity::ERROR),
                message: err.reason().to_string(),
                ..Default::default()
            });
        }
        if let Some(ast) = ast {
            diagnostics.extend(ast.diagnostics(rope));
            return (Some(ast), diagnostics);
        } else {
            return (None, diagnostics);
        }
    }
    (None, diagnostics)
}
