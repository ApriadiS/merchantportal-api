use crate::error::{AppError, PromoTenorError};
use crate::model::promo_tenor_model::*;
use crate::repositories::cache_repository::CacheRepository;
use crate::supabase::SupabaseClient;
use crate::supabase::error::SupabaseError;
use serde_json::Value;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Clone)]
pub struct PromoTenorRepository {
    pub supabase_client: Arc<SupabaseClient>,
    pub cache_repository: Arc<CacheRepository>,
}

impl PromoTenorRepository {
    pub fn new(
        supabase_client: Arc<SupabaseClient>,
        cache_repository: Arc<CacheRepository>,
    ) -> Self {
        Self {
            supabase_client,
            cache_repository,
        }
    }

    pub async fn rep_fetch_all(&self) -> Result<Vec<PromoTenor>, AppError> {
        {
            let cache = self.cache_repository.get_promo_tenor_cache_all();
            let cache_gembok = cache.read().await;
            if !cache_gembok.is_empty() {
                info!("Cache PromoTenor Ditemukan (Cache Hit)!");
                return Ok(cache_gembok.clone());
            }
        }

        info!("Cache PromoTenor Kosong (Cache Miss). Menghubungi Supabase...");

        let promo_tenors_from_db = self
            .supabase_client
            .from::<Value>("promo_tenor")
            .execute()
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::from(PromoTenorError::NotFound("No promo tenors found".to_string()))
                } else {
                    AppError::from(PromoTenorError::DatabaseError(format!("Supabase error: {}", e)))
                }
            })?;

        if promo_tenors_from_db.is_empty() {
            warn!("Tidak ada promo_tenor yang ditemukan di Supabase.");
            return Err(AppError::from(PromoTenorError::NotFound("No promo tenors found".to_string())));
        }

        let promo_tenors: Vec<PromoTenor> = promo_tenors_from_db
            .into_iter()
            .filter_map(|item| serde_json::from_value(item).ok())
            .collect();

        self.cache_repository.clear_promo_tenor_cache_all().await;
        self.cache_repository
            .save_promo_tenor_cache_all(promo_tenors.clone())
            .await;

        Ok(promo_tenors)
    }

    pub async fn rep_fetch_by_id(&self, id: Uuid) -> Result<PromoTenor, AppError> {
        let promo_tenors_from_db = self
            .supabase_client
            .from::<Value>("promo_tenor")
            .eq("id", &id.to_string())
            .execute()
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::from(PromoTenorError::NotFound(format!("PromoTenor with id '{}' not found", id)))
                } else {
                    AppError::from(PromoTenorError::DatabaseError(format!("Supabase error: {}", e)))
                }
            })?;

        if promo_tenors_from_db.is_empty() {
            return Err(AppError::from(PromoTenorError::NotFound(format!("PromoTenor with id '{}' not found", id))));
        }

        let promo_tenor: PromoTenor = serde_json::from_value(promo_tenors_from_db[0].clone())
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))?;

        Ok(promo_tenor)
    }

    pub async fn rep_fetch_by_promo_id(&self, promo_id: Uuid) -> Result<Vec<PromoTenor>, AppError> {
        let cache = self.cache_repository.get_promo_tenor_cache_all();
        let cache_data = cache.read().await;
        let filtered: Vec<PromoTenor> = cache_data.iter().filter(|pt| pt.promo_id == promo_id).cloned().collect();
        Ok(filtered)
    }

    pub async fn rep_fetch_by_tenor(&self, tenor: i32) -> Result<Vec<PromoTenor>, AppError> {
        let cache = self.cache_repository.get_promo_tenor_cache_all();
        let cache_data = cache.read().await;
        let filtered: Vec<PromoTenor> = cache_data.iter().filter(|pt| pt.tenor == tenor).cloned().collect();
        Ok(filtered)
    }

    pub async fn rep_fetch_by_voucher(&self, voucher: &str) -> Result<Vec<PromoTenor>, AppError> {
        let cache = self.cache_repository.get_promo_tenor_cache_all();
        let cache_data = cache.read().await;
        let filtered: Vec<PromoTenor> = cache_data
            .iter()
            .filter(|pt| {
                if let Some(vc) = &pt.voucher_code {
                    vc == voucher
                } else {
                    false
                }
            })
            .cloned()
            .collect();
        Ok(filtered)
    }

    pub async fn rep_insert(&self, payload: CreatePromoTenorPayload) -> Result<PromoTenor, AppError> {
        let inserted_value = self
            .supabase_client
            .from::<Value>("promo_tenor")
            .insert(&payload)
            .await
            .map_err(|e| PromoTenorError::DatabaseError(format!("Supabase insert error: {}", e)))?;

        let promo_tenor: PromoTenor = serde_json::from_value(inserted_value)
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))?;

        self.cache_repository.clear_promo_tenor_cache_all().await;
        Ok(promo_tenor)
    }

    pub async fn rep_update_by_id(
        &self,
        id: Uuid,
        payload: UpdatePromoTenorPayload,
    ) -> Result<PromoTenor, AppError> {
        let updated_vec = self
            .supabase_client
            .from::<Value>("promo_tenor")
            .eq("id", &id.to_string())
            .update(&payload)
            .await
            .map_err(|e| PromoTenorError::DatabaseError(format!("Supabase update error: {}", e)))?;

        self.cache_repository.clear_promo_tenor_cache_all().await;

        let promo_tenor_value = updated_vec
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal("Failed to update promo_tenor".to_string()))?;

        serde_json::from_value(promo_tenor_value)
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))
    }

    pub async fn rep_delete_by_id(&self, id: Uuid) -> Result<(), AppError> {
        let _deleted = self
            .supabase_client
            .from::<Value>("promo_tenor")
            .eq("id", &id.to_string())
            .delete()
            .await
            .map_err(|e| PromoTenorError::DatabaseError(format!("Supabase delete error: {}", e)))?;

        self.cache_repository.clear_promo_tenor_cache_all().await;
        Ok(())
    }

    pub async fn rep_fetch_by_store_id(&self, store_id: Uuid) -> Result<Vec<PromoTenor>, AppError> {
        let promo_store_cache = self.cache_repository.get_promo_store_cache_all();
        let promo_store_data = promo_store_cache.read().await;
        let promo_stores: Vec<_> = promo_store_data.iter().filter(|ps| ps.store_id == store_id).collect();

        let tenor_cache = self.cache_repository.get_promo_tenor_cache_all();
        let tenor_data = tenor_cache.read().await;

        let mut result = Vec::new();
        for ps in promo_stores {
            let tenors: Vec<PromoTenor> = tenor_data.iter().filter(|t| t.promo_id == ps.promo_id).cloned().collect();
            
            // Filter by is_available instead of tenor_ids
            result.extend(tenors.into_iter().filter(|t| t.is_available));
        }

        Ok(result)
    }
}
