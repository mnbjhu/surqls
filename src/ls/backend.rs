use crate::core::parser::parser::File;
use crate::core::parser::symbol::Symbol;
use crate::ls::capabilities::get_capabilities;
use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CompletionParams, CompletionResponse, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
    DocumentSymbolParams, DocumentSymbolResponse, InitializeParams, InitializeResult, Url,
};
use tower_lsp::{Client, LanguageServer};

use super::completions::get_completions;
use super::diagnostics::parse_file;

pub struct Backend {
    pub client: Client,
    pub document_map: DashMap<String, Rope>,
    pub properties: DashMap<String, String>,
    pub ast_map: DashMap<String, File>,
}

impl Backend {
    async fn change(&self, uri: Url, text: String) {
        // let root = self.properties.get("root_dir").unwrap().value().to_string();
        let filename = uri.to_string();
        let rope = Rope::from_str(&text);
        self.document_map.insert(filename.clone(), rope.clone());
        let (ast, diagnostics) = parse_file(text.clone(), &rope);
        if let Some(ast) = ast {
            self.ast_map.insert(filename.clone(), ast);
        }
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let root_dir = params.root_uri.clone().unwrap().to_string();
        self.properties
            .insert("root_dir".to_string(), root_dir.clone());
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
}
