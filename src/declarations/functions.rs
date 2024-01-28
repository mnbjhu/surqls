use std::{collections::HashMap, fmt::Display};

use super::{func::count::get_count_functions, type_::Type};

#[derive(Clone, Debug)]
pub enum GenericType {
    Named(Type),
    TypeParam { super_: Type },
    GenericArray { inner_super_: Type },
    GenericOption { inner_super_: Type },
}

impl Display for GenericType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericType::Named(ref t) => write!(f, "{}", t),
            GenericType::TypeParam { super_: _ } => write!(f, "T"),
            GenericType::GenericArray { inner_super_: _ } => write!(f, "array<T>"),
            GenericType::GenericOption { inner_super_: _ } => write!(f, "option<T>"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub args: Vec<FunctionArg>,
    pub return_type: GenericType,
    pub doc: Option<String>,
}

#[derive(Clone, Debug)]
pub struct FunctionArg(pub String, pub GenericType);

impl Function {
    pub fn get_type(&self, arg: Type) -> Type {
        match self.return_type {
            GenericType::Named(ref t) => t.clone(),
            GenericType::TypeParam { super_: _ } => arg,
            GenericType::GenericArray { inner_super_: _ } => Type::Array(Box::new(arg)),
            GenericType::GenericOption { inner_super_: _ } => Type::Option(Box::new(arg)),
        }
    }

    pub fn get_arg_type(&self, args: Vec<Type>) -> Type {
        let mut found: Option<Type> = None;
        for (index, FunctionArg(_, expected)) in self.args.iter().enumerate() {
            let actual = args.get(index).unwrap();
            match expected {
                GenericType::Named(ref t) => {}
                GenericType::TypeParam { super_ } => {
                    let found_type = if actual.is_assignable_to(super_) {
                        actual.clone()
                    } else {
                        Type::Error
                    };
                    let new_type = match found {
                        Some(ref found_type) => found_type.get_shared_super_type(&found_type),
                        None => found_type,
                    };
                    let found = Some(new_type);
                }
                GenericType::GenericArray { inner_super_ } => {
                    if let Type::Array(ref t) = actual {
                        if t.is_assignable_to(inner_super_) {
                            return t.as_ref().clone();
                        } else {
                            return Type::Error;
                        }
                    }
                }
                GenericType::GenericOption { inner_super_ } => {
                    if let Type::Option(ref t) = actual {
                        if t.is_assignable_to(inner_super_) {
                            return t.as_ref().clone();
                        } else {
                            return Type::Error;
                        }
                    }
                }
            }
        }
        match found {
            Some(t) => t,
            None => Type::Any,
        }
    }

    pub fn get_return_type(&self, args: Vec<Type>) -> Type {
        let arg_type = self.get_arg_type(args);
        self.get_type(arg_type)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut args = vec![];
        for arg in &self.args {
            args.push(format!("{}: {}", arg.0, arg.1));
        }
        write!(f, "({}) -> {}", args.join(", "), self.return_type)
    }
}

pub fn get_functions() -> HashMap<String, Function> {
    return get_count_functions();
}
