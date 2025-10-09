use crate::error::AppError;
use crate::handlers::store_handler::{CreateStorePayload, UpdateStorePayload};
use crate::repositories::store_repository::{Store, StoreRepository};
use std::sync::Arc;

pub struct StoreService {
    repo: Arc<StoreRepository>,
}

impl StoreService {
    pub fn new(repo: Arc<StoreRepository>) -> Self {
        Self { repo }
    }

    pub async fn ser_get_all_stores(&self) -> Result<Vec<Store>, AppError> {
        self.repo.rep_fetch_all().await
    }

    pub async fn ser_get_store_by_route(&self, route: &str) -> Result<Option<Store>, AppError> {
        self.repo.rep_fetch_by_route(route).await.map(Some)
    }

    pub async fn ser_create_store(&self, new_store: CreateStorePayload) -> Result<Store, AppError> {
        self.repo.rep_create(new_store).await
    }

    pub async fn ser_update_store(
        &self,
        route: &str,
        updated_store: UpdateStorePayload,
    ) -> Result<Store, AppError> {
        self.repo.rep_update(route, updated_store).await
    }

    pub async fn ser_delete_store(&self, route: &str) -> Result<(), AppError> {
        self.repo.rep_delete(route).await
    }
}
