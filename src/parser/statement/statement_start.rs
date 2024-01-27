use chumsky::{
    primitive::{choice, end, just},
    Parser,
};

use crate::{
    ast::parser::Extra,
    lexer::{keyword::Keyword, token::Token},
    util::span::ParserInput,
};

pub fn statement_start_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, (), Extra<'tokens>> + Clone + 'tokens {
    choice((
        end().ignored(),
        just(Token::Keyword(Keyword::Create)).ignored(),
        just(Token::Keyword(Keyword::Update)).ignored(),
        just(Token::Keyword(Keyword::Delete)).ignored(),
        just(Token::Keyword(Keyword::Select)).ignored(),
        just(Token::Keyword(Keyword::Define)).ignored(),
        just(Token::Keyword(Keyword::Let)).ignored(),
    ))
}
