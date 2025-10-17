use crate::error::AppError;
use crate::model::promo_model::*;
use crate::repositories::cache_repository::CacheRepository;
use crate::supabase::SupabaseClient;
use crate::supabase::error::SupabaseError;
use serde_json::Value;
use std::sync::Arc;
use tracing::{info, warn};

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

        // --- LANGKAH 2: Jika Tidak Ada, Hubungi Supabase (Cache Miss) ---
        info!("Cache Kosong (Cache Miss). Menghubungi Supabase...");

        // (Di sini seharusnya kode panggilan Supabase Anda)
        // let promos_from_db = self.db_client.from("promos")...
        // Untuk demo, kita buat data palsu seolah-olah dari Supabase:
        let promos_from_db = self
            .supabase_client
            .from::<Value>("promo")
            .execute()
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::NotFound("Tidak ada promo yang ditemukan.".to_string())
                } else {
                    AppError::Internal(format!("Supabase error: {}", e))
                }
            })?;

        if promos_from_db.is_empty() {
            warn!("Tidak ada promo yang ditemukan di Supabase.");
            return Err(AppError::NotFound(
                "Tidak ada promo yang ditemukan.".to_string(),
            ));
        }

        info!(
            "Berhasil mendapatkan {} promo dari Supabase.",
            promos_from_db.len()
        );

        // Konversi Vec<Value> ke Vec<Promo>
        let promos_from_db: Vec<Promo> = promos_from_db
            .into_iter()
            .filter_map(|item| serde_json::from_value(item).ok())
            .collect();

        {
            // --- LANGKAH 3: Simpan ke Cache ---
            self.cache_repository.clear_promo_cache_all().await;
            self.cache_repository
                .save_promo_cache_all(promos_from_db.clone())
                .await;
        }

        // --- LANGKAH 4: Kembalikan Hasil ---
        // 'cache_gembok' (gembok) akan otomatis terlepas di akhir fungsi ini
        Ok(promos_from_db)
    }
    pub async fn rep_get_by_voucher(&self, voucher: &str) -> Result<Promo, AppError> {
        info!("Mencari promo dengan voucher_code: {}", voucher);
        {
            if let Some(cached_promo) = self
                .cache_repository
                .get_promo_cache_by_voucher(voucher)
                .await
            {
                info!("Cache Promo Ditemukan (Cache Hit)! Mengembalikan dari memori.");
                return Ok(cached_promo);
            }
        }

        info!("Cache Promo Tidak Ditemukan (Cache Miss). Menghubungi Supabase...");

        let promos_from_db = self
            .supabase_client
            .from::<Value>("promo")
            .eq("voucher_code", voucher)
            .execute()
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::NotFound(format!(
                        "Promo dengan voucher_code '{}' tidak ditemukan.",
                        voucher
                    ))
                } else {
                    AppError::Internal(format!("Supabase error: {}", e))
                }
            })?;

        if promos_from_db.is_empty() {
            warn!(
                "Promo dengan voucher_code '{}' tidak ditemukan di Supabase.",
                voucher
            );
            return Err(AppError::NotFound(format!(
                "Promo dengan voucher_code '{}' tidak ditemukan.",
                voucher
            )));
        }

        info!(
            "Berhasil mendapatkan promo dengan voucher_code '{}' dari Supabase.",
            voucher
        );

        let promo: Promo = serde_json::from_value(promos_from_db[0].clone())
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))?;

        Ok(promo)
    }

    pub async fn rep_insert(&self, payload: &serde_json::Value) -> Result<Promo, AppError> {
        // Use supabase client insert helper
        let inserted: Promo = self
            .supabase_client
            .from::<Promo>("promo")
            .insert(payload)
            .await
            .map_err(|e| AppError::Internal(format!("Supabase insert error: {}", e)))?;

        // Clear cache to force refresh
        self.cache_repository.clear_promo_cache_all().await;
        Ok(inserted)
    }

    pub async fn rep_update_by_voucher(
        &self,
        voucher: &str,
        payload: &serde_json::Value,
    ) -> Result<Promo, AppError> {
        // Ambil id dari cache berdasarkan voucher jika ada
        if let Some(cached_promo) = self
            .cache_repository
            .get_promo_cache_by_voucher(voucher)
            .await
        {
            info!(
                "Cache Promo Ditemukan (Cache Hit)! Menggunakan cached id: {}",
                cached_promo.id
            );
        } else {
            info!(
                "Cache Promo Tidak Ditemukan (Cache Miss). Menghubungi Supabase untuk mendapatkan id..."
            );
            // Jika tidak ada di cache, ambil dari Supabase
            let promo = match self.rep_get_by_voucher(voucher).await {
                Ok(p) => p,
                Err(e) => {
                    return Err(AppError::NotFound(format!(
                        "Promo dengan voucher_code '{}' tidak ditemukan untuk update. Error: {}",
                        voucher, e
                    )));
                }
            };
            info!("Ditemukan promo di Supabase dengan id: {}", promo.id);
        }

        // Update berdasarkan id yang didapat
        let id = if let Some(cached_promo) = self
            .cache_repository
            .get_promo_cache_by_voucher(voucher)
            .await
        {
            cached_promo.id
        } else {
            let promo = self.rep_get_by_voucher(voucher).await?;
            promo.id
        };

        let updated_vec = self
            .supabase_client
            .from::<Promo>("promo")
            .eq("id", id.to_string().as_str())
            .update(payload)
            .await
            .map_err(|e| AppError::Internal(format!("Supabase update error: {}", e)))?;

        // Clear cache
        self.cache_repository.clear_promo_cache_all().await;

        updated_vec
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal("Failed to update promo".to_string()))
    }

    pub async fn rep_delete_by_voucher(&self, voucher: &str) -> Result<(), AppError> {
        // Get ID from cache or DB
        let id = if let Some(cached_promo) = self
            .cache_repository
            .get_promo_cache_by_voucher(voucher)
            .await
        {
            cached_promo.id
        } else {
            let promo = self.rep_get_by_voucher(voucher).await?;
            promo.id
        };

        let _deleted = self
            .supabase_client
            .from::<Promo>("promo")
            .eq("id", id.to_string().as_str())
            .delete()
            .await
            .map_err(|e| AppError::Internal(format!("Supabase delete error: {}", e)))?;

        self.cache_repository.clear_promo_cache_all().await;
        Ok(())
    }

    pub async fn rep_get_by_store_id(&self, store_id: i64) -> Result<Vec<Promo>, AppError> {
        let cache = self.cache_repository.get_promo_store_cache_all();
        let cache_data = cache.read().await;
        let promo_ids: Vec<i64> = cache_data.iter().filter(|ps| ps.store_id == store_id).map(|ps| ps.promo_id).collect();
        drop(cache_data);

        if promo_ids.is_empty() {
            return Ok(vec![]);
        }

        let promo_cache = self.cache_repository.get_promo_cache_all();
        let promo_data = promo_cache.read().await;
        let promos: Vec<Promo> = promo_data.iter().filter(|p| promo_ids.contains(&p.id)).cloned().collect();
        Ok(promos)
    }
}
