use crate::error::AppError;
use crate::repositories::promo_repository::{Promo, PromoRepository};
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
}
