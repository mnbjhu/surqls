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
                Literal::Decimal(_) => Type::Decimal,
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
                        let mut array_nest_count = 0;
                        let mut ty = ty;
                        while let Type::Array(inner_ty) = ty {
                            ty = *inner_ty.clone();
                            array_nest_count += 1;
                        }
                        if let Type::Object(obj) = ty {
                            if let Some(field) = &obj.get_field(name) {
                                let mut ty = field.ty.clone();
                                for _ in 0..array_nest_count {
                                    ty = Type::Array(Box::new(ty));
                                }
                                return ty;
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
                    let value = match &field.0.value {
                        Some(expr) => expr.0.get_type(scope),
                        None => Type::Any,
                    };
                    fields.push(Field {
                        name: field.0.key.0.clone(),
                        ty: match &field.0.value {
                            Some(expr) => expr.0.get_type(scope),
                            None => Type::Error,
                        },
                        is_required: match value {
                            Type::Option(_) => false,
                            _ => true,
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
                    ty = ty.get_shared_super_type(&item_ty)
                }
                Type::Array(Box::new(ty))
            }
            Expression::Call { name, args } => {
                let name = name
                    .iter()
                    .map(|name| name.0.clone())
                    .collect::<Vec<String>>()
                    .join("::");
                if let Some(def) = &scope.functions.get(&name) {
                    if let Some(args) = args {
                        let arg_types = args
                            .iter()
                            .map(|arg| arg.0.get_type(scope))
                            .collect::<Vec<Type>>();
                        return def.get_return_type(arg_types);
                    };
                    def.get_return_type(vec![])
                } else {
                    Type::Error
                }
            }
            Expression::Inline(s) => s.as_ref().0.get_type(scope),
            _ => Type::Error,
        }
    }
}
