use crate::error::AppError;
use crate::model::promo_tenor_model::*;
use crate::repositories::promo_tenor_repository::PromoTenorRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct PromoTenorService {
    repo: Arc<PromoTenorRepository>,
}

impl PromoTenorService {
    pub fn new(repo: Arc<PromoTenorRepository>) -> Self {
        Self { repo }
    }

    pub async fn ser_get_all_promo_tenors(&self) -> Result<Vec<PromoTenor>, AppError> {
        self.repo.rep_fetch_all().await
    }

    pub async fn ser_get_promo_tenor_by_id(&self, id: Uuid) -> Result<PromoTenor, AppError> {
        self.repo.rep_fetch_by_id(id).await
    }

    pub async fn ser_get_promo_tenors_by_promo_id(&self, promo_id: Uuid) -> Result<Vec<PromoTenor>, AppError> {
        self.repo.rep_fetch_by_promo_id(promo_id).await
    }

    pub async fn ser_get_promo_tenors_by_tenor(&self, tenor: i32) -> Result<Vec<PromoTenor>, AppError> {
        self.repo.rep_fetch_by_tenor(tenor).await
    }

    pub async fn ser_get_promo_tenors_by_voucher(&self, voucher: &str) -> Result<Vec<PromoTenor>, AppError> {
        self.repo.rep_fetch_by_voucher(voucher).await
    }

    pub async fn ser_create_promo_tenor(&self, payload: CreatePromoTenorPayload) -> Result<PromoTenor, AppError> {
        self.repo.rep_insert(payload).await
    }

    pub async fn ser_update_promo_tenor(
        &self,
        id: Uuid,
        payload: UpdatePromoTenorPayload,
    ) -> Result<PromoTenor, AppError> {
        self.repo.rep_update_by_id(id, payload).await
    }

    pub async fn ser_delete_promo_tenor(&self, id: Uuid) -> Result<(), AppError> {
        self.repo.rep_delete_by_id(id).await
    }

    pub async fn ser_get_promo_tenors_by_store_id(&self, store_id: Uuid) -> Result<Vec<PromoTenor>, AppError> {
        self.repo.rep_fetch_by_store_id(store_id).await
    }
}
