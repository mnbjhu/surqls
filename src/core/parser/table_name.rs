use chumsky::{extra, prelude::Rich, select, Parser};

use crate::core::{
    lexer::Token,
    span::{ParserInput, Span, Spanned},
};

use super::delcarations::{ScopedItems, Type};

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
