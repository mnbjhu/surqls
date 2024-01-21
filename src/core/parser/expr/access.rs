use chumsky::{
    extra,
    prelude::Rich,
    primitive::{choice, just, one_of},
    select, IterParser, Parser,
};
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    core::{
        lexer::Token,
        parser::{parser::Extra, symbol::Symbol},
        span::{ParserInput, Span, Spanned},
    },
    ls::util::range::span_to_range,
};

use super::{newline::optional_new_line, parser::Expression};

#[derive(Clone, Debug)]
pub enum Access {
    Property(String),
    Index(Spanned<Expression>),
}

pub fn access_parser<'tokens, 'src: 'tokens>(
    atom: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
        + Clone
        + 'tokens,
    expr: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
       + Clone
       + 'tokens {
    let index = expr
        .clone()
        .delimited_by(
            just(Token::Punctuation('['))
                .ignored()
                .padded_by(optional_new_line()),
            optional_new_line().then_ignore(just(Token::Punctuation(']'))),
        )
        .map(Access::Index)
        .boxed();

    let field = just(Token::Punctuation('.'))
        .padded_by(optional_new_line())
        .ignore_then(select! {
            Token::Identifier(s) => s,
        })
        .labelled("field")
        .map(|s| Access::Property(s.to_string()))
        .boxed();

    let access = atom
        .clone()
        .foldl_with(choice((index, field)).repeated(), |a, acc, s| match acc {
            Access::Index(index) => (
                Expression::Access {
                    expr: Box::new(a),
                    access: (Box::new(Access::Index(index)), s.span()),
                },
                s.span(),
            ),
            Access::Property(field) => (
                Expression::Access {
                    expr: Box::new(a),
                    access: (Box::new(Access::Property(field)), s.span()),
                },
                s.span(),
            ),
        })
        .boxed();

    access
}

impl Symbol for Spanned<Box<Access>> {
    fn get_document_symbol(&self, rope: &ropey::Rope) -> DocumentSymbol {
        match &self.0.as_ref() {
            Access::Property(s) => DocumentSymbol {
                name: s.clone(),
                kind: SymbolKind::FIELD,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
                detail: None,
                deprecated: None,
                tags: None,
            },
            Access::Index(index) => index.get_document_symbol(rope),
        }
    }
}
