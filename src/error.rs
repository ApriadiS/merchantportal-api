use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt;

// ============================================================================
// Domain-Specific Errors
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub enum StoreError {
    NotFound(String),
    AlreadyExists(String),
    InvalidRoute(String),
    InvalidPayload(String),
    DatabaseError(String),
}

#[derive(Debug, Clone, Serialize)]
pub enum PromoError {
    NotFound(String),
    AlreadyExists(String),
    InvalidId(String),
    InvalidPayload(String),
    DatabaseError(String),
}

#[derive(Debug, Clone, Serialize)]
pub enum PromoTenorError {
    NotFound(String),
    AlreadyExists(String),
    InvalidId(String),
    InvalidTenor(String),
    InvalidVoucher(String),
    InvalidPayload(String),
    DatabaseError(String),
}

#[derive(Debug, Clone, Serialize)]
pub enum PromoStoreError {
    NotFound(String),
    AlreadyExists(String),
    InvalidKey(String),
    InvalidPayload(String),
    DatabaseError(String),
}

// ============================================================================
// Application Error
// ============================================================================

#[derive(Debug)]
pub enum AppError {
    // Domain errors
    Store(StoreError),
    Promo(PromoError),
    PromoTenor(PromoTenorError),
    PromoStore(PromoStoreError),
    
    // Auth errors
    Unauthorized,
    InvalidToken(String),
    
    // Generic errors
    BadRequest(String),
    Internal(String),
}

// ============================================================================
// From Implementations
// ============================================================================

impl From<StoreError> for AppError {
    fn from(err: StoreError) -> Self {
        AppError::Store(err)
    }
}

impl From<PromoError> for AppError {
    fn from(err: PromoError) -> Self {
        AppError::Promo(err)
    }
}

impl From<PromoTenorError> for AppError {
    fn from(err: PromoTenorError) -> Self {
        AppError::PromoTenor(err)
    }
}

impl From<PromoStoreError> for AppError {
    fn from(err: PromoStoreError) -> Self {
        AppError::PromoStore(err)
    }
}

// ============================================================================
// Display Implementations
// ============================================================================

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::NotFound(msg) => write!(f, "Store not found: {}", msg),
            StoreError::AlreadyExists(msg) => write!(f, "Store already exists: {}", msg),
            StoreError::InvalidRoute(msg) => write!(f, "Invalid store route: {}", msg),
            StoreError::InvalidPayload(msg) => write!(f, "Invalid store payload: {}", msg),
            StoreError::DatabaseError(msg) => write!(f, "Store database error: {}", msg),
        }
    }
}

impl fmt::Display for PromoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PromoError::NotFound(msg) => write!(f, "Promo not found: {}", msg),
            PromoError::AlreadyExists(msg) => write!(f, "Promo already exists: {}", msg),
            PromoError::InvalidId(msg) => write!(f, "Invalid promo ID: {}", msg),
            PromoError::InvalidPayload(msg) => write!(f, "Invalid promo payload: {}", msg),
            PromoError::DatabaseError(msg) => write!(f, "Promo database error: {}", msg),
        }
    }
}

impl fmt::Display for PromoTenorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PromoTenorError::NotFound(msg) => write!(f, "Promo tenor not found: {}", msg),
            PromoTenorError::AlreadyExists(msg) => write!(f, "Promo tenor already exists: {}", msg),
            PromoTenorError::InvalidId(msg) => write!(f, "Invalid promo tenor ID: {}", msg),
            PromoTenorError::InvalidTenor(msg) => write!(f, "Invalid tenor value: {}", msg),
            PromoTenorError::InvalidVoucher(msg) => write!(f, "Invalid voucher code: {}", msg),
            PromoTenorError::InvalidPayload(msg) => write!(f, "Invalid promo tenor payload: {}", msg),
            PromoTenorError::DatabaseError(msg) => write!(f, "Promo tenor database error: {}", msg),
        }
    }
}

impl fmt::Display for PromoStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PromoStoreError::NotFound(msg) => write!(f, "Promo store relation not found: {}", msg),
            PromoStoreError::AlreadyExists(msg) => write!(f, "Promo store relation already exists: {}", msg),
            PromoStoreError::InvalidKey(msg) => write!(f, "Invalid promo store key: {}", msg),
            PromoStoreError::InvalidPayload(msg) => write!(f, "Invalid promo store payload: {}", msg),
            PromoStoreError::DatabaseError(msg) => write!(f, "Promo store database error: {}", msg),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Store(err) => write!(f, "{}", err),
            AppError::Promo(err) => write!(f, "{}", err),
            AppError::PromoTenor(err) => write!(f, "{}", err),
            AppError::PromoStore(err) => write!(f, "{}", err),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

// ============================================================================
// HTTP Response Implementation
// ============================================================================

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            // Store errors
            AppError::Store(StoreError::NotFound(msg)) => (StatusCode::NOT_FOUND, msg),
            AppError::Store(StoreError::AlreadyExists(msg)) => (StatusCode::CONFLICT, msg),
            AppError::Store(StoreError::InvalidRoute(msg)) => (StatusCode::BAD_REQUEST, msg),
            AppError::Store(StoreError::InvalidPayload(msg)) => (StatusCode::BAD_REQUEST, msg),
            AppError::Store(StoreError::DatabaseError(msg)) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            
            // Promo errors
            AppError::Promo(PromoError::NotFound(msg)) => (StatusCode::NOT_FOUND, msg),
            AppError::Promo(PromoError::AlreadyExists(msg)) => (StatusCode::CONFLICT, msg),
            AppError::Promo(PromoError::InvalidId(msg)) => (StatusCode::BAD_REQUEST, msg),
            AppError::Promo(PromoError::InvalidPayload(msg)) => (StatusCode::BAD_REQUEST, msg),
            AppError::Promo(PromoError::DatabaseError(msg)) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            
            // PromoTenor errors
            AppError::PromoTenor(PromoTenorError::NotFound(msg)) => (StatusCode::NOT_FOUND, msg),
            AppError::PromoTenor(PromoTenorError::AlreadyExists(msg)) => (StatusCode::CONFLICT, msg),
            AppError::PromoTenor(PromoTenorError::InvalidId(msg)) => (StatusCode::BAD_REQUEST, msg),
            AppError::PromoTenor(PromoTenorError::InvalidTenor(msg)) => (StatusCode::BAD_REQUEST, msg),
            AppError::PromoTenor(PromoTenorError::InvalidVoucher(msg)) => (StatusCode::BAD_REQUEST, msg),
            AppError::PromoTenor(PromoTenorError::InvalidPayload(msg)) => (StatusCode::BAD_REQUEST, msg),
            AppError::PromoTenor(PromoTenorError::DatabaseError(msg)) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            
            // PromoStore errors
            AppError::PromoStore(PromoStoreError::NotFound(msg)) => (StatusCode::NOT_FOUND, msg),
            AppError::PromoStore(PromoStoreError::AlreadyExists(msg)) => (StatusCode::CONFLICT, msg),
            AppError::PromoStore(PromoStoreError::InvalidKey(msg)) => (StatusCode::BAD_REQUEST, msg),
            AppError::PromoStore(PromoStoreError::InvalidPayload(msg)) => (StatusCode::BAD_REQUEST, msg),
            AppError::PromoStore(PromoStoreError::DatabaseError(msg)) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            
            // Auth errors
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::InvalidToken(msg) => (StatusCode::UNAUTHORIZED, msg),
            
            // Generic errors
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, body).into_response()
    }
}
