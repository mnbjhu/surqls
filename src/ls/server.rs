use crate::ls::backend::Backend;
use dashmap::DashMap;
use tower_lsp::LspService;
use tower_lsp::Server;

pub async fn launch_server() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        document_map: DashMap::new(),
        properties: DashMap::new(),
        ast_map: DashMap::new(),
    })
    .finish();

    serde_json::json!({"test": 20});
    Server::new(stdin, stdout, socket).serve(service).await;
}
