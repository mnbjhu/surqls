use chumsky::{primitive::none_of, Parser};

use crate::{
    ast::{parser::Extra, statement::statement::Statement},
    lexer::token::Token,
    util::span::ParserInput,
};

pub fn invalid_statement_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, Extra<'tokens>> + Clone {
    let pattern = none_of(Token::Punctuation(';'));
    pattern
        .clone()
        .then(pattern.repeated())
        .map(|_| Statement::Invalid)
}
