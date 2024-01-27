use chumsky::{primitive::just, Parser};

use crate::{ast::parser::Extra, lexer::token::Token, util::span::ParserInput};

pub fn optional_new_line<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, (), Extra<'tokens>> + Clone + 'tokens {
    just(Token::Newline).or_not().ignored()
}
