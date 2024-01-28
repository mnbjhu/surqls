use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    ast::{
        expr::{parser::Expression, types::Typed},
        statement::crud::{select::SelectStatement, update::UpdateStatement},
    },
    declarations::{field::Field, object::Object, scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::{HasDiagnostic, HasDiagnosticsForType},
    util::span::Spanned,
};

use super::table_name::diagnostics_for_table_name;

impl HasDiagnostic for Spanned<&SelectStatement> {
    fn diagnostics(&self, rope: &Rope, scope: &ScopedItems) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        let mut scope = scope.clone();
        if let Some(from) = &self.0.from {
            diagnostics.extend(diagnostics_for_table_name(&from.0, &from.1, rope, &scope));
            if let Some(ty) = scope.table_definitions.get(&from.0) {
                for field in &ty.fields {
                    scope.scoped_table.fields.retain(|f| f.name != field.name);
                    scope.scoped_table.fields.push(field.clone());
                }
            }
            for projection in &self.0.projections {
                let expr = &projection.0.expr;
                diagnostics.extend(expr.diagnostics_for_type(rope, &Type::Any, &scope));
                if let Some(alias) = &projection.0.alias {
                    scope.scoped_table.fields.retain(|f| f.name != alias.0);
                    scope.scoped_table.fields.push(Field {
                        name: alias.0.clone(),
                        ty: expr.0.get_type(&scope),
                        is_required: match expr.0.get_type(&scope) {
                            Type::Option(_) => false,
                            _ => true,
                        },
                    });
                }
            }
            for transform in &self.0.transforms {
                diagnostics.extend(transform.0.diagnostics(rope, &scope));
            }
        }
        for projection in &self.0.projections {
            let expr = &projection.0.expr;
            diagnostics.extend(expr.diagnostics_for_type(rope, &Type::Any, &scope));
        }
        diagnostics
    }
}

impl Typed for SelectStatement {
    fn get_type(&self, scope: &ScopedItems) -> Type {
        let mut scope = scope.clone();
        let mut fields = vec![];
        if let Some(from) = &self.from {
            if let Some(ty) = scope.table_definitions.get(&from.0) {
                for field in &ty.fields {
                    scope.scoped_table.fields.retain(|f| f.name != field.name);
                    scope.scoped_table.fields.push(field.clone());
                }
            }
            for projection in &self.projections {
                let expr = &projection.0.expr;
                if let Some(alias) = &projection.0.alias {
                    scope.scoped_table.fields.retain(|f| f.name != alias.0);
                    scope.scoped_table.fields.push(Field {
                        name: alias.0.clone(),
                        ty: expr.0.get_type(&scope),
                        is_required: match expr.0.get_type(&scope) {
                            Type::Option(_) => false,
                            _ => true,
                        },
                    });
                    fields.push(Field {
                        name: alias.0.clone(),
                        ty: expr.0.get_type(&scope),
                        is_required: match expr.0.get_type(&scope) {
                            Type::Option(_) => false,
                            _ => true,
                        },
                    });
                } else {
                    if let Expression::Identifier(ident) = &expr.0 {
                        if let Some(ty) =
                            scope.scoped_table.fields.iter().find(|f| f.name == *ident)
                        {
                            fields.push(ty.clone());
                        }
                    }
                }
            }
            return Type::Array(Box::new(Type::Object(Object { fields })));
        }
        Type::Any
    }
}
