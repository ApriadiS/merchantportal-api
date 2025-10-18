use crate::error::AppError;
use crate::model::promo_store_model::{
    CreatePromoStorePayload, PromoStore, UpdatePromoStorePayload,
};
use crate::repositories::promo_store_repository::PromoStoreRepository;
use std::sync::Arc;
use uuid::Uuid;

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

    pub async fn ser_get_promo_store_by_key(
        &self,
        promo_id: Uuid,
        store_id: Uuid,
    ) -> Result<Option<PromoStore>, AppError> {
        self.repo.rep_fetch_by_key(promo_id, store_id).await.map(Some)
    }

    pub async fn ser_get_promo_stores_by_promo_id(&self, promo_id: Uuid) -> Result<Vec<PromoStore>, AppError> {
        self.repo.rep_fetch_by_promo_id(promo_id).await
    }

    pub async fn ser_get_promo_stores_by_store_id(&self, store_id: Uuid) -> Result<Vec<PromoStore>, AppError> {
        self.repo.rep_fetch_by_store_id(store_id).await
    }
}

impl PromoStoreService {
    pub async fn ser_create_promo_store(
        &self,
        payload: CreatePromoStorePayload,
    ) -> Result<PromoStore, AppError> {
        self.repo.rep_insert(payload).await
    }

    pub async fn ser_update_promo_store(
        &self,
        promo_id: Uuid,
        store_id: Uuid,
        payload: UpdatePromoStorePayload,
    ) -> Result<PromoStore, AppError> {
        self.repo.rep_update_by_key(promo_id, store_id, payload).await
    }

    pub async fn ser_delete_promo_store(&self, promo_id: Uuid, store_id: Uuid) -> Result<(), AppError> {
        self.repo.rep_delete_by_key(promo_id, store_id).await
    }
}
