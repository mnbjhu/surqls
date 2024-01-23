use ropey::Rope;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    core::{parser::expr::parser::Expression, span::Spanned},
    ls::util::range::span_to_range,
};

use super::create_statement::CreateStatement;

pub enum Statement {
    Create(CreateStatement),
    Return(Spanned<Expression>),
    Invalid,
}
