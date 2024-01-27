use crate::{
    ast::expr::parser::Expression, parser::statement::transform::Unexpected, util::span::Spanned,
};

#[derive(Debug, Clone)]
pub enum Transform {
    Where(Option<Spanned<Expression>>),
    Limit(Option<Spanned<Expression>>),
    Skip(Option<Spanned<Expression>>),
    Invalid(Unexpected),
}
