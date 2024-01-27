use serde_json::Value;
use tower_lsp::lsp_types::{MessageType, ShowDocumentParams};

use super::{backend::Backend, properties::SurrealResponse};

pub async fn send_query(query: String, backend: &Backend, root: String) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8000/sql")
        .body(query)
        .header("NS", "test")
        .header("DB", "test")
        .header("Accept", "application/json")
        .basic_auth("root", Some("root"))
        .send()
        .await;
    match res {
        Ok(res) => {
            let res = res.json::<Vec<SurrealResponse<Value>>>().await.unwrap()[0].clone();
            let SurrealResponse { status, result } = res;
            let result = serde_json::to_string_pretty(&result).unwrap();
            let timestamp = chrono::Local::now().format("%Y-%m-%d-%H:%M:%S");
            let root = root.strip_prefix("file://").unwrap();
            let save_path = format!("{}/.query/{}.json", root, timestamp);
            // Create the directory if it doesn't exist
            std::fs::create_dir_all(format!("{}/.query", root)).expect(
                format!(
                    "Failed to create directory '{}'",
                    format!("{}/.query", root)
                )
                .as_str(),
            );
            std::fs::write(save_path.clone(), result)
                .expect(format!("Failed to save query result to '{}'", save_path.clone()).as_str());
            backend
                .client
                .show_document(ShowDocumentParams {
                    uri: url::Url::from_file_path(save_path).unwrap(),
                    external: Some(true),
                    take_focus: Some(true),
                    selection: None,
                })
                .await
                .unwrap();
        }
        Err(e) => {
            backend
                .client
                .show_message(MessageType::ERROR, e.to_string())
                .await;
        }
    }
}

pub async fn query(query: String, backend: &Backend) -> Option<Value> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8000/sql")
        .body(query)
        .header("NS", "test")
        .header("DB", "test")
        .header("Accept", "application/json")
        .basic_auth("root", Some("root"))
        .send()
        .await;
    match res {
        Ok(res) => {
            let res = res.json::<Vec<SurrealResponse<Value>>>().await;
            if res.is_err() {
                backend
                    .client
                    .show_message(MessageType::ERROR, res.unwrap_err().to_string())
                    .await;
                return None;
            }
            let res = res.unwrap()[0].clone();
            let SurrealResponse { status: _, result } = res;
            // backend
            //     .client
            //     .show_message(MessageType::INFO, "Query successful".to_string())
            //     .await;
            Some(result)
        }
        Err(e) => {
            backend
                .client
                .show_message(MessageType::ERROR, e.to_string())
                .await;
            None
        }
    }
}

pub async fn update_remote_definition(query_text: String, backend: &Backend, root: String) {
    if let Some(_) = query(query_text, backend).await {
        backend.update_definitions().await;
        backend.refresh_diagnostics().await;
    } else {
        backend
            .client
            .show_message(
                MessageType::ERROR,
                "Failed to update remote definition".to_string(),
            )
            .await;
    }
}
