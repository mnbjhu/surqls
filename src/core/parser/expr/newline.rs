use chumsky::{extra, prelude::Rich, primitive::just, Parser};

use crate::core::{
    lexer::Token,
    span::{ParserInput, Span},
};

pub fn optional_new_line<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, (), extra::Err<Rich<'tokens, Token<'src>, Span>>>
       + Clone {
    just(Token::Newline).or_not().ignored()
}
