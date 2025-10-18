use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateStorePayload {
    pub name: String,
    pub company: String,
    pub address: Option<String>,
    pub route: String,
    pub store_type: StoreType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateStorePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_type: Option<StoreType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StoreType {
    KA,
    NKA,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Store {
    pub id: Uuid,
    pub name: String,
    pub company: String,
    pub address: Option<String>,
    pub route: Option<String>,
    pub store_type: Option<StoreType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
