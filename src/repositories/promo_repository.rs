use crate::error::AppError;
use crate::repositories::cache_repository::CacheRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use supabase_rs::SupabaseClient;
use tracing::{info, warn};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AdminPromoType {
    FIX,
    PERCENT,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Promo {
    pub id_promo: i64,
    pub title_promo: String,
    pub start_date_promo: String,
    pub end_date_promo: String,
    pub is_active: bool,
    pub voucher_code: String,
    pub min_transaction_promo: f64,
    pub admin_promo_type: AdminPromoType,
    pub admin_promo: f64,
    pub interest_rate: f64,
    pub tenor_promo: i64,
    pub subsidi_promo: f64,
    pub free_installment: i64,
}

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
            .from("promo")
            .execute()
            .await
            .map_err(|e| AppError::Internal(format!("Supabase error: {}", e)))?;

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
            .from("promo")
            .eq("voucher_code", voucher)
            .execute()
            .await
            .map_err(|e| AppError::Internal(format!("Supabase error: {}", e)))?;

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
}
