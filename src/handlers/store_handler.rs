use crate::app_state::AppState;
use crate::error::AppError;
use crate::repositories::store_repository::Store;
use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;

pub async fn han_get_stores(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Store>>, AppError> {
    // Contoh error: Unauthorized
    // if !authorized { return Err(AppError::Unauthorized); }

    // Contoh error: NotFound
    let stores = state.store_service.ser_get_all_stores().await?;
    if stores.is_empty() {
        return Err(AppError::NotFound("No stores found".to_string()));
    }

    // Contoh error: BadRequest
    // if false { return Err(AppError::BadRequest("Invalid query".to_string())); }

    // Contoh error: Internal
    // if false { return Err(AppError::Internal("Internal error".to_string())); }

    Ok(Json(stores))
}

pub async fn han_get_store_by_route(
    State(state): State<Arc<AppState>>,
    Path(store_route): Path<String>,
) -> Result<Json<Store>, AppError> {
    // Contoh error: Unauthorized
    // if !authorized { return Err(AppError::Unauthorized); }

    // Contoh error: NotFound
    let store = state
        .store_service
        .ser_get_store_by_route(&store_route)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "Store dengan route '{}' tidak ditemukan",
                store_route
            ))
        })?;

    // Contoh error: BadRequest
    // if false { return Err(AppError::BadRequest("Invalid query".to_string())); }

    // Contoh error: Internal
    // if false { return Err(AppError::Internal("Internal error".to_string())); }

    Ok(Json(store))
}
