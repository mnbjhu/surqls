use chumsky::{
    primitive::{choice, just},
    recursive::recursive,
    select, Parser,
};
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    core::{
        lexer::Token,
        parser::{parser::Extra, symbol::Symbol},
        span::{ParserInput, Spanned},
    },
    ls::util::range::span_to_range,
};

use super::{
    access::{access_parser, Access},
    array::array_parser,
    literal::{literal_parser, Literal},
    object::{object_parser, ObjectEntry},
    op::{op_parser, BinaryOperator},
    unary::UnaryOperator,
};

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Binary {
        left: Box<Spanned<Expression>>,
        op: Spanned<BinaryOperator>,
        right: Box<Spanned<Expression>>,
    },
    Unary {
        op: Spanned<UnaryOperator>,
        expr: Box<Spanned<Expression>>,
    },
    FunctionCall {
        name: Spanned<String>,
        args: Vec<Spanned<Expression>>,
    },
    Invalid,
    Access {
        expr: Box<Spanned<Expression>>,
        access: Spanned<Box<Access>>,
    },
    Array(Vec<Spanned<Expression>>),
    Object(Vec<Spanned<ObjectEntry>>),
}

pub fn expr_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>> + Clone {
    recursive(|expr| {
        let literal = literal_parser()
            .map(Expression::Literal)
            .map_with(|e, s| (e, s.span()));
        let ident = select! {
            Token::Identifier(s) => s.to_string(),
        }
        .map(Expression::Identifier)
        .map_with(|e, s| (e, s.span()));
        let bracketed = expr.clone().delimited_by(
            just(Token::Punctuation('(')).ignored(),
            just(Token::Punctuation(')')).ignored(),
        );
        let atom = choice((literal, ident, bracketed));
        let access = access_parser(atom, expr.clone());

        op_parser(access.clone())
            .or(access)
            .or(array_parser(expr.clone()))
            .or(object_parser(expr))
    })
    .labelled("expression")
}

impl Symbol for Spanned<Expression> {
    fn get_document_symbol(&self, rope: &ropey::Rope) -> DocumentSymbol {
        let range = span_to_range(&self.1, rope).unwrap();
        match &self.0 {
            Expression::Literal(l) => (l, self.1).get_document_symbol(rope),
            Expression::Identifier(s) => DocumentSymbol {
                name: s.to_string(),
                kind: SymbolKind::VARIABLE,
                tags: None,
                detail: None,
                deprecated: None,
                range: range.clone(),
                selection_range: range,
                children: None,
            },
            Expression::Binary { left, op, right } => DocumentSymbol {
                name: op.0.to_string(),
                kind: SymbolKind::OPERATOR,
                tags: None,
                detail: None,
                deprecated: None,
                range: range.clone(),
                selection_range: range,
                children: Some(vec![
                    left.get_document_symbol(rope),
                    right.get_document_symbol(rope),
                ]),
            },
            Expression::Unary { op, expr } => DocumentSymbol {
                name: op.0.to_string(),
                kind: SymbolKind::OPERATOR,
                tags: None,
                detail: None,
                deprecated: None,
                range: range.clone(),
                selection_range: range,
                children: Some(vec![expr.get_document_symbol(rope)]),
            },
            Expression::FunctionCall { name, args } => DocumentSymbol {
                name: name.0.to_string(),
                kind: SymbolKind::FUNCTION,
                tags: None,
                detail: None,
                deprecated: None,
                range: range.clone(),
                selection_range: range,
                children: Some(
                    args.iter()
                        .map(|arg| arg.get_document_symbol(rope))
                        .collect(),
                ),
            },
            Expression::Invalid => DocumentSymbol {
                name: "INVALID".to_string(),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: range.clone(),
                selection_range: range,
                children: None,
            },
            Expression::Access { expr, access } => DocumentSymbol {
                name: match &access.0.as_ref() {
                    Access::Index(_) => "Index".to_string(),
                    Access::Property(_) => "Property".to_string(),
                },
                kind: SymbolKind::VARIABLE,
                tags: None,
                detail: None,
                deprecated: None,
                range: range.clone(),
                selection_range: range,
                children: Some(vec![
                    expr.get_document_symbol(rope),
                    access.get_document_symbol(rope),
                ]),
            },
            Expression::Array(c) => DocumentSymbol {
                name: "Array".to_string(),
                kind: SymbolKind::ARRAY,
                tags: None,
                detail: None,
                deprecated: None,
                range: range.clone(),
                selection_range: range,
                children: Some(c.iter().map(|e| e.get_document_symbol(rope)).collect()),
            },
            Expression::Object(obj) => DocumentSymbol {
                name: "Object".to_string(),
                kind: SymbolKind::STRUCT,
                tags: None,
                detail: None,
                deprecated: None,
                range: range.clone(),
                selection_range: range,
                children: Some(
                    obj.iter()
                        .map(|(e, span)| DocumentSymbol {
                            name: e.key.0.to_string(),
                            kind: SymbolKind::KEY,
                            tags: None,
                            detail: None,
                            deprecated: None,
                            range: span_to_range(&span, rope).unwrap(),
                            selection_range: span_to_range(&span, rope).unwrap(),
                            children: e.value.clone().map(|s| vec![s.get_document_symbol(rope)]),
                        })
                        .collect(),
                ),
            },
        }
    }
}
