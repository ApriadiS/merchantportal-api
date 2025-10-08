use crate::error::AppError;
use crate::repositories::promo_store_repository::{PromoStore, PromoStoreRepository};
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
