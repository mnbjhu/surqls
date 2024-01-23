use chumsky::{extra, prelude::Rich, select, Parser};
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    core::{
        lexer::Token,
        parser::{parser::Extra, symbol::Symbol},
        span::{ParserInput, Span, Spanned},
    },
    ls::util::range::span_to_range,
};

pub fn literal_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Literal, Extra<'tokens>> + Clone {
    select! {
        Token::Integer(i) => Literal::Int(i),
        Token::Float(f) => Literal::Float(f),
        Token::String(s) => Literal::String(s.to_string()),
        Token::Boolean(b) => Literal::Bool(b),
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

impl Symbol for Spanned<&Literal> {
    fn get_document_symbol(&self, rope: &ropey::Rope) -> DocumentSymbol {
        DocumentSymbol {
            name: format!("{:?}", self.0),
            kind: match self.0 {
                Literal::String(_) => SymbolKind::STRING,
                Literal::Int(_) => SymbolKind::NUMBER,
                Literal::Float(_) => SymbolKind::NUMBER,
                Literal::Bool(_) => SymbolKind::BOOLEAN,
                Literal::Null => SymbolKind::NULL,
            },
            tags: None,
            detail: None,
            deprecated: None,
            range: span_to_range(&self.1, rope).unwrap(),
            selection_range: span_to_range(&self.1, rope).unwrap(),
            children: None,
        }
    }
}
