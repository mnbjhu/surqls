use std::sync::Arc;

use crate::ast::parser::File;
use crate::ast::statement::define::DefineStatement;
use crate::ast::statement::statement::Statement;
use crate::declarations::scoped_item::ScopedItems;
use crate::features::completions::completions::get_completions;
use crate::features::diagnostics::diagnostic::parse_file;
use crate::features::symbols::Symbol;
use crate::ls::capabilities::get_capabilities;
use crate::util::range::span_to_range;
use dashmap::DashMap;
use ropey::Rope;
use serde_json::Value;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CodeActionOrCommand, CodeActionParams, CodeActionResponse, Command, CompletionParams,
    CompletionResponse, DidChangeTextDocumentParams, DidOpenTextDocumentParams, DocumentSymbol,
    DocumentSymbolParams, DocumentSymbolResponse, ExecuteCommandParams, InitializeParams,
    InitializeResult, MessageType, Position, Range, SymbolKind, Url, WorkspaceEdit,
};
use tower_lsp::{Client, LanguageServer};

use super::properties::{get_table_defs, parse_config};
use super::query::{query, send_query, update_remote_definition};

pub struct Backend {
    pub client: Client,
    pub document_map: DashMap<String, Rope>,
    pub properties: DashMap<String, String>,
    pub ast_map: DashMap<String, File>,
    pub state: Arc<Mutex<ScopedItems>>,
}

impl Backend {
    async fn change(&self, uri: Url, text: String) {
        let filename = uri.to_string();
        let rope = Rope::from_str(&text);
        let mut scope = self.state.lock().await;
        self.document_map.insert(filename.clone(), rope.clone());
        let (ast, diagnostics) = parse_file(text.clone(), &rope, &mut scope);
        if let Some(ast) = ast {
            self.ast_map.insert(filename.clone(), ast);
        } else {
            self.ast_map.remove(filename.clone().as_str());
        }
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    pub async fn update_definitions(&self) {
        let defs = get_table_defs(&self).await;
        let mut scope = self.state.lock().await;
        scope.table_definitions = defs;
    }

    pub async fn refresh_diagnostics(&self) {
        let mut scope = self.state.lock().await;
        for (uri, rope) in self.document_map.clone().into_iter() {
            let (ast, diagnostics) = parse_file(rope.to_string(), &rope, &mut scope);
            if let Some(ast) = ast {
                self.ast_map.insert(uri.clone(), ast);
            }
            let uri = Url::parse(&uri).unwrap();
            self.client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let root_dir = params.root_uri.clone().unwrap().to_string();
        self.properties
            .insert("root_dir".to_string(), root_dir.clone());
        match parse_config(self).await {
            Ok(_) => {
                let defs = get_table_defs(&self).await;
                self.state.lock().await.table_definitions = defs;
            }
            Err(err) => {
                self.client
                    .show_message(MessageType::ERROR, format!("{:?}", err))
                    .await;
            }
        }
        // let path = params.root_uri.unwrap().to_file_path().unwrap();
        Ok(get_capabilities())
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.change(uri, text).await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.content_changes.pop().unwrap().text;
        self.change(uri, text).await;
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        let rope = self.document_map.get(uri.to_string().as_str()).unwrap();
        let ast = self.ast_map.get(uri.to_string().as_str());
        if ast.is_none() {
            return Ok(Some(DocumentSymbolResponse::Nested(vec![DocumentSymbol {
                name: "Error".to_string(),
                kind: SymbolKind::NULL,
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                selection_range: Default::default(),
                children: None,
                detail: None,
                deprecated: None,
                tags: None,
            }])));
        }
        let ast = ast.unwrap();
        let mut symbols = Vec::new();
        for statement in ast.value() {
            let stm = statement.get_document_symbol(&rope.value());
            symbols.push(stm);
        }
        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let completions = get_completions(self, _params).await;
        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        let command = params.command;
        match command.as_str() {
            "db.refresh" => {
                self.update_definitions().await;
                self.refresh_diagnostics().await;
                self.client
                    .show_message(MessageType::INFO, "Definitions Refreshed")
                    .await;
            }
            "db.run" => {
                let params: CodeActionParams =
                    serde_json::from_value(params.arguments[0].clone()).unwrap();
                let uri = params.text_document.uri;
                let rope = self.document_map.get(uri.to_string().as_str()).unwrap();
                let ast = self.ast_map.get(uri.to_string().as_str());
                for (_, span) in ast.unwrap().value() {
                    let range = span_to_range(span, &rope.value()).unwrap();
                    if range.start <= params.range.start && params.range.start <= range.end {
                        let query = rope.slice((span.start)..(span.end)).to_string();
                        let root = self.properties.get("root_dir").unwrap().clone();
                        send_query(query, &self, root).await;
                    }
                }
            }
            "db.define" => {
                let params: CodeActionParams =
                    serde_json::from_value(params.arguments[0].clone()).unwrap();
                let uri = params.text_document.uri;
                let rope = self.document_map.get(uri.to_string().as_str()).unwrap();
                let ast = self.ast_map.get(uri.to_string().as_str());
                for (_, span) in ast.unwrap().value() {
                    let range = span_to_range(span, &rope.value()).unwrap();
                    if range.start <= params.range.start && params.range.start <= range.end {
                        let query_text = rope.slice((span.start)..(span.end)).to_string();
                        query(query_text, &self).await;
                        self.client
                            .log_message(MessageType::WARNING, "Query Sent")
                            .await;
                    }
                }
                self.client
                    .log_message(MessageType::WARNING, "Def found")
                    .await;
                self.update_definitions().await;
                self.client
                    .log_message(MessageType::WARNING, "Definitions Updated")
                    .await;
                self.refresh_diagnostics().await;
                self.client
                    .log_message(MessageType::WARNING, "Diagnostic Refreshed")
                    .await;
            }
            _ => {}
        }
        Ok(None)
    }

    async fn code_action(&self, _params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let rope = self
            .document_map
            .get(_params.text_document.uri.to_string().as_str())
            .unwrap()
            .value()
            .clone();

        let mut actions = Vec::new();
        if let Some(ast) = self
            .ast_map
            .get(_params.text_document.uri.to_string().as_str())
        {
            let pos = _params.range.start;
            for (stmt, span) in ast.value() {
                let range = span_to_range(span, &rope).unwrap();
                if range.start <= pos && pos <= range.end {
                    match &stmt {
                        Statement::Define(_) => {
                            let refresh = CodeActionOrCommand::Command(Command {
                                title: "Update remote definition".to_string(),
                                command: "db.define".to_string(),
                                arguments: Some(vec![
                                    serde_json::to_value(_params.clone()).unwrap()
                                ]),
                            });
                            actions.push(refresh);
                        }
                        _ => {
                            let run = CodeActionOrCommand::Command(Command {
                                title: "Run Query".to_string(),
                                command: "db.run".to_string(),
                                arguments: Some(vec![
                                    serde_json::to_value(_params.clone()).unwrap()
                                ]),
                            });
                            actions.push(run);
                        }
                    }
                }
            }
        }
        let refresh = CodeActionOrCommand::Command(Command {
            title: "Sync Definitions With Database".to_string(),
            command: "db.refresh".to_string(),
            arguments: Some(vec![serde_json::to_value(WorkspaceEdit::default()).unwrap()]),
        });
        actions.push(refresh);
        Ok(Some(actions))
    }
}
