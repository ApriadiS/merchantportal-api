use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::app_state::AppState;
use axum::extract::State;

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetricsResponse {
    pub uptime_seconds: u64,
    pub cache_promo_count: usize,
    pub cache_store_count: usize,
    pub cache_promo_store_count: usize,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn metrics(State(state): State<Arc<AppState>>) -> Json<MetricsResponse> {
    let promo_count = state.cache_repository.get_promo_cache_all().read().await.len();
    let store_count = state.cache_repository.get_store_cache_all().read().await.len();
    let promo_store_count = state.cache_repository.get_promo_store_cache_all().read().await.len();

    Json(MetricsResponse {
        uptime_seconds: 0, // TODO: implement actual uptime tracking
        cache_promo_count: promo_count,
        cache_store_count: store_count,
        cache_promo_store_count: promo_store_count,
    })
}
