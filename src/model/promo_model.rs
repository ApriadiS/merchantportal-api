use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AdminPromoType {
    FIX,
    PERCENT,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DiscountPromoType {
    FIX,
    PERCENT,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePromoPayload {
    pub title_promo: String,
    pub admin_promo_type: AdminPromoType,
    pub interest_rate: f64,
    pub discount_type: DiscountPromoType,
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_promo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_promo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatePromoPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_promo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_promo_type: Option<AdminPromoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_type: Option<DiscountPromoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_promo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_promo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Promo {
    pub id_promo: Uuid,
    pub title_promo: String,
    pub admin_promo_type: AdminPromoType,
    pub interest_rate: f64,
    pub discount_type: DiscountPromoType,
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_promo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_promo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

pub type PromoResponse = Promo;
