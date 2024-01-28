use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    ast::statement::define::DefineStatement,
    declarations::{field::Field, scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::HasDiagnostic,
    ls::properties::parse_declared_type,
    util::{range::span_to_range, span::Spanned},
};

impl HasDiagnostic for Spanned<&DefineStatement> {
    fn diagnostics(&self, rope: &Rope, scope: &ScopedItems) -> Vec<Diagnostic> {
        match &self.0 {
            DefineStatement::Table(table) => match scope.table_definitions.get(&table.0.name.0) {
                Some(_) => vec![],
                None => vec![Diagnostic {
                    range: span_to_range(&self.1, rope).unwrap(),
                    severity: Some(DiagnosticSeverity::WARNING),
                    message: "Table not defined on database".to_string(),
                    ..Default::default()
                }],
            },
            DefineStatement::Field(field) => {
                let table_name = &field.0.table_name.0;
                let field_name = &field.0.name.0;
                match scope.table_definitions.get(table_name) {
                    Some(table) => {
                        let mut scoped_type = table.clone();
                        for parent in &field.0.parents {
                            if let Some(new_scoped_type) = scoped_type.get_field(&parent.0) {
                                if let Type::Object(obj) = &new_scoped_type.ty {
                                    scoped_type = obj.clone();
                                } else {
                                    return vec![Diagnostic {
                                        range: span_to_range(&parent.1, rope).unwrap(),
                                        severity: Some(DiagnosticSeverity::ERROR),
                                        message: format!("Field {} is not an object", parent.0,),
                                        ..Default::default()
                                    }];
                                }
                            } else {
                                return vec![Diagnostic {
                                    range: span_to_range(&parent.1, rope).unwrap(),
                                    severity: Some(DiagnosticSeverity::ERROR),
                                    message: format!("Field {} not found", parent.0,),
                                    ..Default::default()
                                }];
                            }
                        }
                        match scoped_type.get_field(field_name) {
                            Some(ty) => {
                                let declared_type = parse_declared_type(&field.0.type_.0);
                                let both_object = match (&ty.ty, &declared_type) {
                                    (Type::Object(_), Type::Object(_)) => true,
                                    _ => false,
                                };
                                if ty.ty != declared_type && !both_object {
                                    return vec![Diagnostic {
                                        range: span_to_range(&self.1, rope).unwrap(),
                                        severity: Some(DiagnosticSeverity::WARNING),
                                        message: format!(
                                            "Field doesn't match database, remote: {}, local: {}",
                                            ty.ty, declared_type
                                        ),
                                        ..Default::default()
                                    }];
                                }
                            }
                            None => {
                                return vec![Diagnostic {
                                    range: span_to_range(&self.1, rope).unwrap(),
                                    severity: Some(DiagnosticSeverity::WARNING),
                                    message: "Field not defined on database".to_string(),
                                    ..Default::default()
                                }]
                            }
                        };
                        vec![]
                    }
                    None => vec![Diagnostic {
                        range: span_to_range(&field.0.table_name.1, rope).unwrap(),
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: "Table not found".to_string(),
                        ..Default::default()
                    }],
                }
            }
        }
    }
}
