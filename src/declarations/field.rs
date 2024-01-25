use super::type_::Type;

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}
