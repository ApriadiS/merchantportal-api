use crate::error::AppError;
use crate::model::store_model::*;
use crate::model::store_model::{
    CreateStorePayload, Store, UpdateStorePayload, UpdateStorePayloadWithID,
};
use crate::repositories::cache_repository::CacheRepository;
use crate::supabase::SupabaseClient;
use crate::supabase::error::SupabaseError;
use serde_json::Value;
use std::sync::Arc;
use tracing::{info, warn};

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
            .from::<Value>("store")
            .execute()
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::NotFound("Tidak ada store yang ditemukan.".to_string())
                } else {
                    AppError::Internal(format!("Supabase error: {}", e))
                }
            })?;

        if stores_from_db.is_empty() {
            warn!("Tidak ada store yang ditemukan di Supabase.");
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
            .from::<Value>("store")
            .eq("route", route)
            .execute()
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::NotFound(format!(
                        "Tidak ada store yang ditemukan untuk route: {}.",
                        route
                    ))
                } else {
                    AppError::Internal(format!("Supabase error: {}", e))
                }
            })?;

        if stores_from_db.is_empty() {
            warn!(
                "Tidak ada store yang ditemukan di Supabase untuk route: {}",
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

    pub async fn rep_create(&self, new_store: CreateStorePayload) -> Result<Store, AppError> {
        let created_store = self
            .supabase_client
            .from::<Value>("store")
            .insert(&new_store)
            .await
            .map_err(|e: SupabaseError| {
                AppError::Internal(format!("Supabase error during create: {}", e))
            })?;

        if created_store.is_null() {
            warn!("Gagal membuat store baru di Supabase.");
            return Err(AppError::Internal("Gagal membuat store baru.".to_string()));
        }

        info!("Berhasil membuat store baru di Supabase.");

        let store: Store = serde_json::from_value(created_store.clone())
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))?;

        // Invalidate cache setelah membuat store baru
        self.cache_repository.clear_store_cache_all().await;

        Ok(store)
    }

    pub async fn rep_update(
        &self,
        route: &str,
        updated_store: UpdateStorePayload,
    ) -> Result<Store, AppError> {
        // Ambil id dari cache jika ada
        let id = {
            let cache = self.cache_repository.get_store_cache_by_route(route).await;
            if let Some(cached_store) = cache {
                cached_store.id
            } else {
                // Jika tidak ada di cache, ambil dari database
                let store = self.rep_fetch_by_route(route).await?;
                store.id
            }
        };

        // menambahkan id kedalam updated_store
        let payload_update: UpdateStorePayloadWithID = UpdateStorePayloadWithID {
            id: id as u64,
            name: updated_store.name.clone(),
            company: updated_store.company.clone(),
            address: updated_store.address.clone(),
            route: updated_store.route.clone(),
            store_type: updated_store.store_type.clone(),
        };

        let updated = self
            .supabase_client
            .from::<Value>("store")
            .eq("id", payload_update.id.to_string().as_str())
            .update(&payload_update)
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::NotFound(format!("Store dengan route '{}' tidak ditemukan.", route))
                } else {
                    AppError::Internal(format!("Supabase error during update: {}", e))
                }
            })?;

        if updated.is_empty() {
            warn!("Gagal memperbarui store di Supabase.");
            return Err(AppError::Internal("Gagal memperbarui store.".to_string()));
        }

        info!("Berhasil memperbarui store di Supabase.");

        let store: Store = serde_json::from_value(updated[0].clone())
            .map_err(|e| AppError::Internal(format!("Deserialization error: {}", e)))?;

        // Invalidate cache setelah memperbarui store
        self.cache_repository.clear_store_cache_all().await;

        Ok(store)
    }

    pub async fn rep_delete(&self, id: &str) -> Result<(), AppError> {
        // Ambil id dari cache jika ada
        let id = {
            let cache = self.cache_repository.get_store_cache_by_route(id).await;
            if let Some(cached_store) = cache {
                cached_store.id
            } else {
                // Jika tidak ada di cache, ambil dari database
                let store = self.rep_fetch_by_route(id).await?;
                store.id
            }
        };

        let deleted = self
            .supabase_client
            .from::<Value>("store")
            .eq("id", id.to_string().as_str())
            .delete()
            .await
            .map_err(|e: SupabaseError| {
                if e.is_not_found() {
                    AppError::NotFound(format!("Store dengan id '{}' tidak ditemukan.", id))
                } else {
                    AppError::Internal(format!("Supabase error during delete: {}", e))
                }
            })?;

        if deleted.is_empty() {
            warn!("Gagal menghapus store di Supabase.");
            return Err(AppError::Internal("Gagal menghapus store.".to_string()));
        }

        info!("Berhasil menghapus store di Supabase.");

        // Invalidate cache setelah menghapus store
        self.cache_repository.clear_store_cache_all().await;

        Ok(())
    }
}
