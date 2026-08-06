use std::{env, sync::Arc};

use anyhow::{Context, Result};
use lume_service::{IngestService, serve_tcp};
use lume_storage::HistoryStore;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = env::var("LUME_ENDPOINT").unwrap_or_else(|_| "127.0.0.1:43191".into());
    let history = env::var("LUME_HISTORY").unwrap_or_else(|_| "lume-history.sqlite".into());
    let token = env::var("LUME_AUTH_TOKEN").unwrap_or_else(|_| "local-development-token".into());

    let listener = TcpListener::bind(&endpoint)
        .await
        .with_context(|| format!("failed to bind Lume Service at {endpoint}"))?;
    let store = Arc::new(
        HistoryStore::open(&history)
            .with_context(|| format!("failed to open Local History at {history}"))?,
    );
    println!("Lume Service listening on {endpoint}");
    serve_tcp(listener, IngestService::new(store, token), async {
        let _ = tokio::signal::ctrl_c().await;
    })
    .await?;
    Ok(())
}
