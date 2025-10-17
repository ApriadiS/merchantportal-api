use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PromoValueType {
    FIX,
    PERCENT,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePromoPayload {
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub is_active: bool,
    pub voucher_code: String,
    pub min_transaction: i64,
    pub admin_fee_type: PromoValueType,
    pub admin_fee: f64,
    pub interest_rate: f64,
    pub tenor: i64,
    pub subsidi: f64,
    pub free_installment: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_type: Option<PromoValueType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discount: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatePromoPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_transaction: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_type: Option<PromoValueType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discount: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Promo {
    pub id: i64,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub is_active: bool,
    pub voucher_code: String,
    pub min_transaction: i64,
    pub admin_fee_type: PromoValueType,
    pub admin_fee: f64,
    pub interest_rate: f64,
    pub tenor: i64,
    pub subsidi: f64,
    pub free_installment: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_type: Option<PromoValueType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discount: Option<f64>,
}
