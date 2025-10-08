use crate::app_state::AppState;
use crate::error::AppError;
use crate::repositories::promo_repository::Promo;
use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;
use tracing::info;

pub async fn han_get_all_promos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Promo>>, AppError> {
    // Contoh error: Unauthorized
    // if !authorized { return Err(AppError::Unauthorized); }

    // Contoh error: NotFound
    let promos = state.promo_service.ser_get_all_promos().await?;
    if promos.is_empty() {
        return Err(AppError::NotFound("No promos found".to_string()));
    }

    // Contoh error: BadRequest
    // if false { return Err(AppError::BadRequest("Invalid query".to_string())); }

    // Contoh error: Internal
    // if false { return Err(AppError::Internal("Internal error".to_string())); }

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
