use crate::error::AppError;
use crate::model::promo_store_model::{
    CreatePromoStorePayload, PromoStore, UpdatePromoStorePayload,
};
use crate::repositories::promo_store_repository::PromoStoreRepository;
use serde_json::Value as JsonValue;
use std::sync::Arc;

pub struct PromoStoreService {
    repo: Arc<PromoStoreRepository>,
}

impl PromoStoreService {
    pub fn new(repo: Arc<PromoStoreRepository>) -> Self {
        Self { repo }
    }

    pub async fn ser_get_all_promo_stores(&self) -> Result<Vec<PromoStore>, AppError> {
        self.repo.rep_fetch_all().await
    }

    pub async fn ser_get_promo_store_by_id(
        &self,
        id: &u32,
    ) -> Result<Option<PromoStore>, AppError> {
        self.repo.rep_fetch_by_id(&id).await.map(Some)
    }
}

impl PromoStoreService {
    pub async fn ser_create_promo_store(
        &self,
        payload: CreatePromoStorePayload,
    ) -> Result<PromoStore, AppError> {
        let json = serde_json::to_value(payload)
            .map_err(|e| AppError::Internal(format!("Serialize error: {}", e)))?;
        let created = self.repo.rep_insert(&json).await?;
        Ok(created)
    }

    pub async fn ser_update_promo_store(
        &self,
        id: &u32,
        payload: UpdatePromoStorePayload,
    ) -> Result<PromoStore, AppError> {
        let json = serde_json::to_value(payload)
            .map_err(|e| AppError::Internal(format!("Serialize error: {}", e)))?;
        let updated = self.repo.rep_update_by_id(id, &json).await?;
        Ok(updated)
    }

    pub async fn ser_delete_promo_store(&self, id: &u32) -> Result<(), AppError> {
        self.repo.rep_delete_by_id(id).await?;
        Ok(())
    }
}
