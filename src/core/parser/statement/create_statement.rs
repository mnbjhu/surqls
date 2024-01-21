use chumsky::{
    prelude::Rich,
    primitive::{any, choice, end, just, none_of, one_of},
    recovery::{skip_then_retry_until, skip_until, via_parser},
    IterParser, Parser,
};
use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DocumentSymbol, SymbolKind};

use crate::{
    core::{
        lexer::{Keyword, Token},
        parser::{
            diagnostic::HasDiagnostic,
            expr::{
                newline::optional_new_line,
                parser::{expr_parser, Expression},
            },
            parser::Extra,
            symbol::Symbol,
            table_name::{table_name_parser, TableName},
        },
        span::{ParserInput, Spanned},
    },
    ls::util::range::span_to_range,
};

pub struct CreateStatement {
    pub table: Option<Spanned<TableName>>,
    pub content: Option<Spanned<Expression>>,
    pub transforms: Vec<Spanned<Transform>>,
}

pub struct Content(pub Option<Spanned<Expression>>);

pub enum Transform {
    Where(Option<Spanned<Expression>>),
    GroupBy(Option<Spanned<Expression>>),
    OrderBy(Option<Spanned<Expression>>),
    Limit(Option<Spanned<Expression>>),
    Skip(Option<Spanned<Expression>>),
    Unknown,
}

pub fn transform_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Transform, Extra<'tokens>> + Clone {
    let where_ = just(Token::Keyword(Keyword::Where))
        .ignore_then(optional_new_line().ignore_then(expr_parser()))
        .map(|e| Transform::Where(Some(e)));
    // .recover_with(via_parser(
    //     just(Token::Keyword(Keyword::Where))
    //         .map(|_| Transform::Where(None))
    //         .map_err_with_state(|_, span, _| {
    //             Rich::custom(span, "Expected expression after 'WHERE'".to_string())
    //         }),
    // ));
    let group_by = just(Token::Keyword(Keyword::Group))
        .ignore_then(just(Token::Keyword(Keyword::By)))
        .ignore_then(optional_new_line().ignore_then(expr_parser()))
        .map(|e| Transform::Where(Some(e)));
    // .recover_with(via_parser(
    //     just(Token::Keyword(Keyword::Group))
    //         .ignore_then(just(Token::Keyword(Keyword::By)))
    //         .map(|_| Transform::Where(None))
    //         .map_err_with_state(|_, span, _| {
    //             Rich::custom(span, "Expected expression after 'GROUP BY'".to_string())
    //         }),
    // ));
    let order_by = just(Token::Keyword(Keyword::Order))
        .ignore_then(just(Token::Keyword(Keyword::By)))
        .ignore_then(optional_new_line().ignore_then(expr_parser()))
        .map(|e| Transform::Where(Some(e)));
    // .recover_with(via_parser(
    //     just(Token::Keyword(Keyword::Order))
    //         .ignore_then(just(Token::Keyword(Keyword::By)))
    //         .map(|_| Transform::Where(None))
    //         .map_err_with_state(|_, span, _| {
    //             Rich::custom(span, "Expected expression after 'ORDER BY'".to_string())
    //         }),
    // ));
    let limit = just(Token::Keyword(Keyword::Limit))
        .ignore_then(optional_new_line().ignore_then(expr_parser()))
        .map(|e| Transform::Where(Some(e)));
    // .recover_with(via_parser(
    //     just(Token::Keyword(Keyword::Limit))
    //         .map(|_| Transform::Where(None))
    //         .map_err_with_state(|_, span, _| {
    //             Rich::custom(span, "Expected expression after 'LIMIT'".to_string())
    //         }),
    // ));
    let skip = just(Token::Keyword(Keyword::Skip))
        .ignore_then(optional_new_line().ignore_then(expr_parser()))
        .map(|e| Transform::Where(Some(e)));
    // .recover_with(via_parser(
    //     just(Token::Keyword(Keyword::Skip))
    //         .map(|_| Transform::Where(None))
    //         .map_err_with_state(|_, span, _| {
    //             Rich::custom(span, "Expected expression after 'SKIP'".to_string())
    //         }),
    // ));
    choice((where_, group_by, order_by, limit, skip)).recover_with(skip_then_retry_until(
        any().ignored(),
        choice((
            end(),
            one_of(vec![
                Token::Keyword(Keyword::Where),
                Token::Keyword(Keyword::Group),
                Token::Keyword(Keyword::By),
                Token::Keyword(Keyword::Order),
                Token::Keyword(Keyword::Limit),
                Token::Keyword(Keyword::Skip),
                Token::Punctuation(';'),
            ])
            .ignored(),
        )),
    ))
    // .recover_with(via_parser(invalid_transform_parser()))
}

fn invalid_transform_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Transform, Extra<'tokens>> + Clone {
    let others = none_of(vec![
        Token::Keyword(Keyword::Where),
        Token::Keyword(Keyword::Group),
        Token::Keyword(Keyword::By),
        Token::Keyword(Keyword::Order),
        Token::Keyword(Keyword::Limit),
        Token::Keyword(Keyword::Skip),
        Token::Punctuation(';'),
        Token::Newline,
    ]);
    others
        .clone()
        .then(others.repeated())
        .then_ignore(end().or_not())
        .map(|_| Transform::Unknown)
}

pub fn create_statement_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, CreateStatement, Extra<'tokens>> + Clone {
    let create_part = just(Token::Keyword(Keyword::Create))
        .ignore_then(optional_new_line().ignore_then(table_name_parser()))
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
        .then(
            optional_new_line()
                .ignore_then(where_part)
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
impl Symbol for Spanned<&CreateStatement> {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol {
        let mut children = Vec::new();
        if let Some(content) = &self.0.content {
            children.push(content.get_document_symbol(rope));
        }
        for transform in &self.0.transforms {
            children.push(transform.get_document_symbol(rope));
        }
        DocumentSymbol {
            name: format!("CREATE {:?}", self.0.table),
            kind: SymbolKind::STRUCT,
            tags: None,
            detail: None,
            deprecated: None,
            range: span_to_range(&self.1, rope).unwrap(),
            selection_range: span_to_range(&self.1, rope).unwrap(),
            children: Some(children),
        }
    }
}

impl Symbol for Spanned<Content> {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol {
        match &self.0 {
            Content(Some(expr)) => expr.get_document_symbol(rope),
            Content(None) => DocumentSymbol {
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

impl Symbol for Spanned<Transform> {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol {
        match &self.0 {
            Transform::Where(Some(expr)) => expr.get_document_symbol(rope),
            Transform::Where(None) => DocumentSymbol {
                name: "Where".to_string(),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
            Transform::GroupBy(Some(expr)) => expr.get_document_symbol(rope),
            Transform::GroupBy(None) => DocumentSymbol {
                name: "Group By".to_string(),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
            Transform::OrderBy(Some(expr)) => expr.get_document_symbol(rope),
            Transform::OrderBy(None) => DocumentSymbol {
                name: "Order By".to_string(),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
            Transform::Limit(Some(expr)) => expr.get_document_symbol(rope),
            Transform::Limit(None) => DocumentSymbol {
                name: "Limit".to_string(),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
            Transform::Skip(Some(expr)) => expr.get_document_symbol(rope),
            Transform::Skip(None) => DocumentSymbol {
                name: "Skip".to_string(),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
            Transform::Unknown => DocumentSymbol {
                name: "Unknown Transform".to_string(),
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

impl HasDiagnostic for Spanned<&CreateStatement> {
    fn diagnostics(&self, rope: &Rope) -> Vec<Diagnostic> {
        match &self.0.table {
            Some(table) => {
                return table.diagnostics(rope);
            }
            None => vec![],
        }
    }
}
