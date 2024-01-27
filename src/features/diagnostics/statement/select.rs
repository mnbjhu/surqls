use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    ast::{expr::types::Typed, statement::crud::select::SelectStatement},
    declarations::{field::Field, scoped_item::ScopedItems, type_::Type},
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
