use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoStore {
    pub id: Uuid,
    pub promo_id: Uuid,
    pub store_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenor_ids: Option<Vec<Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePromoStorePayload {
    pub promo_id: Uuid,
    pub store_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenor_ids: Option<Vec<Uuid>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatePromoStorePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenor_ids: Option<Vec<Uuid>>,
}
