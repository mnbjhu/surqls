use chumsky::{primitive::just, select, IterParser, Parser};

use crate::{
    ast::{parser::Extra, statement::define::field::DefineField},
    lexer::{keyword::Keyword, token::Token},
    parser::{expr::newline::optional_new_line, type_::type_parser},
    util::span::ParserInput,
};

use super::table::permission_parser;

pub fn define_field_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, DefineField, Extra<'tokens>> + Clone {
    let ident = select! {
        Token::Identifier(ident) => ident,
    }
    .map_with(|i, s| (i.to_string(), s.span()));

    let parents = ident
        .clone()
        .then_ignore(just(Token::Punctuation('.')))
        .repeated()
        .collect::<Vec<_>>();

    let type_ = just(Token::Keyword(Keyword::Type))
        .ignore_then(optional_new_line().ignore_then(type_parser()));

    just(Token::Keyword(Keyword::Field))
        .ignore_then(parents)
        .then(ident.clone())
        .then_ignore(just(Token::Keyword(Keyword::On)))
        .then_ignore(just(Token::Keyword(Keyword::Table)).or_not())
        .then(ident)
        .then(optional_new_line().ignore_then(type_))
        .then(optional_new_line().ignore_then(permission_parser().or_not()))
        .map(
            |((((parents, name), table_name), type_), permission)| DefineField {
                name,
                parents,
                table_name,
                type_,
                permission,
            },
        )
}
