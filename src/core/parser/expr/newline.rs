use chumsky::{extra, prelude::Rich, primitive::just, Parser};

use crate::core::{
    lexer::Token,
    parser::parser::Extra,
    span::{ParserInput, Span},
};

pub fn optional_new_line<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, (), Extra<'tokens>> + Clone {
    just(Token::Newline).or_not().ignored()
}
