use crate::error::AppError;
use crate::model::promo_store_model::PromoStore;
use crate::repositories::cache_repository::CacheRepository;
use crate::supabase::SupabaseClient;
use crate::supabase::error::SupabaseError;
use serde_json::Value;
use std::sync::Arc;
use tracing::{info, warn};

#[derive(Clone)]
pub struct PromoStoreRepository {
    pub supabase_client: Arc<SupabaseClient>,
    pub cache_repository: Arc<CacheRepository>,
}

impl PromoStoreRepository {
    pub fn new(
        supabase_client: Arc<SupabaseClient>,
        cache_repository: Arc<CacheRepository>,
    ) -> Self {
        Self {
            supabase_client,
            cache_repository,
        }
    }

    pub async fn rep_fetch_all(&self) -> Result<Vec<PromoStore>, AppError> {
        {
            let cache = self.cache_repository.get_promo_store_cache_all();
            let cache_gembok = cache.read().await;
            if !cache_gembok.is_empty() {
                info!("Cache PromoStore Ditemukan (Cache Hit)! Mengembalikan dari memori.");
                return Ok(cache_gembok.clone());
            }
        }

        info!("Cache PromoStore Kosong (Cache Miss). Menghubungi Supabase...");

        let promo_stores_from_db = self
            .supabase_client
            .from::<Value>("promo_store")
            .execute()
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::NotFound("Tidak ada promo_store yang ditemukan.".to_string())
                } else {
                    AppError::Internal(format!("Supabase error: {}", e))
                }
            })?;

        if promo_stores_from_db.is_empty() {
            warn!("Tidak ada promo_store yang ditemukan di Supabase.");
            return Err(AppError::NotFound(
                "Tidak ada promo_store yang ditemukan.".to_string(),
            ));
        }

        info!(
            "Berhasil mendapatkan {} promo_store dari Supabase.",
            promo_stores_from_db.len()
        );

        let promo_stores: Vec<PromoStore> = promo_stores_from_db
            .into_iter()
            .filter_map(|item| serde_json::from_value(item).ok())
            .collect();

        {
            self.cache_repository.clear_promo_store_cache_all().await;
            self.cache_repository
                .save_promo_store_cache_all(promo_stores.clone())
                .await;
        }

        Ok(promo_stores)
    }
    pub async fn rep_fetch_by_id(&self, id: &u32) -> Result<PromoStore, AppError> {
        {
            if let Some(cached) = self.cache_repository.get_promo_store_cache_by_id(id).await {
                info!("Cache PromoStore Ditemukan (Cache Hit)! Mengembalikan dari memori.");
                return Ok(cached);
            }
        }

        info!("Cache PromoStore Kosong (Cache Miss). Menghubungi Supabase...");

        let promos_from_db = self
            .supabase_client
            .from::<Value>("promo_store")
            .eq("id", &id.to_string())
            .execute()
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::NotFound(format!(
                        "Tidak ada promo_store yang ditemukan untuk id {}.",
                        id
                    ))
                } else {
                    AppError::Internal(format!("Supabase error: {}", e))
                }
            })?;

        if promos_from_db.is_empty() {
            warn!(
                "Tidak ada promo_store yang ditemukan di Supabase untuk id {}.",
                id
            );
            return Err(AppError::NotFound(format!(
                "Tidak ada promo_store yang ditemukan untuk id {}.",
                id
            )));
        }

        info!(
            "Berhasil mendapatkan promo_store dengan id {} dari Supabase.",
            id
        );

        let promo: PromoStore = serde_json::from_value(promos_from_db[0].clone())
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))?;

        Ok(promo)
    }

    pub async fn rep_insert(&self, payload: &serde_json::Value) -> Result<PromoStore, AppError> {
        let inserted: PromoStore = self
            .supabase_client
            .from::<PromoStore>("promo_store")
            .insert(payload)
            .await
            .map_err(|e| AppError::Internal(format!("Supabase insert error: {}", e)))?;

        self.cache_repository.clear_promo_store_cache_all().await;
        Ok(inserted)
    }

    pub async fn rep_update_by_id(
        &self,
        id: &u32,
        payload: &serde_json::Value,
    ) -> Result<PromoStore, AppError> {
        let updated_vec = self
            .supabase_client
            .from::<PromoStore>("promo_store")
            .eq("id", &id.to_string())
            .update(payload)
            .await
            .map_err(|e| AppError::Internal(format!("Supabase update error: {}", e)))?;

        self.cache_repository.clear_promo_store_cache_all().await;

        updated_vec
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal("Failed to update promo_store".to_string()))
    }

    pub async fn rep_delete_by_id(&self, id: &u32) -> Result<(), AppError> {
        let _deleted = self
            .supabase_client
            .from::<PromoStore>("promo_store")
            .eq("id", &id.to_string())
            .delete()
            .await
            .map_err(|e| AppError::Internal(format!("Supabase delete error: {}", e)))?;

        self.cache_repository.clear_promo_store_cache_all().await;
        Ok(())
    }
}
