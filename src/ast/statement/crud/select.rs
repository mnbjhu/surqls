use crate::{
    ast::{projection::Projection, statement::transform::Transform},
    util::span::Spanned,
};

#[derive(Debug, Clone)]
pub struct SelectStatement {
    pub projections: Vec<Spanned<Projection>>,
    pub from: Option<Spanned<String>>,
    pub transforms: Vec<Spanned<Transform>>,
}
