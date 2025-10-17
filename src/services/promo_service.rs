use crate::error::AppError;
use crate::model::promo_model::Promo;
use crate::model::promo_model::*;
use crate::repositories::promo_repository::PromoRepository;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

pub struct PromoService {
    repo: Arc<PromoRepository>,
}

impl PromoService {
    pub fn new(repo: Arc<PromoRepository>) -> Self {
        Self { repo }
    }

    pub async fn ser_get_all_promos(&self) -> Result<Vec<Promo>, AppError> {
        self.repo.rep_fetch_all().await
    }

    pub async fn ser_get_promo_by_voucher(&self, voucher: &str) -> Result<Option<Promo>, AppError> {
        self.repo.rep_get_by_voucher(voucher).await.map(Some)
    }

    pub async fn ser_get_promos_by_store_id(&self, store_id: i64) -> Result<Vec<Promo>, AppError> {
        self.repo.rep_get_by_store_id(store_id).await
    }
}

impl PromoService {
    pub async fn ser_create_promo(&self, payload: CreatePromoPayload) -> Result<Promo, AppError> {
        let json = serde_json::to_value(payload)
            .map_err(|e| AppError::Internal(format!("Serialize error: {}", e)))?;
        let created = self.repo.rep_insert(&json).await?;
        Ok(created)
    }

    pub async fn ser_update_promo(
        &self,
        voucher: &str,
        payload: UpdatePromoPayload,
    ) -> Result<Promo, AppError> {
        let json = serde_json::to_value(payload)
            .map_err(|e| AppError::Internal(format!("Serialize error: {}", e)))?;
        let updated = self.repo.rep_update_by_voucher(voucher, &json).await?;
        Ok(updated)
    }

    pub async fn ser_delete_promo(&self, voucher: &str) -> Result<(), AppError> {
        self.repo.rep_delete_by_voucher(voucher).await?;
        Ok(())
    }
}
