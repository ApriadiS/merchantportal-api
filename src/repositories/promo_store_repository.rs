use crate::error::{AppError, PromoStoreError};
use crate::model::promo_store_model::*;
use crate::repositories::cache_repository::CacheRepository;
use crate::supabase::SupabaseClient;
use crate::supabase::error::SupabaseError;
use serde_json::Value;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

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
                    AppError::from(PromoStoreError::NotFound("No promo store relations found".to_string()))
                } else {
                    AppError::from(PromoStoreError::DatabaseError(format!("Supabase error: {}", e)))
                }
            })?;

        if promo_stores_from_db.is_empty() {
            warn!("Tidak ada promo_store yang ditemukan di Supabase.");
            return Err(AppError::from(PromoStoreError::NotFound("No promo store relations found".to_string())));
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
    pub async fn rep_fetch_by_key(&self, promo_id: Uuid, store_id: Uuid) -> Result<PromoStore, AppError> {
        if let Some(cached) = self.cache_repository.get_promo_store_cache_by_key(promo_id, store_id).await {
            info!("Cache PromoStore Ditemukan (Cache Hit)! Mengembalikan dari memori.");
            return Ok(cached);
        }

        info!("Cache PromoStore Kosong (Cache Miss). Menghubungi Supabase...");

        let promos_from_db = self
            .supabase_client
            .from::<Value>("promo_store")
            .eq("promo_id", &promo_id.to_string())
            .eq("store_id", &store_id.to_string())
            .execute()
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::from(PromoStoreError::NotFound(format!(
                        "PromoStore with promo_id '{}' and store_id '{}' not found",
                        promo_id, store_id
                    )))
                } else {
                    AppError::from(PromoStoreError::DatabaseError(format!("Supabase error: {}", e)))
                }
            })?;

        if promos_from_db.is_empty() {
            warn!("Tidak ada promo_store yang ditemukan di Supabase untuk promo_id {} dan store_id {}.", promo_id, store_id);
            return Err(AppError::from(PromoStoreError::NotFound(format!(
                "PromoStore with promo_id '{}' and store_id '{}' not found",
                promo_id, store_id
            ))));
        }

        info!(
            "Berhasil mendapatkan promo_store dengan promo_id {} dan store_id {} dari Supabase.",
            promo_id, store_id
        );

        let promo: PromoStore = serde_json::from_value(promos_from_db[0].clone())
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))?;

        Ok(promo)
    }

    pub async fn rep_insert(&self, payload: CreatePromoStorePayload) -> Result<PromoStore, AppError> {
        let inserted_value = self
            .supabase_client
            .from::<Value>("promo_store")
            .insert(&payload)
            .await
            .map_err(|e| PromoStoreError::DatabaseError(format!("Supabase insert error: {}", e)))?;

        let promo_store: PromoStore = serde_json::from_value(inserted_value)
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))?;

        self.cache_repository.clear_promo_store_cache_all().await;
        self.cache_repository.clear_promo_tenor_cache_all().await;
        Ok(promo_store)
    }

    pub async fn rep_update_by_key(
        &self,
        promo_id: Uuid,
        store_id: Uuid,
        payload: UpdatePromoStorePayload,
    ) -> Result<PromoStore, AppError> {
        // Get promo_store to find id
        let promo_store = self.rep_fetch_by_key(promo_id, store_id).await?;

        let updated_vec = self
            .supabase_client
            .from::<Value>("promo_store")
            .eq("id", &promo_store.id.to_string())
            .update(&payload)
            .await
            .map_err(|e| PromoStoreError::DatabaseError(format!("Supabase update error: {}", e)))?;

        self.cache_repository.clear_promo_store_cache_all().await;
        self.cache_repository.clear_promo_tenor_cache_all().await;

        let promo_store_value = updated_vec
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal("Failed to update promo_store".to_string()))?;

        serde_json::from_value(promo_store_value)
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))
    }

    pub async fn rep_delete_by_key(&self, promo_id: Uuid, store_id: Uuid) -> Result<(), AppError> {
        // Get promo_store to find id
        let promo_store = self.rep_fetch_by_key(promo_id, store_id).await?;

        let _deleted = self
            .supabase_client
            .from::<Value>("promo_store")
            .eq("id", &promo_store.id.to_string())
            .delete()
            .await
            .map_err(|e| PromoStoreError::DatabaseError(format!("Supabase delete error: {}", e)))?;

        self.cache_repository.clear_promo_store_cache_all().await;
        self.cache_repository.clear_promo_tenor_cache_all().await;
        Ok(())
    }

    pub async fn rep_fetch_by_promo_id(&self, promo_id: Uuid) -> Result<Vec<PromoStore>, AppError> {
        let cache = self.cache_repository.get_promo_store_cache_all();
        let cache_data = cache.read().await;
        let filtered: Vec<PromoStore> = cache_data.iter().filter(|ps| ps.promo_id == promo_id).cloned().collect();
        Ok(filtered)
    }

    pub async fn rep_fetch_by_store_id(&self, store_id: Uuid) -> Result<Vec<PromoStore>, AppError> {
        let cache = self.cache_repository.get_promo_store_cache_all();
        let cache_data = cache.read().await;
        let filtered: Vec<PromoStore> = cache_data.iter().filter(|ps| ps.store_id == store_id).cloned().collect();
        Ok(filtered)
    }
}
