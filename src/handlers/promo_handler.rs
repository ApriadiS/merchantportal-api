use crate::app_state::AppState;
use crate::error::AppError;
use crate::model::promo_model::Promo;
use crate::model::promo_model::*;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::info;

#[derive(Deserialize)]
pub struct PromoQuery {
    pub store_id: Option<i64>,
}

pub async fn han_get_all_promos(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PromoQuery>,
) -> Result<Json<Vec<Promo>>, AppError> {
    if let Some(store_id) = query.store_id {
        let promos = state.promo_service.ser_get_promos_by_store_id(store_id).await?;
        return Ok(Json(promos));
    }

    let promos = state.promo_service.ser_get_all_promos().await?;
    if promos.is_empty() {
        return Err(AppError::NotFound("No promos found".to_string()));
    }

    Ok(Json(promos))
}

pub async fn han_get_promo_by_voucher(
    State(state): State<Arc<AppState>>,
    Path(voucher): Path<String>,
) -> Result<Json<Promo>, AppError> {
    info!("Handler mencari promo dengan voucher_code: {}", voucher);
    // Contoh error: Unauthorized
    // if !authorized { return Err(AppError::Unauthorized); }

    // Contoh error: NotFound
    let promo = state
        .promo_service
        .ser_get_promo_by_voucher(&voucher)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "Promo dengan voucher_code '{}' tidak ditemukan",
                voucher
            ))
        })?;

    // Contoh error: BadRequest
    // if false { return Err(AppError::BadRequest("Invalid query".to_string())); }

    // Contoh error: Internal
    // if false { return Err(AppError::Internal("Internal error".to_string())); }

    Ok(Json(promo))
}

// Reuse payload types defined in services to avoid duplicate types

pub async fn han_create_promo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePromoPayload>,
) -> Result<Json<Promo>, AppError> {
    // Validate duplicate voucher
    if let Ok(_) =
        han_get_promo_by_voucher(State(state.clone()), Path(payload.voucher_code.clone())).await
    {
        return Err(AppError::BadRequest(format!(
            "Promo with voucher '{}' already exists",
            payload.voucher_code
        )));
    }

    let created = state.promo_service.ser_create_promo(payload).await?;
    Ok(Json(created))
}

pub async fn han_update_promo(
    State(state): State<Arc<AppState>>,
    Path(voucher): Path<String>,
    Json(payload): Json<UpdatePromoPayload>,
) -> Result<Json<Promo>, AppError> {
    // Ensure exists
    if let Err(_) = han_get_promo_by_voucher(State(state.clone()), Path(voucher.clone())).await {
        return Err(AppError::NotFound(format!(
            "Promo with voucher '{}' not found",
            voucher
        )));
    }

    let updated = state
        .promo_service
        .ser_update_promo(&voucher, payload)
        .await?;
    Ok(Json(updated))
}

pub async fn han_delete_promo(
    State(state): State<Arc<AppState>>,
    Path(voucher): Path<String>,
) -> Result<Json<()>, AppError> {
    // Ensure exists
    if let Err(_) = han_get_promo_by_voucher(State(state.clone()), Path(voucher.clone())).await {
        return Err(AppError::NotFound(format!(
            "Promo with voucher '{}' not found",
            voucher
        )));
    }

    state.promo_service.ser_delete_promo(&voucher).await?;
    Ok(Json(()))
}
