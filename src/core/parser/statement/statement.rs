use chumsky::extra::State;
use ropey::Rope;
use tower_lsp::{
    lsp_types::{
        CompletionItem, Diagnostic, DiagnosticSeverity, DocumentSymbol, Position, SymbolKind,
    },
    Client,
};

use crate::{
    core::{
        parser::{
            completion::HasCompletionItems, delcarations::ScopedItems, diagnostic::HasDiagnostic,
            symbol::Symbol,
        },
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
    fn diagnostics(&self, rope: &Rope, scope: &mut ScopedItems) -> Vec<Diagnostic> {
        let text = rope.slice(self.1.start..self.1.end);
        match &self.0 {
            Statement::Crud(crud) => (crud, self.1).diagnostics(rope, scope),
            Statement::Create(create) => (create, self.1).diagnostics(rope, scope),
            Statement::Invalid => vec![Diagnostic {
                range: span_to_range(&self.1, rope).unwrap(),
                severity: Some(DiagnosticSeverity::ERROR),
                message: format!("Invalid Statement: '{}'", text),
                ..Default::default()
            }],
        }
    }
}

impl HasCompletionItems for Statement {
    fn get_completion_items(
        &self,
        scope: &mut ScopedItems,
        position: Position,
        rope: &Rope,
        client: &Client,
    ) -> Vec<CompletionItem> {
        match &self {
            Statement::Crud(crud) => vec![],
            Statement::Create(create) => create.get_completion_items(scope, position, rope, client),
            Statement::Invalid => vec![],
        }
    }
}
