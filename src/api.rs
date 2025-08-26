use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt as _;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub tx: tokio::sync::broadcast::Sender<String>,
}

pub async fn serve(bind: String, db: SqlitePool, tx: tokio::sync::broadcast::Sender<String>) -> anyhow::Result<()> {
    let state = AppState { db, tx };

    let app = Router::new()
        .route("/health", get(health))
        .route("/burns", get(burns))
        .route("/attestations", get(attestations))
        .route("/events", get(events))
        .with_state(state);

    tracing::info!("HTTP API listening on http://{bind}");
    axum::Server::bind(&bind.parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"ok": true}))
}

#[derive(Deserialize)]
struct ListQ { limit: Option<i64> }

async fn burns(State(st): State<AppState>, Query(q): Query<ListQ>) -> Json<serde_json::Value> {
    let limit = q.limit.unwrap_or(50).clamp(1, 500);
    let rows = crate::repo::list_burns(&st.db, limit).await.unwrap_or_default();
    Json(serde_json::json!({ "items": rows }))
}

async fn attestations(State(st): State<AppState>, Query(q): Query<ListQ>) -> Json<serde_json::Value> {
    let limit = q.limit.unwrap_or(50).clamp(1, 500);
    let rows = crate::repo::list_attestations(&st.db, limit).await.unwrap_or_default();
    Json(serde_json::json!({ "items": rows }))
}

async fn events(State(st): State<AppState>) -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    let mut rx = st.tx.subscribe();
    let stream = async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    yield Ok(Event::default().event("relayer").data(msg));
                }
                Err(_) => {
                    tokio::time::sleep(Duration::from_millis(200)).await;
                }
            }
        }
    };
    Sse::new(stream)
}
