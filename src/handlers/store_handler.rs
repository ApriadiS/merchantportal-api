use crate::error::AppError;
use crate::repositories::store_repository::Store;
use crate::{app_state::AppState, repositories::store_repository::StoreType};
use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateStorePayload {
    pub name: String,
    pub company: String,
    pub address: String,
    pub route: String,
    pub store_type: StoreType,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateStorePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_type: Option<StoreType>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateStorePayloadWithID {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_type: Option<StoreType>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteStorePayload {
    pub route: String,
}

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

pub async fn han_create_store(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateStorePayload>,
) -> Result<Json<Store>, AppError> {
    // Check apakah store ada di dengan han_get_store_by_route
    if let Ok(_) = han_get_store_by_route(State(state.clone()), Path(payload.route.clone())).await {
        return Err(AppError::BadRequest(format!(
            "Store dengan route '{}' sudah ada",
            payload.route
        )));
    }
    // Implementasi pembuatan store
    let stores = state.store_service.ser_create_store(payload).await?;

    // Contoh respons sukses (ganti dengan hasil sebenarnya)
    Ok(Json(stores))
}

pub async fn han_update_store(
    State(state): State<Arc<AppState>>,
    Path(store_route): Path<String>,
    Json(payload): Json<UpdateStorePayload>,
) -> Result<Json<Store>, AppError> {
    // Check apakah store ada di dengan han_get_store_by_route
    if let Err(_) = han_get_store_by_route(State(state.clone()), Path(store_route.clone())).await {
        return Err(AppError::NotFound(format!(
            "Store dengan route '{}' tidak ditemukan",
            store_route
        )));
    }

    let store = state
        .store_service
        .ser_update_store(&store_route, payload)
        .await?;

    // Contoh respons sukses (ganti dengan hasil sebenarnya)
    Ok(Json(store))
}

pub async fn han_delete_store(
    State(state): State<Arc<AppState>>,
    Path(store_route): Path<DeleteStorePayload>,
) -> Result<Json<()>, AppError> {
    // Check apakah store ada di dengan han_get_store_by_route
    if let Err(_) =
        han_get_store_by_route(State(state.clone()), Path(store_route.route.clone())).await
    {
        return Err(AppError::NotFound(format!(
            "Store dengan route '{}' tidak ditemukan",
            store_route.route
        )));
    }

    let store = state
        .store_service
        .ser_delete_store(&store_route.route)
        .await?;

    // Contoh respons sukses (ganti dengan hasil sebenarnya)
    Ok(Json(()))
}
