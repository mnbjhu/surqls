use chumsky::{extra, prelude::Rich};

use crate::{
    declarations::scoped_item::ScopedItems,
    lexer::token::Token,
    util::span::{Span, Spanned},
};

use super::statement::statement::Statement;

pub type File = Vec<Spanned<Statement>>;
pub type Extra<'tokens> = extra::Full<Rich<'tokens, Token, Span>, ScopedItems, ()>;
