use chumsky::{prelude::Input, Parser};
use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::core::{
    lexer::lexer,
    parser::parser::{parser, File},
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
            message: err.found().unwrap().to_string(),
            ..Default::default()
        });
    }
    if let Some(tokens) = tokens {
        let parser_result = parser().parse(
            tokens
                .as_slice()
                .spanned((rope.len_chars()..rope.len_chars()).into()),
        );
        let (ast, errs) = parser_result.into_output_errors();
        for err in errs {
            diagnostics.push(Diagnostic {
                range: span_to_range(err.span(), rope).unwrap(),
                severity: Some(DiagnosticSeverity::ERROR),
                message: err.found().unwrap().to_string(),
                ..Default::default()
            });
        }
        return (ast, diagnostics);
    }
    (None, diagnostics)
}
