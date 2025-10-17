use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoStore {
    pub promo_id: i64,
    pub store_id: i64,
}

// Payload types for creating/updating PromoStore. These were previously in
// the service layer; moving them here keeps all data shapes together.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePromoStorePayload {
    pub promo_id: i64,
    pub store_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatePromoStorePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<i64>,
}
