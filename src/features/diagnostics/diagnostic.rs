use chumsky::{prelude::Input, Parser};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    ast::parser::File, declarations::scoped_item::ScopedItems, lexer::lexer::lexer,
    parser::parser::parser, util::range::span_to_range,
};
use ropey::Rope;

use crate::declarations::type_::Type;

pub trait HasDiagnosticsForType {
    fn diagnostics_for_type(
        &self,
        rope: &Rope,
        type_: &Type,
        scope: &ScopedItems,
    ) -> Vec<Diagnostic>;
}
pub trait HasDiagnostic {
    fn diagnostics(&self, rope: &Rope, scope: &ScopedItems) -> Vec<Diagnostic>;
}

pub fn parse_file<'rope>(
    text: String,
    rope: &'rope Rope,
    mut scope: &mut ScopedItems,
) -> (Option<File>, Vec<Diagnostic>) {
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
