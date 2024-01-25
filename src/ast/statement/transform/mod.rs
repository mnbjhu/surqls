use crate::{
    ast::expr::parser::Expression, parser::statement::transform::Unexpected, util::span::Spanned,
};

pub enum Transform {
    Where(Option<Spanned<Expression>>),
    Limit(Option<Spanned<Expression>>),
    Skip(Option<Spanned<Expression>>),
    Invalid(Unexpected),
}
