use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::{
    ast::expr::parser::Expression,
    declarations::{scoped_item::ScopedItems, type_::Type},
    features::diagnostics::diagnostic::HasDiagnosticsForType,
    util::span::Spanned,
};

impl HasDiagnosticsForType for Spanned<&Vec<Spanned<Expression>>> {
    fn diagnostics_for_type(
        &self,
        rope: &Rope,
        type_: &Type,
        scope: &mut ScopedItems,
    ) -> Vec<Diagnostic> {
        if let Type::Array(inner_type) = type_ {
            let mut diagnostics = vec![];
            for expr in self.0.iter() {
                diagnostics.extend(expr.diagnostics_for_type(rope, inner_type, scope));
            }
            diagnostics
        } else {
            vec![Diagnostic {
                range: Default::default(),
                severity: None,
                message: "Expected array type".to_string(),
                ..Default::default()
            }]
        }
    }
}
