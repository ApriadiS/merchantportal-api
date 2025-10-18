use axum::{
    body::Body,
    extract::State,
    http::{Request, header, Uri, Method},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tower_http::cors::{CorsLayer, Any};
use tracing::{error, info, Span};
use chrono::TimeZone;
use uuid::Uuid;

use crate::{app_state::AppState, error::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user_id)
    pub aud: String, // Audience
    pub exp: usize,  // Expiration time
}

// Middleware authentication: cek cache dulu (scoped read-lock), jika tidak ada -> decode JWT
pub async fn auth(
    State(state): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let start = Instant::now();
    let mode = std::env::var("MODE").unwrap_or_else(|_| "prod".to_string());

    if mode == "dev" {
        return Ok(next.run(request).await);
    }

    let uri = request.uri();
    let path = uri.path();
    
    if path == "/get-promo" && uri.query().map_or(false, |q| q.contains("store_id=")) {
        return Ok(next.run(request).await);
    }
    
    if path == "/get-store" || path.starts_with("/get-store/") {
        return Ok(next.run(request).await);
    }

    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer ").map(|v| v.to_string()))
        .ok_or(AppError::Unauthorized)?;

    let cache_start = Instant::now();
    let cache_hit = if let Some(cached) = state.cache_repository.get_cached_claims(&token).await {
        if let Ok(claims) = serde_json::from_value::<Claims>(cached) {
            request.extensions_mut().insert(Arc::new(claims));
            let jwt_duration = start.elapsed().as_millis();
            info!(
                jwt_validation_ms = jwt_duration,
                cache_hit = true,
                "JWT validated from cache"
            );
            return Ok(next.run(request).await);
        }
        false
    } else {
        false
    };

    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::Internal("JWT_SECRET not set".to_string()))?;

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["authenticated"]);

    let decode_start = Instant::now();
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &validation,
    )
    .map_err(|e| {
        error!("JWT Validation Error: {:?}", e);
        AppError::Unauthorized
    })?;

    let token_data = decoded.claims;

    if let Ok(json_claims) = serde_json::to_value(&token_data) {
        let expiry = chrono::Utc
            .timestamp_opt(token_data.exp as i64, 0)
            .single()
            .unwrap_or_else(|| chrono::Utc::now());
        state
            .cache_repository
            .save_token_claims(token.clone(), Some(json_claims), expiry)
            .await;
    }

    request.extensions_mut().insert(Arc::new(token_data));

    let jwt_duration = start.elapsed().as_millis();
    info!(
        jwt_validation_ms = jwt_duration,
        cache_hit = cache_hit,
        "JWT validated"
    );

    Ok(next.run(request).await)
}

pub async fn request_logging(
    request: Request<Body>,
    next: Next,
) -> Response {
    let start = Instant::now();
    let request_id = Uuid::new_v4().to_string();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path().to_string();

    let response = next.run(request).await;
    
    let duration = start.elapsed().as_millis();
    let status = response.status().as_u16();

    info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        status = status,
        request_duration_ms = duration,
        "Request completed"
    );

    response
}

pub fn create_cors_layer() -> CorsLayer {
    let allowed_origins = std::env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    
    let origins: Vec<String> = allowed_origins
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let allowed_methods = std::env::var("CORS_ALLOWED_METHODS")
        .unwrap_or_else(|_| "GET,POST,PUT,DELETE".to_string());
    
    let methods: Vec<Method> = allowed_methods
        .split(',')
        .filter_map(|m| m.trim().parse().ok())
        .collect();

    let max_age = std::env::var("CORS_MAX_AGE")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(3600);

    if origins.is_empty() || origins.iter().any(|o| o == "*") {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(methods)
            .allow_headers(Any)
            .max_age(std::time::Duration::from_secs(max_age))
    } else {
        let origin_headers: Vec<_> = origins
            .iter()
            .filter_map(|o| o.parse().ok())
            .collect();
        
        CorsLayer::new()
            .allow_origin(origin_headers)
            .allow_methods(methods)
            .allow_headers(Any)
            .max_age(std::time::Duration::from_secs(max_age))
    }
}