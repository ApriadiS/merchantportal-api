use crate::error::{AppError, StoreError};
use crate::model::store_model::{Store, CreateStorePayload, UpdateStorePayload};
use crate::app_state::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;

pub async fn han_get_stores(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Store>>, AppError> {
    let stores = state.store_service.ser_get_all_stores().await?;
    if stores.is_empty() {
        return Err(StoreError::NotFound("No stores found".to_string()).into());
    }
    Ok(Json(stores))
}

pub async fn han_get_store_by_route(
    State(state): State<Arc<AppState>>,
    Path(store_route): Path<String>,
) -> Result<Json<Store>, AppError> {
    let store = state
        .store_service
        .ser_get_store_by_route(&store_route)
        .await?
        .ok_or_else(|| {
            StoreError::NotFound(format!("Store with route '{}' not found", store_route))
        })?;
    Ok(Json(store))
}

pub async fn han_create_store(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateStorePayload>,
) -> Result<Json<Store>, AppError> {
    if let Ok(_) = han_get_store_by_route(State(state.clone()), Path(payload.route.clone())).await {
        return Err(StoreError::AlreadyExists(format!("Store with route '{}' already exists", payload.route)).into());
    }
    let store = state.store_service.ser_create_store(payload).await?;
    Ok(Json(store))
}

pub async fn han_update_store(
    State(state): State<Arc<AppState>>,
    Path(store_route): Path<String>,
    Json(payload): Json<UpdateStorePayload>,
) -> Result<Json<Store>, AppError> {
    if let Err(_) = han_get_store_by_route(State(state.clone()), Path(store_route.clone())).await {
        return Err(StoreError::NotFound(format!("Store with route '{}' not found", store_route)).into());
    }
    let store = state.store_service.ser_update_store(&store_route, payload).await?;
    Ok(Json(store))
}

pub async fn han_delete_store(
    State(state): State<Arc<AppState>>,
    Path(store_route): Path<String>,
) -> Result<Json<()>, AppError> {
    state.store_service.ser_delete_store(&store_route).await?;
    Ok(Json(()))
}
