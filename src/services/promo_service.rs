use crate::error::AppError;
use crate::model::promo_model::*;
use crate::repositories::promo_repository::PromoRepository;
use std::sync::Arc;
use uuid::Uuid;

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

    pub async fn ser_get_promo_by_id(&self, id_promo: Uuid) -> Result<Promo, AppError> {
        self.repo.rep_get_by_id(id_promo).await
    }

    pub async fn ser_get_promos_by_store_id(&self, store_id: Uuid) -> Result<Vec<Promo>, AppError> {
        self.repo.rep_get_by_store_id(store_id).await
    }

    pub async fn ser_create_promo(&self, payload: CreatePromoPayload) -> Result<Promo, AppError> {
        self.repo.rep_insert(payload).await
    }

    pub async fn ser_update_promo(
        &self,
        id_promo: Uuid,
        payload: UpdatePromoPayload,
    ) -> Result<Promo, AppError> {
        self.repo.rep_update_by_id(id_promo, payload).await
    }

    pub async fn ser_delete_promo(&self, id_promo: Uuid) -> Result<(), AppError> {
        self.repo.rep_delete_by_id(id_promo).await
    }
}
