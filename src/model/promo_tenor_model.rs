use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePromoTenorPayload {
    pub promo_id: Uuid,
    pub tenor: i32,
    pub min_transaction: i32,
    pub subsidi: f64,
    pub admin: f64,
    pub discount: i64,
    pub max_discount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_code: Option<String>,
    pub free_installment: i32,
    pub is_available: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatePromoTenorPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenor: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_transaction: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsidi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_discount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_installment: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoTenor {
    pub id: Uuid,
    pub promo_id: Uuid,
    pub tenor: i32,
    pub min_transaction: i32,
    pub subsidi: f64,
    pub admin: f64,
    pub discount: i64,
    pub max_discount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_code: Option<String>,
    pub free_installment: i32,
    pub is_available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

pub type PromoTenorResponse = PromoTenor;
