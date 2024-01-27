use crate::declarations::{field::Field, object::Object, scoped_item::ScopedItems, type_::Type};

use super::{access::Access, literal::Literal, parser::Expression};

pub trait Typed {
    fn get_type(&self, scope: &ScopedItems) -> Type;
}

impl Typed for Expression {
    fn get_type(&self, scope: &ScopedItems) -> Type {
        match self {
            Expression::Literal(literal) => match literal {
                Literal::String(_) => Type::String,
                Literal::Int(_) => Type::Int,
                Literal::Float(_) => Type::Float,
                Literal::Bool(_) => Type::Bool,
                Literal::Null => Type::Null,
                Literal::DateTime(_) => Type::DateTime,
                Literal::Duration(_) => Type::Duration,
                Literal::RecordString(s) => {
                    let split = s.split(':').collect::<Vec<&str>>();
                    let table = split[0];
                    Type::Record(table.to_string())
                }
            },
            Expression::Identifier(name) => {
                if let Some(field) = scope.scoped_table.get_field(name) {
                    field.ty.clone()
                } else {
                    Type::Error
                }
            }
            Expression::Variable(name) => {
                if let Some(ty) = scope.variables.get(name) {
                    ty.clone()
                } else {
                    Type::Error
                }
            }
            Expression::Access { expr, access } => {
                let ty = expr.0.get_type(scope);
                match &access.0.as_ref() {
                    Access::Property(name) => {
                        if let Type::Object(obj) = ty {
                            if let Some(field) = &obj.get_field(name) {
                                return field.ty.clone();
                            };
                        };
                        Type::Error
                    }
                    Access::Index(_) => {
                        if let Type::Array(ty) = ty {
                            return *ty.clone();
                        };
                        Type::Error
                    }
                }
            }
            Expression::Object(object) => {
                let mut fields = Vec::new();
                for field in object {
                    fields.push(Field {
                        name: field.0.key.0.clone(),
                        ty: match &field.0.value {
                            Some(expr) => expr.0.get_type(scope),
                            None => Type::Error,
                        },
                    });
                }
                Type::Object(Object { fields })
            }
            Expression::Array(array) => {
                if array.is_empty() {
                    return Type::Array(Box::new(Type::Any));
                }
                let mut ty = array[0].0.get_type(scope);
                for item in array {
                    let item_ty = item.0.get_type(scope);
                    if !item_ty.is_assignable_to(&ty) {
                        ty = ty.get_shared_super_type(&item_ty)
                    }
                }
                Type::Array(Box::new(ty))
            }
            _ => Type::Error,
        }
    }
}
