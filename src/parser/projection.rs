use chumsky::{primitive::just, select, Parser};

use crate::{
    ast::{parser::Extra, projection::Projection},
    lexer::{keyword::Keyword, token::Token},
    util::span::{ParserInput, Spanned},
};

use super::expr::{newline::optional_new_line, parser::expr_parser};

pub fn projection_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Projection>, Extra<'tokens>> + Clone {
    let identifier_parser = select! {
        Token::Identifier(s) => s,
    }
    .map_with(|s, span| (s.to_string(), span.span()));
    let alias = just(Token::Keyword(Keyword::As))
        .padded_by(optional_new_line())
        .ignore_then(identifier_parser);
    let projection = expr_parser()
        .then(alias.or_not())
        .map(|(expr, alias)| Projection { expr, alias })
        .map_with(|p, span| (p, span.span()));
    projection
}
