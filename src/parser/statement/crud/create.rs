use chumsky::{primitive::just, recovery::via_parser, select, IterParser, Parser};

use crate::{
    ast::{parser::Extra, statement::crud::create::CreateStatement},
    lexer::{keyword::Keyword, token::Token},
    parser::{expr::newline::optional_new_line, statement::transform::transform_parser},
    util::span::ParserInput,
};

use super::content::content_parser;

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

    create_part
        .clone()
        .then_ignore(optional_new_line())
        .then(content_parser())
        .recover_with(via_parser(create_part.map(|x| (x, None))))
        .then_ignore(optional_new_line())
        .then(
            transform_parser()
                .map_with(|part, scope| (part, scope.span()))
                .separated_by(optional_new_line())
                .collect::<Vec<_>>(),
        )
        .map(|((table, content), transforms)| CreateStatement {
            table,
            content,
            transforms,
        })
}
