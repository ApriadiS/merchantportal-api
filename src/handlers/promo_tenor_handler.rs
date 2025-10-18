use crate::app_state::AppState;
use crate::error::{AppError, PromoTenorError};
use crate::model::promo_tenor_model::*;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PromoTenorQuery {
    pub promo_id: Option<Uuid>,
    pub tenor: Option<i32>,
    pub voucher: Option<String>,
}

pub async fn han_get_all_promo_tenors(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PromoTenorQuery>,
) -> Result<Json<Vec<PromoTenorResponse>>, AppError> {
    // Filter by promo_id
    if let Some(promo_id) = query.promo_id {
        let promo_tenors = state.promo_tenor_service.ser_get_promo_tenors_by_promo_id(promo_id).await?;
        return Ok(Json(promo_tenors));
    }

    // Filter by tenor
    if let Some(tenor) = query.tenor {
        let promo_tenors = state.promo_tenor_service.ser_get_promo_tenors_by_tenor(tenor).await?;
        return Ok(Json(promo_tenors));
    }

    // Filter by voucher
    if let Some(voucher) = query.voucher {
        let promo_tenors = state.promo_tenor_service.ser_get_promo_tenors_by_voucher(&voucher).await?;
        return Ok(Json(promo_tenors));
    }

    // Get all
    let promo_tenors = state.promo_tenor_service.ser_get_all_promo_tenors().await?;
    if promo_tenors.is_empty() {
        return Err(PromoTenorError::NotFound("No promo tenor found".to_string()).into());
    }
    Ok(Json(promo_tenors))
}

pub async fn han_get_promo_tenor_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<PromoTenorResponse>, AppError> {
    info!("Handler mencari promo_tenor dengan id: {}", id);
    let promo_tenor = state.promo_tenor_service.ser_get_promo_tenor_by_id(id).await?;
    Ok(Json(promo_tenor))
}

pub async fn han_create_promo_tenor(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePromoTenorPayload>,
) -> Result<Json<PromoTenorResponse>, AppError> {
    let created = state.promo_tenor_service.ser_create_promo_tenor(payload).await?;
    Ok(Json(created))
}

pub async fn han_update_promo_tenor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePromoTenorPayload>,
) -> Result<Json<PromoTenorResponse>, AppError> {
    let updated = state.promo_tenor_service.ser_update_promo_tenor(id, payload).await?;
    Ok(Json(updated))
}

pub async fn han_delete_promo_tenor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    state.promo_tenor_service.ser_delete_promo_tenor(id).await?;
    Ok(Json(()))
}

pub async fn han_get_promo_tenors_by_store_id(
    State(state): State<Arc<AppState>>,
    Path(store_id): Path<Uuid>,
) -> Result<Json<Vec<PromoTenorResponse>>, AppError> {
    let tenors = state.promo_tenor_service.ser_get_promo_tenors_by_store_id(store_id).await?;
    Ok(Json(tenors))
}
