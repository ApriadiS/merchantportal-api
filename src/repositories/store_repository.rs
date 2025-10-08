use crate::error::AppError;
use crate::repositories::cache_repository::CacheRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use supabase_rs::SupabaseClient;
use tracing::info;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StoreType {
    ONLINE,
    OFFLINE,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Store {
    pub id: i64,
    pub name: String,
    pub company: String,
    pub address: String,
    pub route: Option<String>,
    pub store_type: Option<StoreType>,
}

#[derive(Clone)]
pub struct StoreRepository {
    pub supabase_client: Arc<SupabaseClient>,
    pub cache_repository: Arc<CacheRepository>,
}

impl StoreRepository {
    pub fn new(
        supabase_client: Arc<SupabaseClient>,
        cache_repository: Arc<CacheRepository>,
    ) -> Self {
        Self {
            supabase_client,
            cache_repository,
        }
    }

    pub async fn rep_fetch_all(&self) -> Result<Vec<Store>, AppError> {
        {
            let cache = self.cache_repository.get_store_cache_all();
            let cache_gembok = cache.read().await;
            if !cache_gembok.is_empty() {
                info!("Cache Store Ditemukan (Cache Hit)! Mengembalikan dari memori.");
                return Ok(cache_gembok.clone());
            }
        }

        info!("Cache Store Kosong (Cache Miss). Menghubungi Supabase...");

        let stores_from_db = self
            .supabase_client
            .from("store")
            .execute()
            .await
            .map_err(|e| AppError::Internal(format!("Supabase error: {}", e)))?;

        if stores_from_db.is_empty() {
            println!("WARN: Tidak ada store yang ditemukan di Supabase.");
            return Err(AppError::NotFound(
                "Tidak ada store yang ditemukan.".to_string(),
            ));
        }

        info!(
            "Berhasil mendapatkan {} store dari Supabase.",
            stores_from_db.len()
        );

        let stores: Vec<Store> = stores_from_db
            .into_iter()
            .filter_map(|item| serde_json::from_value(item).ok())
            .collect();

        {
            self.cache_repository.clear_store_cache_all().await;
            self.cache_repository
                .save_store_cache_all(stores.clone())
                .await;
        }

        Ok(stores)
    }
    pub async fn rep_fetch_by_route(&self, route: &str) -> Result<Store, AppError> {
        {
            if let Some(cached_store) = self.cache_repository.get_store_cache_by_route(route).await
            {
                info!(
                    "Cache Store Ditemukan berdasarkan route (Cache Hit)! Mengembalikan dari memori."
                );
                return Ok(cached_store);
            }
        }

        info!("Cache Store berdasarkan route Kosong (Cache Miss). Menghubungi Supabase...");

        let stores_from_db = self
            .supabase_client
            .from("store")
            .eq("route", route)
            .execute()
            .await
            .map_err(|e| AppError::Internal(format!("Supabase error: {}", e)))?;

        if stores_from_db.is_empty() {
            println!(
                "WARN: Tidak ada store yang ditemukan di Supabase untuk route: {}",
                route
            );
            return Err(AppError::NotFound(format!(
                "Tidak ada store yang ditemukan untuk route: {}.",
                route
            )));
        }

        info!(
            "Berhasil mendapatkan {} store dari Supabase untuk route: {}.",
            stores_from_db.len(),
            route
        );

        let store: Store = serde_json::from_value(stores_from_db[0].clone())
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))?;

        Ok(store)
    }
}
