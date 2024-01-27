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
        scope: &ScopedItems,
    ) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if let Type::Array(inner_type) = type_ {
            for expr in self.0.iter() {
                diagnostics.extend(expr.diagnostics_for_type(rope, inner_type, scope));
            }
        }
        diagnostics
    }
}
