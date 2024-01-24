use std::sync::Arc;

use crate::core::parser::delcarations::ScopedItems;
use crate::core::parser::parser::File;
use crate::features::symbols::Symbol;
use crate::ls::capabilities::get_capabilities;
use dashmap::DashMap;
use ropey::Rope;
use serde_json::Value;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CodeActionOrCommand, CodeActionParams, CodeActionResponse, Command, CompletionParams,
    CompletionResponse, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
    DocumentSymbolParams, DocumentSymbolResponse, ExecuteCommandParams, InitializeParams,
    InitializeResult, MessageType, Url, WorkspaceEdit,
};
use tower_lsp::{Client, LanguageServer};

use super::completions::get_completions;
use super::diagnostics::parse_file;
use super::properties::{get_table_defs, parse_config};

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
        }
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn update_definitions(&self) {
        let defs = get_table_defs(&self).await;
        let mut scope = self.state.lock().await;
        scope.table_definitions = defs;
    }

    async fn refresh_diagnostics(&self) {
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
                let scope = ScopedItems {
                    table_definitions: defs,
                    ..Default::default()
                };
                self.state.lock().await.table_definitions = scope.table_definitions;
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
            return Ok(None);
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
            _ => {}
        }
        Ok(None)
    }

    async fn code_action(&self, _params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        Ok(Some(vec![CodeActionOrCommand::Command(Command {
            title: "Sync Definitions With Database".to_string(),
            command: "db.refresh".to_string(),
            arguments: Some(vec![serde_json::to_value(WorkspaceEdit::default()).unwrap()]),
        })]))
    }
}
