use std::collections::HashMap;

use chumsky::{input::Input, Parser};
use serde::Deserialize;
use tower_lsp::lsp_types::MessageType;

use crate::ast::statement::define::DefineStatement;
use crate::ast::statement::statement::Statement;
use crate::ast::type_::Type as AstType;
use crate::declarations::field::Field;
use crate::declarations::object::Object;
use crate::declarations::scoped_item::ScopedItems;
use crate::declarations::type_::Type;
use crate::{lexer::lexer::lexer, parser::parser::parser};

use super::backend::Backend;

#[derive(Deserialize)]
pub struct Properties {
    pub connection: Connection,
}

#[derive(Deserialize)]
pub struct Connection {
    pub host: String,
    pub port: i32,
    pub user: Option<String>,
    pub pass: Option<String>,
}

pub async fn parse_config(backend: &Backend) -> Result<(), ()> {
    let root = backend
        .properties
        .get("root_dir")
        .unwrap()
        .value()
        .to_string();

    let root = root.strip_prefix("file://").unwrap();
    let config_path = format!("{}/surqls.toml", root);

    let toml_str = std::fs::read_to_string(config_path);
    if let Err(e) = toml_str {
        backend
            .client
            .show_message(MessageType::ERROR, e.to_string())
            .await;
        return Err(());
    }
    let toml_str = toml_str.unwrap();
    let properties = toml::from_str::<Properties>(&toml_str);
    match properties {
        Ok(properties) => {
            backend
                .properties
                .insert("host".to_string(), properties.connection.host);
            backend
                .properties
                .insert("port".to_string(), properties.connection.port.to_string());
            if let Some(user) = properties.connection.user {
                backend
                    .properties
                    .insert("user".to_string(), user.to_string());
            }
            if let Some(pass) = properties.connection.pass {
                backend
                    .properties
                    .insert("pass".to_string(), pass.to_string());
            }
            Ok(())
        }
        Err(e) => {
            backend
                .client
                .show_message(MessageType::ERROR, e.to_string())
                .await;
            Err(())
        }
    }
}

pub async fn get_table_defs(backend: &Backend) -> HashMap<String, Object> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8000/sql")
        .body("INFO FOR DATABASE;")
        .header("NS", "test")
        .header("DB", "test")
        .header("Accept", "application/json")
        .basic_auth("root", Some("root"))
        .send()
        .await;
    let mut table_defs = HashMap::new();
    match res {
        Ok(res) => {
            let tables = res
                .json::<InfoResult>()
                .await
                .unwrap()
                .pop()
                .unwrap()
                .result
                .tables;
            for (name, _) in tables {
                let query = format!("INFO FOR TABLE {};", &name);
                let text = client
                    .post("http://localhost:8000/sql")
                    .body(query.clone())
                    .header("NS", "test")
                    .header("DB", "test")
                    .header("Accept", "application/json")
                    .basic_auth("root", Some("root"))
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let text = serde_json::from_str::<TableResult>(&text.clone())
                    .expect(format!("Failed to parse table {:?}, query: {}", text, &query).as_str())
                    .pop()
                    .unwrap()
                    .result
                    .fields
                    .into_iter()
                    .map(|(_, text)| text)
                    .collect::<Vec<_>>()
                    .join(";\n");

                let tokens = lexer().parse(&text).into_output().unwrap();
                let parser_result = parser()
                    .parse_with_state(
                        tokens.as_slice().spanned((text.len()..text.len()).into()),
                        &mut ScopedItems::default(),
                    )
                    .into_output()
                    .expect(format!("Failed to parse table {:?}", text).as_str());
                let statements = parser_result
                    .into_iter()
                    .map(|s| match s.0 {
                        Statement::Define(s) => s.0,
                        _ => panic!("Expected define statement"),
                    })
                    .collect::<Vec<_>>();
                let mut type_ = parse_table_defs(&statements, "".to_string(), backend).await;
                type_.fields.push(Field {
                    name: "id".to_string(),
                    ty: Type::Record(name.clone()),
                    is_required: false,
                });
                table_defs.insert(name, type_);
            }

            backend
                .client
                .log_message(MessageType::ERROR, format!("{:?}", table_defs))
                .await;
        }
        Err(e) => {
            backend
                .client
                .show_message(MessageType::ERROR, e.to_string())
                .await;
        }
    }
    table_defs
}

#[derive(Deserialize, Debug, Clone)]
pub struct SurrealResponse<T> {
    pub status: String,
    pub result: T,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseInfo {
    pub tables: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct TableInfo {
    pub fields: HashMap<String, String>,
}

type InfoResult = Vec<SurrealResponse<DatabaseInfo>>;
type TableResult = Vec<SurrealResponse<TableInfo>>;

#[async_recursion::async_recursion]
pub async fn parse_table_defs(
    statements: &Vec<DefineStatement>,
    parents: String,
    backend: &Backend,
) -> Object {
    backend
        .client
        .log_message(
            MessageType::ERROR,
            format!("parsing table defs parents: {}", parents),
        )
        .await;
    let mut fields = Vec::new();
    for statement in statements {
        match statement {
            DefineStatement::Field((field, _)) => {
                let parent = field
                    .parents
                    .clone()
                    .into_iter()
                    .map(|(p, _)| p)
                    .collect::<Vec<_>>()
                    .join(".");
                if parent == parents {
                    if &field.type_.0.name.0 == "object" {
                        let new_parent = if parents == "" {
                            field.name.0.clone()
                        } else {
                            format!("{}.{}", parents, field.name.0)
                        };
                        let ty =
                            Type::Object(parse_table_defs(statements, new_parent, backend).await);
                        fields.push(Field {
                            name: field.name.0.clone(),
                            ty,
                            is_required: false,
                        });
                    } else {
                        let ty = parse_declared_type(&field.type_.0);
                        fields.push(Field {
                            name: field.name.0.clone(),
                            is_required: match ty {
                                Type::Option(_) => false,
                                _ => true,
                            },
                            ty,
                        });
                    }
                }
            }
            DefineStatement::Table(_) => continue,
        }
    }
    Object { fields }
}

pub fn parse_declared_type(AstType { name, args }: &AstType) -> Type {
    match name.0.as_str() {
        "string" => Type::String,
        "int" => Type::Int,
        "float" => Type::Float,
        "boolean" => Type::Bool,
        "decimal" => Type::Decimal,
        "number" => Type::Number,
        "null" => Type::Null,
        "any" => Type::Any,
        "datetime" => Type::DateTime,
        "array" => {
            if args.len() != 1 {
                Type::Error
            } else {
                Type::Array(Box::new(parse_declared_type(&args[0].0)))
            }
        }
        "object" => Type::Object(Object {
            fields: args
                .iter()
                .map(|arg| {
                    let AstType { name, args: _ } = &arg.0;
                    let ty = parse_declared_type(&arg.0);
                    Field {
                        name: name.0.clone(),
                        is_required: match ty {
                            Type::Option(_) => false,
                            _ => true,
                        },
                        ty,
                    }
                })
                .collect::<Vec<_>>(),
        }),
        "option" => {
            if args.len() != 1 {
                Type::Error
            } else {
                Type::Option(Box::new(parse_declared_type(&args[0].0)))
            }
        }
        "record" => {
            if args.len() != 1 {
                Type::Error
            } else {
                let AstType { name, args } = &args[0].0;
                if args.len() != 0 {
                    Type::Error
                } else {
                    Type::Record(name.0.clone())
                }
            }
        }
        _ => Type::Any,
    }
}
