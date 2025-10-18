use std::sync::Arc;

use crate::repositories::cache_repository::CacheRepository;
use crate::services::promo_service::PromoService;
use crate::services::promo_store_service::PromoStoreService;
use crate::services::promo_tenor_service::PromoTenorService;
use crate::services::store_service::StoreService;

pub struct AppState {
    pub cache_repository: Arc<CacheRepository>,
    pub promo_service: PromoService,
    pub promo_tenor_service: PromoTenorService,
    pub store_service: StoreService,
    pub promo_store_service: PromoStoreService,
}
