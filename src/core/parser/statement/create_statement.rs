use std::fmt::Display;

use chumsky::{primitive::just, recovery::via_parser, select, Parser};

use crate::core::{
    lexer::{Keyword, Token},
    parser::{
        expr::{
            newline::optional_new_line,
            parser::{expr_parser, Expression},
        },
        parser::Extra,
    },
    span::{ParserInput, Spanned},
};

pub struct CreateStatement {
    pub table: Option<Spanned<String>>,
    pub content: Option<Spanned<Expression>>,
    pub transforms: Vec<Spanned<Transform>>,
}

pub enum Transform {
    Where(Option<Spanned<Expression>>),
    GroupBy(Option<Spanned<Expression>>),
    OrderBy(Option<Spanned<Expression>>),
    Limit(Option<Spanned<Expression>>),
    Skip(Option<Spanned<Expression>>),
    Unknown,
}

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transform::Where(_) => write!(f, "where"),
            Transform::GroupBy(_) => write!(f, "group by"),
            Transform::OrderBy(_) => write!(f, "order by"),
            Transform::Limit(_) => write!(f, "limit"),
            Transform::Skip(_) => write!(f, "skip"),
            Transform::Unknown => write!(f, "unknown"),
        }
    }
}

pub fn create_statement_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, CreateStatement, Extra<'tokens>> + Clone {
    let ident = select! {
        Token::Identifier(s) => s,
    }
    .map_with(|x, s| (x, s.span()));
    let create_part = just(Token::Keyword(Keyword::Create))
        .ignore_then(optional_new_line().ignore_then(ident))
        .map(|x| Some(x))
        .recover_with(via_parser(
            just(Token::Keyword(Keyword::Create)).map(|_| None),
        ));
    let content_part = just(Token::Keyword(Keyword::Content))
        .ignore_then(optional_new_line().ignore_then(expr_parser()))
        .map(|x| Some(x))
        .recover_with(via_parser(
            just(Token::Keyword(Keyword::Content)).map(|_| None),
        ));

    let where_part = just(Token::Keyword(Keyword::Where))
        .ignore_then(optional_new_line().ignore_then(expr_parser()))
        .map(|x| Transform::Where(Some(x)))
        .recover_with(via_parser(
            just(Token::Keyword(Keyword::Where)).map(|_| Transform::Where(None)),
        ));

    create_part
        .clone()
        .then_ignore(optional_new_line())
        .then(content_part)
        .recover_with(via_parser(create_part.map(|x| (x, None))))
        .then_ignore(optional_new_line())
        .then(
            where_part
                .map_with(|part, scope| (part, scope.span()))
                .or_not(),
        )
        .map(|((table, content), transforms)| CreateStatement {
            table,
            content,
            transforms: match transforms {
                Some(where_) => vec![where_],
                None => vec![],
            },
        })
}
