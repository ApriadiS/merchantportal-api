use crate::app_state::AppState;
use crate::error::AppError;
use crate::model::promo_store_model::PromoStore;
use crate::model::promo_store_model::{CreatePromoStorePayload, UpdatePromoStorePayload};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct PromoStoreQuery {
    pub promo_id: Option<i64>,
    pub store_id: Option<i64>,
}

pub async fn han_get_promo_stores(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PromoStoreQuery>,
) -> Result<Json<Vec<PromoStore>>, AppError> {
    if let Some(promo_id) = query.promo_id {
        let promo_stores = state.promo_store_service.ser_get_promo_stores_by_promo_id(promo_id).await?;
        return Ok(Json(promo_stores));
    }

    if let Some(store_id) = query.store_id {
        let promo_stores = state.promo_store_service.ser_get_promo_stores_by_store_id(store_id).await?;
        return Ok(Json(promo_stores));
    }

    let promo_stores = state.promo_store_service.ser_get_all_promo_stores().await?;
    if promo_stores.is_empty() {
        return Err(AppError::NotFound("No promo stores found".to_string()));
    }

    Ok(Json(promo_stores))
}

#[derive(Deserialize)]
pub struct PromoStoreKey {
    promo_id: i64,
    store_id: i64,
}

pub async fn han_get_promo_store_by_key(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Result<Json<PromoStore>, AppError> {
    let parts: Vec<&str> = key.split('-').collect();
    if parts.len() != 2 {
        return Err(AppError::BadRequest("Invalid key format. Use: promo_id-store_id".to_string()));
    }

    let promo_id = parts[0].parse::<i64>()
        .map_err(|_| AppError::BadRequest("Invalid promo_id".to_string()))?;
    let store_id = parts[1].parse::<i64>()
        .map_err(|_| AppError::BadRequest("Invalid store_id".to_string()))?;

    let promo_store = state
        .promo_store_service
        .ser_get_promo_store_by_key(promo_id, store_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "PromoStore dengan promo_id '{}' dan store_id '{}' tidak ditemukan",
                promo_id, store_id
            ))
        })?;

    Ok(Json(promo_store))
}

// reuse payload types from services to avoid duplicate type definitions

pub async fn han_create_promo_store(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePromoStorePayload>,
) -> Result<Json<PromoStore>, AppError> {
    let created = state
        .promo_store_service
        .ser_create_promo_store(payload)
        .await?;
    Ok(Json(created))
}

pub async fn han_update_promo_store(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
    Json(payload): Json<UpdatePromoStorePayload>,
) -> Result<Json<PromoStore>, AppError> {
    let parts: Vec<&str> = key.split('-').collect();
    if parts.len() != 2 {
        return Err(AppError::BadRequest("Invalid key format. Use: promo_id-store_id".to_string()));
    }

    let promo_id = parts[0].parse::<i64>()
        .map_err(|_| AppError::BadRequest("Invalid promo_id".to_string()))?;
    let store_id = parts[1].parse::<i64>()
        .map_err(|_| AppError::BadRequest("Invalid store_id".to_string()))?;

    if let Err(_) = han_get_promo_store_by_key(State(state.clone()), Path(key.clone())).await {
        return Err(AppError::NotFound(format!(
            "PromoStore with promo_id '{}' and store_id '{}' not found",
            promo_id, store_id
        )));
    }

    let updated = state
        .promo_store_service
        .ser_update_promo_store(promo_id, store_id, payload)
        .await?;
    Ok(Json(updated))
}

pub async fn han_delete_promo_store(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Result<Json<()>, AppError> {
    let parts: Vec<&str> = key.split('-').collect();
    if parts.len() != 2 {
        return Err(AppError::BadRequest("Invalid key format. Use: promo_id-store_id".to_string()));
    }

    let promo_id = parts[0].parse::<i64>()
        .map_err(|_| AppError::BadRequest("Invalid promo_id".to_string()))?;
    let store_id = parts[1].parse::<i64>()
        .map_err(|_| AppError::BadRequest("Invalid store_id".to_string()))?;

    if let Err(_) = han_get_promo_store_by_key(State(state.clone()), Path(key)).await {
        return Err(AppError::NotFound(format!(
            "PromoStore with promo_id '{}' and store_id '{}' not found",
            promo_id, store_id
        )));
    }

    state
        .promo_store_service
        .ser_delete_promo_store(promo_id, store_id)
        .await?;
    Ok(Json(()))
}
