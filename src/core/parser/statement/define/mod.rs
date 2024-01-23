pub mod field;
pub mod table;

use crate::core::{
    lexer::{Keyword, Token},
    parser::{expr::newline::optional_new_line, parser::Extra},
    span::{ParserInput, Spanned},
};
use chumsky::{
    primitive::{choice, just},
    Parser,
};

use self::{
    field::{define_field_parser, DefineField},
    table::{define_table_parser, DefineTable},
};

pub fn define_statement_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<DefineStatement>, Extra<'tokens>> + Clone
{
    let kind = choice((
        define_table_parser().map_with(|t, s| DefineStatement::Table((t, s.span()))),
        define_field_parser().map_with(|f, s| DefineStatement::Field((f, s.span()))),
    ));

    just(Token::Keyword(Keyword::Define))
        .ignore_then(optional_new_line().ignore_then(kind))
        .map_with(|s, scope| (s, scope.span()))
}

pub enum DefineStatement {
    Table(Spanned<DefineTable>),
    Field(Spanned<DefineField>),
}
