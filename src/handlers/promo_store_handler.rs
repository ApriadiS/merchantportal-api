use crate::app_state::AppState;
use crate::error::{AppError, PromoStoreError};
use crate::model::promo_store_model::PromoStore;
use crate::model::promo_store_model::{CreatePromoStorePayload, UpdatePromoStorePayload};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PromoStoreQuery {
    pub promo_id: Option<Uuid>,
    pub store_id: Option<Uuid>,
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
        return Err(PromoStoreError::NotFound("No promo store relations found".to_string()).into());
    }
    Ok(Json(promo_stores))
}

#[derive(Deserialize)]
pub struct PromoStoreKey {
    promo_id: Uuid,
    store_id: Uuid,
}

pub async fn han_get_promo_store_by_key(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Result<Json<PromoStore>, AppError> {
    if key.len() != 73 {
        return Err(PromoStoreError::InvalidKey("Invalid key format. Expected: {uuid}-{uuid}".to_string()).into());
    }

    let promo_id = Uuid::parse_str(&key[0..36])
        .map_err(|_| PromoStoreError::InvalidKey("Invalid promo_id format".to_string()))?;
    let store_id = Uuid::parse_str(&key[37..73])
        .map_err(|_| PromoStoreError::InvalidKey("Invalid store_id format".to_string()))?;

    let promo_store = state
        .promo_store_service
        .ser_get_promo_store_by_key(promo_id, store_id)
        .await?
        .ok_or_else(|| {
            PromoStoreError::NotFound(format!(
                "PromoStore with promo_id '{}' and store_id '{}' not found",
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
    if key.len() != 73 {
        return Err(PromoStoreError::InvalidKey("Invalid key format. Expected: {uuid}-{uuid}".to_string()).into());
    }

    let promo_id = Uuid::parse_str(&key[0..36])
        .map_err(|_| PromoStoreError::InvalidKey("Invalid promo_id format".to_string()))?;
    let store_id = Uuid::parse_str(&key[37..73])
        .map_err(|_| PromoStoreError::InvalidKey("Invalid store_id format".to_string()))?;

    if let Err(_) = han_get_promo_store_by_key(State(state.clone()), Path(key.clone())).await {
        return Err(PromoStoreError::NotFound(format!(
            "PromoStore with promo_id '{}' and store_id '{}' not found",
            promo_id, store_id
        )).into());
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
    if key.len() != 73 {
        return Err(PromoStoreError::InvalidKey("Invalid key format. Expected: {uuid}-{uuid}".to_string()).into());
    }

    let promo_id = Uuid::parse_str(&key[0..36])
        .map_err(|_| PromoStoreError::InvalidKey("Invalid promo_id format".to_string()))?;
    let store_id = Uuid::parse_str(&key[37..73])
        .map_err(|_| PromoStoreError::InvalidKey("Invalid store_id format".to_string()))?;

    if let Err(_) = han_get_promo_store_by_key(State(state.clone()), Path(key)).await {
        return Err(PromoStoreError::NotFound(format!(
            "PromoStore with promo_id '{}' and store_id '{}' not found",
            promo_id, store_id
        )).into());
    }

    state
        .promo_store_service
        .ser_delete_promo_store(promo_id, store_id)
        .await?;
    Ok(Json(()))
}
