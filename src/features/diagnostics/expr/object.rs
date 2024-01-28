use std::collections::HashMap;

use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity};

use crate::{
    ast::expr::object::ObjectEntry,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::HasDiagnosticsForType,
    util::{range::span_to_range, span::Spanned},
};

impl HasDiagnosticsForType for Spanned<&Vec<Spanned<ObjectEntry>>> {
    fn diagnostics_for_type(
        &self,
        rope: &Rope,
        type_: &Type,
        scope: &ScopedItems,
    ) -> Vec<Diagnostic> {
        match type_ {
            Type::Object(obj) => {
                let mut diagnostics = vec![];
                let mut missing = obj.fields.clone().into_iter().collect::<Vec<_>>();
                let mut defined: HashMap<String, Spanned<ObjectEntry>> = HashMap::new();
                for entry in self.0 {
                    let ObjectEntry { key, value } = &entry.0;
                    missing.retain(|x| x.name != key.0);
                    if let Some(defined_field) = obj.get_field(&key.0.to_string()) {
                        if let Some(prev) = defined.get(&key.0) {
                            diagnostics.push(Diagnostic {
                                range: span_to_range(&key.1, rope).unwrap(),
                                severity: Some(DiagnosticSeverity::ERROR),
                                message: format!("Duplicated entry for field '{}'", key.0),
                                ..Default::default()
                            });
                            diagnostics.push(Diagnostic {
                                range: span_to_range(&prev.0.key.1, rope).unwrap(),
                                severity: Some(DiagnosticSeverity::INFORMATION),
                                message: format!("Field '{}' previously defined here", key.0),
                                ..Default::default()
                            });
                        } else {
                            defined.insert(key.0.clone(), entry.clone());
                            if let Some(value) = value {
                                diagnostics.extend(value.diagnostics_for_type(
                                    rope,
                                    &defined_field.ty,
                                    scope,
                                ));
                            }
                        }
                    } else {
                        diagnostics.push(Diagnostic {
                            range: span_to_range(&key.1, rope).unwrap(),
                            severity: Some(DiagnosticSeverity::ERROR),
                            message: format!("Field {} does not exist", key.0),
                            ..Default::default()
                        });
                    }
                }
                if missing.len() > 0 {
                    diagnostics.push(Diagnostic {
                        range: span_to_range(&self.1, rope).unwrap(),
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: format!(
                            "Missing fields: {}",
                            missing
                                .into_iter()
                                .map(|x| x.name)
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                        ..Default::default()
                    });
                }
                diagnostics
            }
            _ => {
                vec![]
            }
        }
    }
}
