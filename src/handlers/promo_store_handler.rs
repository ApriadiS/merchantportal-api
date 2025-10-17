use crate::app_state::AppState;
use crate::error::AppError;
use crate::model::promo_store_model::PromoStore;
use crate::model::promo_store_model::{CreatePromoStorePayload, UpdatePromoStorePayload};
use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;

pub async fn han_get_promo_stores(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PromoStore>>, AppError> {
    // Contoh error: Unauthorized
    // if !authorized { return Err(AppError::Unauthorized); }

    // Contoh error: NotFound
    let promo_stores = state.promo_store_service.ser_get_all_promo_stores().await?;
    if promo_stores.is_empty() {
        return Err(AppError::NotFound("No promo stores found".to_string()));
    }

    // Contoh error: BadRequest
    // if false { return Err(AppError::BadRequest("Invalid query".to_string())); }

    // Contoh error: Internal
    // if false { return Err(AppError::Internal("Internal error".to_string())); }

    Ok(Json(promo_stores))
}

pub async fn han_get_promo_store_by_id(
    State(state): State<Arc<AppState>>,
    Path(promo_store_id): Path<u32>,
) -> Result<Json<PromoStore>, AppError> {
    // Contoh error: Unauthorized
    // if !authorized { return Err(AppError::Unauthorized); }

    // Contoh error: NotFound
    let promo_store = state
        .promo_store_service
        .ser_get_promo_store_by_id(&promo_store_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "PromoStore dengan id '{}' tidak ditemukan",
                promo_store_id
            ))
        })?;

    // Contoh error: BadRequest
    // if false { return Err(AppError::BadRequest("Invalid query".to_string())); }

    // Contoh error: Internal
    // if false { return Err(AppError::Internal("Internal error".to_string())); }

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
    Path(promo_store_id): Path<u32>,
    Json(payload): Json<UpdatePromoStorePayload>,
) -> Result<Json<PromoStore>, AppError> {
    // Ensure exists
    if let Err(_) = han_get_promo_store_by_id(State(state.clone()), Path(promo_store_id)).await {
        return Err(AppError::NotFound(format!(
            "PromoStore with id '{}' not found",
            promo_store_id
        )));
    }

    let updated = state
        .promo_store_service
        .ser_update_promo_store(&promo_store_id, payload)
        .await?;
    Ok(Json(updated))
}

pub async fn han_delete_promo_store(
    State(state): State<Arc<AppState>>,
    Path(promo_store_id): Path<u32>,
) -> Result<Json<()>, AppError> {
    // Ensure exists
    if let Err(_) = han_get_promo_store_by_id(State(state.clone()), Path(promo_store_id)).await {
        return Err(AppError::NotFound(format!(
            "PromoStore with id '{}' not found",
            promo_store_id
        )));
    }

    state
        .promo_store_service
        .ser_delete_promo_store(&promo_store_id)
        .await?;
    Ok(Json(()))
}
