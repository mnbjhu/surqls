pub mod field;
pub mod table;

use chumsky::{
    primitive::{choice, just},
    Parser,
};

use crate::{
    ast::{
        parser::Extra,
        statement::{define::DefineStatement, statement::Statement},
    },
    lexer::{keyword::Keyword, token::Token},
    parser::expr::newline::optional_new_line,
    util::span::{ParserInput, Spanned},
};

use self::{field::define_field_parser, table::define_table_parser};

pub fn define_statement_parser<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Statement>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<DefineStatement>, Extra<'tokens>>
       + Clone
       + 'tokens {
    let kind = choice((
        define_table_parser().map_with(|t, s| DefineStatement::Table((t, s.span()))),
        define_field_parser().map_with(|f, s| DefineStatement::Field((f, s.span()))),
    ));

    just(Token::Keyword(Keyword::Define))
        .ignore_then(optional_new_line().ignore_then(kind))
        .map_with(|s, scope| (s, scope.span()))
}
