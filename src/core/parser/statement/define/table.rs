use crate::core::{
    lexer::{Keyword, Token},
    parser::{expr::newline::optional_new_line, parser::Extra},
    span::{ParserInput, Spanned},
};
use chumsky::{
    primitive::{choice, just},
    recovery::via_parser,
    select, Parser,
};

pub enum Permission {
    Full,
    None,
}

pub struct DefineTable {
    pub name: Spanned<String>,
    pub permission: Option<Spanned<Permission>>,
}

pub fn define_table_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, DefineTable, Extra<'tokens>> + Clone {
    let ident = select! {
        Token::Identifier(ident) => ident,
    }
    .map_with(|i, s| (i.to_string(), s.span()));

    just(Token::Keyword(Keyword::Table))
        .ignore_then(optional_new_line().ignore_then(ident))
        .then(optional_new_line().ignore_then(permission_parser().or_not()))
        .map(|(name, permission)| DefineTable { name, permission })
}

pub fn permission_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Permission>, Extra<'tokens>> + Clone {
    // let recovered = just(Token::Keyword(Keyword::Permissions)).map(|_| Permission::None);
    just(Token::Keyword(Keyword::Permissions))
        .ignore_then(optional_new_line())
        .ignore_then(choice((
            just(Token::Keyword(Keyword::Full)).map(|_| Permission::Full),
            just(Token::Keyword(Keyword::None)).map(|_| Permission::None),
        )))
        .map_with(|p, s| (p, s.span()))
}
