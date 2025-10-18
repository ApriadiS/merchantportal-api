use crate::error::{AppError, PromoError};
use crate::model::promo_model::*;
use crate::repositories::cache_repository::CacheRepository;
use crate::supabase::SupabaseClient;
use crate::supabase::error::SupabaseError;
use serde_json::Value;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Clone)]
pub struct PromoRepository {
    pub supabase_client: Arc<SupabaseClient>,
    pub cache_repository: Arc<CacheRepository>,
}

impl PromoRepository {
    pub fn new(
        supabase_client: Arc<SupabaseClient>,
        cache_repository: Arc<CacheRepository>,
    ) -> Self {
        Self {
            supabase_client,
            cache_repository,
        }
    }

    pub async fn rep_fetch_all(&self) -> Result<Vec<Promo>, AppError> {
        {
            let cache = self.cache_repository.get_promo_cache_all();
            let cache_gembok = cache.read().await;
            if !cache_gembok.is_empty() {
                info!("Cache Ditemukan (Cache Hit)! Mengembalikan dari memori.");
                return Ok(cache_gembok.clone());
            }
        }

        info!("Cache Kosong (Cache Miss). Menghubungi Supabase...");

        let promos_from_db = self
            .supabase_client
            .from::<Value>("promo")
            .execute()
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::from(PromoError::NotFound("No promos found".to_string()))
                } else {
                    AppError::from(PromoError::DatabaseError(format!("Supabase error: {}", e)))
                }
            })?;

        if promos_from_db.is_empty() {
            warn!("Tidak ada promo yang ditemukan di Supabase.");
            return Err(AppError::from(PromoError::NotFound("No promos found".to_string())));
        }

        info!(
            "Berhasil mendapatkan {} promo dari Supabase.",
            promos_from_db.len()
        );

        let promos_from_db: Vec<Promo> = promos_from_db
            .into_iter()
            .filter_map(|item| {
                match serde_json::from_value::<Promo>(item.clone()) {
                    Ok(promo) => Some(promo),
                    Err(e) => {
                        warn!("Failed to deserialize promo: {}. Data: {:?}", e, item);
                        None
                    }
                }
            })
            .collect();

        info!("Berhasil deserialize {} promo.", promos_from_db.len());

        if promos_from_db.is_empty() {
            warn!("Semua promo gagal di-deserialize!");
            return Err(AppError::Internal("Failed to deserialize promos".to_string()));
        }

        self.cache_repository.clear_promo_cache_all().await;
        self.cache_repository
            .save_promo_cache_all(promos_from_db.clone())
            .await;

        Ok(promos_from_db)
    }

    pub async fn rep_get_by_id(&self, id_promo: Uuid) -> Result<Promo, AppError> {
        info!("Mencari promo dengan id_promo: {}", id_promo);

        let promos_from_db = self
            .supabase_client
            .from::<Value>("promo")
            .eq("id_promo", &id_promo.to_string())
            .execute()
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::from(PromoError::NotFound(format!("Promo with id '{}' not found", id_promo)))
                } else {
                    AppError::from(PromoError::DatabaseError(format!("Supabase error: {}", e)))
                }
            })?;

        if promos_from_db.is_empty() {
            return Err(AppError::from(PromoError::NotFound(format!("Promo with id '{}' not found", id_promo))));
        }

        let promo: Promo = serde_json::from_value(promos_from_db[0].clone())
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))?;

        Ok(promo)
    }

    pub async fn rep_insert(&self, payload: CreatePromoPayload) -> Result<Promo, AppError> {
        let inserted_value = self
            .supabase_client
            .from::<Value>("promo")
            .insert(&payload)
            .await
            .map_err(|e| PromoError::DatabaseError(format!("Supabase insert error: {}", e)))?;

        let promo: Promo = serde_json::from_value(inserted_value)
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))?;

        self.cache_repository.clear_promo_cache_all().await;
        Ok(promo)
    }

    pub async fn rep_update_by_id(
        &self,
        id_promo: Uuid,
        payload: UpdatePromoPayload,
    ) -> Result<Promo, AppError> {
        let updated_vec = self
            .supabase_client
            .from::<Value>("promo")
            .eq("id_promo", &id_promo.to_string())
            .update(&payload)
            .await
            .map_err(|e| PromoError::DatabaseError(format!("Supabase update error: {}", e)))?;

        self.cache_repository.clear_promo_cache_all().await;

        let promo_value = updated_vec
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal("Failed to update promo".to_string()))?;

        serde_json::from_value(promo_value)
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))
    }

    pub async fn rep_delete_by_id(&self, id_promo: Uuid) -> Result<(), AppError> {
        let _deleted = self
            .supabase_client
            .from::<Value>("promo")
            .eq("id_promo", &id_promo.to_string())
            .delete()
            .await
            .map_err(|e| PromoError::DatabaseError(format!("Supabase delete error: {}", e)))?;

        self.cache_repository.clear_promo_cache_all().await;
        Ok(())
    }

    pub async fn rep_get_by_store_id(&self, store_id: Uuid) -> Result<Vec<Promo>, AppError> {
        let cache = self.cache_repository.get_promo_store_cache_all();
        let cache_data = cache.read().await;
        let promo_ids: Vec<Uuid> = cache_data.iter().filter(|ps| ps.store_id == store_id).map(|ps| ps.promo_id).collect();
        drop(cache_data);

        if promo_ids.is_empty() {
            return Ok(vec![]);
        }

        let promo_cache = self.cache_repository.get_promo_cache_all();
        let promo_data = promo_cache.read().await;
        let promos: Vec<Promo> = promo_data.iter().filter(|p| promo_ids.contains(&p.id_promo)).cloned().collect();
        Ok(promos)
    }
}
