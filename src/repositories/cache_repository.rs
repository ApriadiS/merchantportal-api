use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::model::promo_model::Promo;
use crate::model::promo_store_model::PromoStore;
use crate::model::store_model::Store;

#[derive(Clone)]
pub struct Token {
    pub token: String,
    pub expiry: chrono::DateTime<chrono::Utc>,
    // optional cached claims stored as JSON to avoid depending on middleware types
    pub claims: Option<JsonValue>,
}

#[derive(Clone)]
pub struct AuthTokenCache {
    pub token: HashMap<String, Token>,
}

impl AuthTokenCache {
    pub fn new() -> Self {
        Self {
            token: HashMap::new(),
        }
    }

    pub fn is_valid(&self, key: &str) -> bool {
        if let Some(token) = self.token.get(key) {
            chrono::Utc::now() < token.expiry
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CacheRepository {
    promo_cache_all: Arc<RwLock<Vec<Promo>>>,
    store_cache_all: Arc<RwLock<Vec<Store>>>,
    promo_store_cache_all: Arc<RwLock<Vec<PromoStore>>>,

    promo_cache_by_voucher: Arc<RwLock<HashMap<String, Promo>>>,
    store_cache_by_route: Arc<RwLock<HashMap<String, Store>>>,
    promo_store_cache_by_key: Arc<RwLock<HashMap<String, PromoStore>>>,

    auth_token_cache: Arc<RwLock<Option<AuthTokenCache>>>,
}
impl CacheRepository {
    pub fn new() -> Self {
        Self {
            promo_cache_all: Arc::new(RwLock::new(Vec::new())),
            store_cache_all: Arc::new(RwLock::new(Vec::new())),
            promo_store_cache_all: Arc::new(RwLock::new(Vec::new())),

            promo_cache_by_voucher: Arc::new(RwLock::new(HashMap::new())),
            store_cache_by_route: Arc::new(RwLock::new(HashMap::new())),
            promo_store_cache_by_key: Arc::new(RwLock::new(HashMap::new())),

            auth_token_cache: Arc::new(RwLock::new(None)),
        }
    }

    pub fn get_auth_token_cache_ref(&self) -> &Arc<RwLock<Option<AuthTokenCache>>> {
        &self.auth_token_cache
    }

    pub async fn get_auth_token_cache(&self, token: String) -> bool {
        let cache = self.auth_token_cache.read().await;
        info!("Mendapatkan cache auth token...");
        cache.as_ref().map(|c| c.is_valid(&token)).unwrap_or(false)
    }

    /// Return cached claims (JSON) if token exists and is still valid
    pub async fn get_cached_claims(&self, token: &str) -> Option<JsonValue> {
        let cache = self.auth_token_cache.read().await;
        if let Some(auth) = cache.as_ref() {
            if let Some(t) = auth.token.get(token) {
                if chrono::Utc::now() < t.expiry {
                    return t.claims.clone();
                }
            }
        }
        None
    }

    /// Save a token with expiry and optional claims into the cache
    pub async fn save_token_claims(
        &self,
        token: String,
        claims: Option<JsonValue>,
        expiry: chrono::DateTime<chrono::Utc>,
    ) {
        let mut cache = self.auth_token_cache.write().await;
        let mut auth = cache.take().unwrap_or(AuthTokenCache {
            token: HashMap::new(),
        });
        auth.token.insert(
            token.clone(),
            Token {
                token,
                expiry,
                claims,
            },
        );
        *cache = Some(auth);
        info!("Saved token claims into cache");
    }

    pub async fn save_auth_token_cache(&self, new_cache: AuthTokenCache) {
        let mut cache = self.auth_token_cache.write().await;
        *cache = Some(new_cache);
        info!("Menyimpan cache auth token...");
    }

    pub fn get_promo_cache_all(&self) -> Arc<RwLock<Vec<Promo>>> {
        info!("Mendapatkan cache promo (all)...");
        Arc::clone(&self.promo_cache_all)
    }

    pub fn get_store_cache_all(&self) -> Arc<RwLock<Vec<Store>>> {
        info!("Mendapatkan cache store (all)...");
        Arc::clone(&self.store_cache_all)
    }

    pub fn get_promo_store_cache_all(&self) -> Arc<RwLock<Vec<PromoStore>>> {
        info!("Mendapatkan cache promo_store (all)...");
        Arc::clone(&self.promo_store_cache_all)
    }

    pub async fn get_promo_cache_by_voucher(&self, voucher: &str) -> Option<Promo> {
        let cache = self.promo_cache_by_voucher.read().await;
        info!("Mendapatkan cache promo (by voucher)...");
        cache.get(voucher).cloned()
    }

    pub async fn get_store_cache_by_route(&self, route: &str) -> Option<Store> {
        let cache = self.store_cache_by_route.read().await;
        info!("Mendapatkan cache store (by route)...");
        cache.get(route).cloned()
    }

    pub async fn get_promo_store_cache_by_key(&self, promo_id: i64, store_id: i64) -> Option<PromoStore> {
        let cache = self.promo_store_cache_by_key.read().await;
        info!("Mendapatkan cache promo_store (by key)...");
        let key = format!("{}-{}", promo_id, store_id);
        cache.get(&key).cloned()
    }

    pub async fn save_promo_cache_all(&self, promos: Vec<Promo>) {
        let mut cache = self.promo_cache_all.write().await;
        *cache = promos;

        info!("Menyimpan cache promo (all)...");

        // Membagi data
        let mut cache_by_voucher = self.promo_cache_by_voucher.write().await;
        info!("Memperbarui cache promo (by voucher)...");
        cache_by_voucher.clear();
        for promo in cache.iter() {
            cache_by_voucher.insert(promo.voucher_code.clone(), promo.clone());
        }
        info!(
            "Cache promo (by voucher) diperbarui dengan {} entri.",
            cache_by_voucher.len()
        );
    }

    pub async fn save_store_cache_all(&self, stores: Vec<Store>) {
        let mut cache = self.store_cache_all.write().await;
        *cache = stores;
        info!("Menyimpan cache store (all)...");

        // Update cache by route
        let mut cache_by_route = self.store_cache_by_route.write().await;
        info!("Memperbarui cache store (by route)...");
        cache_by_route.clear();
        for store in cache.iter() {
            if let Some(route) = &store.route {
                cache_by_route.insert(route.clone(), store.clone());
            }
        }
        info!(
            "Cache store (by route) diperbarui dengan {} entri.",
            cache_by_route.len()
        );
    }

    pub async fn save_promo_store_cache_all(&self, promo_stores: Vec<PromoStore>) {
        let mut cache = self.promo_store_cache_all.write().await;
        *cache = promo_stores;
        info!("Menyimpan cache promo_store (all)...");

        // Update cache by composite key
        let mut cache_by_key = self.promo_store_cache_by_key.write().await;
        info!("Memperbarui cache promo_store (by key)...");
        cache_by_key.clear();
        for promo_store in cache.iter() {
            let key = format!("{}-{}", promo_store.promo_id, promo_store.store_id);
            cache_by_key.insert(key, promo_store.clone());
        }
        info!(
            "Cache promo_store (by key) diperbarui dengan {} entri.",
            cache_by_key.len()
        );
    }

    pub async fn clear_promo_cache_all(&self) {
        let mut cache = self.promo_cache_all.write().await;
        cache.clear();
        info!("Menghapus cache promo (all)...");
    }
    pub async fn clear_store_cache_all(&self) {
        let mut cache = self.store_cache_all.write().await;
        cache.clear();
        info!("Menghapus cache store (all)...");
    }
    pub async fn clear_promo_store_cache_all(&self) {
        let mut cache = self.promo_store_cache_all.write().await;
        cache.clear();
        info!("Menghapus cache promo_store (all)...");
    }
}
