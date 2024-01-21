use std::fmt::Display;

use chumsky::{select, Parser};

use crate::core::{
    lexer::Token,
    parser::{
        delcarations::{ScopedItems, Type},
        parser::Extra,
    },
    span::{ParserInput, Spanned},
};

#[derive(Debug, Clone)]
pub enum Field {
    NotFound(String),
    Found(String, Type),
}

pub fn field_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Field>, Extra<'tokens>> + Clone {
    let field = select! {
        Token::Identifier(s) => s,
    }
    .map_with(|e, s| {
        let state: &mut ScopedItems = s.state();
        match state.scoped_table.get(&e) {
            Some(t) => (Field::Found(e.to_string(), t.clone()), s.span()),
            None => (Field::NotFound(e.to_string()), s.span()),
        }
    });
    field
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::NotFound(s) => write!(f, "{}: NOT_FOUND", s),
            Field::Found(s, t) => write!(f, "{}: {}", s, t),
        }
    }
}
