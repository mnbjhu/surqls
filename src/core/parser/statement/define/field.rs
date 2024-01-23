use crate::core::{
    lexer::{Keyword, Token},
    parser::{
        expr::newline::optional_new_line,
        parser::Extra,
        type_::{type_parser, Type},
    },
    span::{ParserInput, Spanned},
};
use chumsky::{primitive::just, select, IterParser, Parser};

use super::table::{permission_parser, Permission};

pub struct DefineField {
    pub name: Spanned<String>,
    pub parents: Vec<Spanned<String>>,
    pub table_name: Spanned<String>,
    pub type_: Spanned<Type>,
    pub permission: Option<Spanned<Permission>>,
}

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
        .ignore_then(ident.clone())
        .then_ignore(just(Token::Keyword(Keyword::On)))
        .then_ignore(just(Token::Keyword(Keyword::Table)).or_not())
        .then(parents)
        .then(ident)
        .then(optional_new_line().ignore_then(type_))
        .then(optional_new_line().ignore_then(permission_parser().or_not()))
        .map(
            |((((name, parents), table_name), type_), permission)| DefineField {
                name,
                parents,
                table_name,
                type_,
                permission,
            },
        )
}
