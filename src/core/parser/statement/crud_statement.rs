use chumsky::{extra, prelude::Rich, IterParser, Parser};
use ropey::Rope;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    core::{
        lexer::Token,
        parser::{
            parts::{
                start::{statement_start_parser, StatementStart},
                statement_part::{statement_part_parser, StatementPart},
            },
            symbol::Symbol,
        },
        span::{ParserInput, Span, Spanned},
    },
    ls::util::range::span_to_range,
};

pub struct CrudStatement {
    pub start: Spanned<StatementStart>,
    pub parts: Vec<Spanned<StatementPart>>,
}

pub fn crud_statement_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    CrudStatement,
    extra::Err<Rich<'tokens, Token<'src>, Span>>,
> + Clone {
    statement_start_parser()
        .then(statement_part_parser().repeated().collect::<Vec<_>>())
        .map(|(start, parts)| CrudStatement { start, parts })
}

impl Symbol for Spanned<&CrudStatement> {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol {
        let mut children = Vec::new();
        for part in &self.0.parts {
            children.push(part.get_document_symbol(rope));
        }
        DocumentSymbol {
            name: self.0.start.0.to_string(),
            kind: SymbolKind::EVENT,
            tags: None,
            detail: None,
            deprecated: None,
            range: span_to_range(&self.1, rope).unwrap(),
            selection_range: span_to_range(&self.1, rope).unwrap(),
            children: Some(children),
        }
    }
}
