use chumsky::{
    primitive::{choice, just},
    recursive::recursive,
    select, Parser,
};

use crate::{
    ast::{expr::parser::Expression, parser::Extra, statement::statement::Statement},
    lexer::token::Token,
    util::span::{ParserInput, Spanned},
};

use super::{
    access::access_parser, array::array_parser, code_block::code_block_parser,
    function::function_parser, literal::literal_parser, newline::optional_new_line,
    object::object_parser, op::op_parser,
};

pub fn expr_parser<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Statement>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
       + Clone
       + 'tokens {
    recursive(|expr| {
        let literal = literal_parser()
            .map(Expression::Literal)
            .map_with(|e, s| (e, s.span()));
        let ident = select! {
            Token::Identifier(s) => s.to_string(),
        }
        .map(Expression::Identifier)
        .map_with(|e, s| (e, s.span()));

        let variable = select! {
            Token::Variable(s) => s.to_string(),
        }
        .map(Expression::Variable)
        .map_with(|e, s| (e, s.span()));

        let bracketed = expr.clone().delimited_by(
            just(Token::Punctuation('(')).ignored(),
            just(Token::Punctuation(')')).ignored(),
        );

        let bracketed_statement = stmt
            .clone()
            .delimited_by(
                just(Token::Punctuation('(')).then(optional_new_line()),
                optional_new_line().then(just(Token::Punctuation(')'))),
            )
            .map(|e| Expression::Inline(Box::new(e)))
            .map_with(|e, s| (e, s.span()));

        let atom = choice((
            literal,
            function_parser(expr.clone()),
            variable,
            ident,
            bracketed_statement,
            bracketed,
        ));
        let access = access_parser(atom, expr.clone());

        op_parser(access.clone())
            .or(access)
            .or(array_parser(expr.clone()))
            .or(object_parser(expr))
            .or(code_block_parser(stmt))
    })
    .labelled("expression")
}
