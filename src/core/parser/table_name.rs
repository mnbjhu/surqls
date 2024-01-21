use chumsky::{extra, prelude::Rich, select, Parser};
use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    core::{
        lexer::Token,
        span::{ParserInput, Span, Spanned},
    },
    ls::util::range::span_to_range,
};

use super::{
    delcarations::{ScopedItems, Type},
    diagnostic::HasDiagnostic,
};

#[derive(Clone, Debug)]
pub enum TableName {
    NotFound(String),
    Found(String, Type),
}

pub fn table_name_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Spanned<TableName>,
    extra::Full<Rich<'tokens, Token, Span>, ScopedItems, ()>,
> + Clone {
    let ident = select! {
        Token::Identifier(ident) => ident.to_string(),
    };
    ident
        .map_with(move |s, scope| {
            let name = s.clone();
            let state: &mut ScopedItems = scope.state();
            match state.table_definitions.get(&s) {
                Some(ty) => TableName::Found(name, ty.clone()),
                None => TableName::NotFound(name),
            }
        })
        .map_with(|s, span| (s, span.span()))
}

impl HasDiagnostic for Spanned<TableName> {
    fn diagnostics(&self, rope: &Rope) -> Vec<Diagnostic> {
        let mut not_found = Diagnostic {
            range: span_to_range(&self.1, rope).unwrap(),
            severity: Some(DiagnosticSeverity::ERROR),
            related_information: None,
            ..Default::default()
        };
        match &self.0 {
            TableName::NotFound(name) => {
                not_found.message = format!("Table '{}' not found", name);
                vec![not_found]
            }
            TableName::Found(_, _) => vec![],
        }
    }
}
