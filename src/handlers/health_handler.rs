use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use crate::app_state::AppState;
use axum::extract::State;
use chrono::Utc;

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub checks: HealthChecks,
}

#[derive(Serialize, Deserialize)]
pub struct HealthChecks {
    pub cache: CheckStatus,
}

#[derive(Serialize, Deserialize)]
pub struct CheckStatus {
    pub status: String,
    pub items: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ReadyResponse {
    pub ready: bool,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetricsResponse {
    pub cache_promo_count: usize,
    pub cache_store_count: usize,
    pub cache_promo_store_count: usize,
    pub cache_promo_tenor_count: usize,
}

pub async fn health_check(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let promo_count = state.cache_repository.get_promo_cache_all().read().await.len();
    let store_count = state.cache_repository.get_store_cache_all().read().await.len();
    let promo_store_count = state.cache_repository.get_promo_store_cache_all().read().await.len();
    let promo_tenor_count = state.cache_repository.get_promo_tenor_cache_all().read().await.len();
    
    let total_items = promo_count + store_count + promo_store_count + promo_tenor_count;
    let cache_status = if total_items > 0 { "up" } else { "warming" };

    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        checks: HealthChecks {
            cache: CheckStatus {
                status: cache_status.to_string(),
                items: total_items,
            },
        },
    })
}

pub async fn ready_check(State(state): State<Arc<AppState>>) -> Json<ReadyResponse> {
    let promo_count = state.cache_repository.get_promo_cache_all().read().await.len();
    let store_count = state.cache_repository.get_store_cache_all().read().await.len();
    
    let ready = promo_count > 0 && store_count > 0;

    Json(ReadyResponse {
        ready,
        timestamp: Utc::now().to_rfc3339(),
    })
}

pub async fn metrics(State(state): State<Arc<AppState>>) -> Json<MetricsResponse> {
    let promo_count = state.cache_repository.get_promo_cache_all().read().await.len();
    let store_count = state.cache_repository.get_store_cache_all().read().await.len();
    let promo_store_count = state.cache_repository.get_promo_store_cache_all().read().await.len();
    let promo_tenor_count = state.cache_repository.get_promo_tenor_cache_all().read().await.len();

    Json(MetricsResponse {
        cache_promo_count: promo_count,
        cache_store_count: store_count,
        cache_promo_store_count: promo_store_count,
        cache_promo_tenor_count: promo_tenor_count,
    })
}
