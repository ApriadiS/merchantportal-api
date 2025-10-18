use crate::app_state::AppState;
use crate::error::{AppError, PromoError};
use crate::model::promo_model::*;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PromoQuery {
    pub store_id: Option<Uuid>,
}

pub async fn han_get_all_promos(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PromoQuery>,
) -> Result<Json<Vec<PromoResponse>>, AppError> {
    if let Some(store_id) = query.store_id {
        let promos = state.promo_service.ser_get_promos_by_store_id(store_id).await?;
        return Ok(Json(promos));
    }

    let promos = state.promo_service.ser_get_all_promos().await?;
    if promos.is_empty() {
        return Err(PromoError::NotFound("No promos found".to_string()).into());
    }
    Ok(Json(promos))
}

pub async fn han_get_promo_by_id(
    State(state): State<Arc<AppState>>,
    Path(id_promo): Path<Uuid>,
) -> Result<Json<PromoResponse>, AppError> {
    info!("Handler mencari promo dengan id_promo: {}", id_promo);
    let promo = state.promo_service.ser_get_promo_by_id(id_promo).await?;
    Ok(Json(promo))
}

pub async fn han_create_promo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePromoPayload>,
) -> Result<Json<PromoResponse>, AppError> {
    let created = state.promo_service.ser_create_promo(payload).await?;
    Ok(Json(created))
}

pub async fn han_update_promo(
    State(state): State<Arc<AppState>>,
    Path(id_promo): Path<Uuid>,
    Json(payload): Json<UpdatePromoPayload>,
) -> Result<Json<PromoResponse>, AppError> {
    let updated = state.promo_service.ser_update_promo(id_promo, payload).await?;
    Ok(Json(updated))
}

pub async fn han_delete_promo(
    State(state): State<Arc<AppState>>,
    Path(id_promo): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    state.promo_service.ser_delete_promo(id_promo).await?;
    Ok(Json(()))
}
