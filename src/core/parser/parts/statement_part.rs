use chumsky::{
    extra,
    prelude::Rich,
    primitive::{choice, just},
    select, IterParser, Parser,
};
use ropey::Rope;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    core::{
        lexer::{Keyword, Token},
        parser::{
            expr::{
                newline::optional_new_line,
                parser::{expr_parser, Expression},
            },
            parser::Extra,
            projection::{projection_parser, Projection},
            symbol::Symbol,
        },
        span::{ParserInput, Span, Spanned},
    },
    ls::util::range::span_to_range,
};

pub enum StatementPart {
    WhereClause(Option<Spanned<Expression>>),
    OrderByClause(Vec<Spanned<Projection>>),
    LimitClause(Option<Spanned<Expression>>),
    SkipClause(Option<Spanned<Expression>>),
    From(Option<Spanned<String>>),
    Content(Option<Spanned<Expression>>),
}

pub fn statement_part_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<StatementPart>, Extra<'tokens>> + Clone
{
    let where_clause = just(Token::Keyword(Keyword::Where))
        .ignore_then(optional_new_line().ignore_then(expr_parser()).or_not())
        .map(StatementPart::WhereClause);

    let order_by_clause = just(Token::Keyword(Keyword::Order))
        .then_ignore(just(Token::Keyword(Keyword::By)))
        .ignore_then(
            optional_new_line().ignore_then(
                projection_parser()
                    .separated_by(just(Token::Punctuation(',')))
                    .collect::<Vec<_>>(),
            ),
        )
        .map(StatementPart::OrderByClause);

    let limit_clause = just(Token::Keyword(Keyword::Limit))
        .ignore_then(optional_new_line().ignore_then(expr_parser()).or_not())
        .map(StatementPart::LimitClause);

    let skip_clause = just(Token::Keyword(Keyword::Skip))
        .ignore_then(optional_new_line().ignore_then(expr_parser()).or_not())
        .map(StatementPart::SkipClause);

    let from_clause = just(Token::Keyword(Keyword::From))
        .ignore_then(
            optional_new_line().ignore_then(
                select! {
                    Token::Identifier(s) => s.to_string(),
                }
                .map_with(|s, span| (s, span.span()))
                .or_not(),
            ),
        )
        .map(StatementPart::From);

    let content = just(Token::Keyword(Keyword::Content))
        .ignore_then(optional_new_line().ignore_then(expr_parser()))
        .map(|e| StatementPart::Content(Some(e)));

    choice((
        where_clause,
        order_by_clause,
        limit_clause,
        skip_clause,
        from_clause,
        content,
    ))
    .map_with(|s, span| (s, span.span()))
}

impl Symbol for Spanned<StatementPart> {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol {
        match &self.0 {
            StatementPart::WhereClause(_) => DocumentSymbol {
                name: "Where".to_string(),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
            StatementPart::OrderByClause(_) => DocumentSymbol {
                name: "Order By".to_string(),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
            StatementPart::LimitClause(_) => DocumentSymbol {
                name: "Limit".to_string(),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
            StatementPart::SkipClause(_) => DocumentSymbol {
                name: "Skip".to_string(),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
            StatementPart::From(_) => DocumentSymbol {
                name: "From".to_string(),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
            StatementPart::Content(_) => DocumentSymbol {
                name: "Content".to_string(),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
        }
    }
}
