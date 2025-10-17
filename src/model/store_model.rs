use crate::repositories::cache_repository::CacheRepository;
use crate::supabase::SupabaseClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateStorePayload {
    pub name: String,
    pub company: String,
    pub address: String,
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
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateStorePayloadWithID {
    pub id: u64,
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
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteStorePayload {
    pub route: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StoreType {
    KA,
    NKA,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Store {
    pub id: u64,
    pub name: String,
    pub company: String,
    pub address: String,
    pub route: Option<String>,
    pub store_type: Option<StoreType>,
}

#[derive(Clone)]
pub struct StoreRepository;
