use chumsky::{prelude::Input, Parser};
use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    core::{
        lexer::lexer,
        parser::{
            delcarations::ScopedItems,
            parser::{parser, File},
        },
    },
    features::diagnostics::diagnostic::HasDiagnostic,
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
        let mut scope = ScopedItems::default();
        let parser_result = parser().parse_with_state(
            tokens
                .as_slice()
                .spanned((rope.len_chars()..rope.len_chars()).into()),
            &mut scope,
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
            diagnostics.extend(ast.diagnostics(rope, &mut scope));
            return (Some(ast), diagnostics);
        } else {
            return (None, diagnostics);
        }
    }
    (None, diagnostics)
}
