use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AdminPromoType {
    FIX,
    PERCENT,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePromoPayload {
    pub title_promo: String,
    pub start_date_promo: String,
    pub end_date_promo: String,
    pub is_active: bool,
    pub voucher_code: String,
    pub min_transaction_promo: u64,
    pub admin_promo_type: AdminPromoType,
    pub admin_promo: f64,
    pub interest_rate: f64,
    pub tenor_promo: i64,
    pub subsidi_promo: f64,
    pub free_installment: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatePromoPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_promo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_promo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_promo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_transaction_promo: Option<f64>,
}

// Domain entity for Promo moved here from repositories. This is the canonical
// representation of a stored promo. Derive Serialize/Deserialize so it can be
// converted to/from JSON when talking to Supabase or HTTP layers.
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
