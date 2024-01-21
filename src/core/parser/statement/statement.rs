use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, DocumentSymbol, SymbolKind};

use crate::{
    core::{
        parser::{diagnostic::HasDiagnostic, symbol::Symbol},
        span::Spanned,
    },
    ls::util::range::span_to_range,
};

use super::{create_statement::CreateStatement, crud_statement::CrudStatement};

pub enum Statement {
    Crud(CrudStatement),
    Create(CreateStatement),
    Invalid,
}

impl Symbol for &Spanned<Statement> {
    fn get_document_symbol(&self, rope: &Rope) -> DocumentSymbol {
        let text = rope.slice(self.1.start..self.1.end);
        match &self.0 {
            Statement::Crud(crud) => (crud, self.1).get_document_symbol(rope),
            Statement::Create(create) => (create, self.1).get_document_symbol(rope),
            Statement::Invalid => DocumentSymbol {
                name: format!("Invalid Statement: '{}'", text),
                kind: SymbolKind::NULL,
                tags: None,
                detail: None,
                deprecated: None,
                range: span_to_range(&self.1, rope).unwrap(),
                selection_range: span_to_range(&self.1, rope).unwrap(),
                children: None,
            },
        }
    }
}

impl HasDiagnostic for Spanned<Statement> {
    fn diagnostics(&self, rope: &Rope) -> Vec<Diagnostic> {
        let text = rope.slice(self.1.start..self.1.end);
        match &self.0 {
            Statement::Crud(crud) => (crud, self.1).diagnostics(rope),
            Statement::Create(create) => (create, self.1).diagnostics(rope),
            Statement::Invalid => vec![Diagnostic {
                range: span_to_range(&self.1, rope).unwrap(),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!("Invalid Statement: '{}'", text),
                ..Default::default()
            }],
        }
    }
}
