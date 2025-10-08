use crate::services::promo_service::PromoService;
use crate::services::promo_store_service::PromoStoreService;
use crate::services::store_service::StoreService;
pub struct AppState {
    pub promo_service: PromoService,
    pub store_service: StoreService,
    pub promo_store_service: PromoStoreService,
}
