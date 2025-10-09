use axum::{
    body::Body,
    extract::State,
    http::{Request, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;

use chrono::TimeZone;

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
    let mode = std::env::var("MODE").unwrap_or_else(|_| "prod".to_string());

    if mode == "dev" {
        return Ok(next.run(request).await);
    } else {
        // 1. Ekstrak token dari header
        let token = request
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer ").map(|v| v.to_string()))
            .ok_or(AppError::Unauthorized)?;

        // 2. Cek token di cache (ambil cached claims jika ada)
        if let Some(cached) = state.cache_repository.get_cached_claims(&token).await {
            // Deserialize JSON claims to Claims type
            if let Ok(claims) = serde_json::from_value::<Claims>(cached) {
                request.extensions_mut().insert(Arc::new(claims));
                return Ok(next.run(request).await);
            } else {
                // If deserialization fails, fall through to full decode
                tracing::warn!(
                    "Cached claims exist but failed to deserialize, falling back to decode"
                );
            }
        }

        // 3. Jika cache miss -> decode dan validasi JWT
        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| AppError::Internal("JWT_SECRET not set".to_string()))?;

        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["authenticated"]);

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

        // Save claims to cache as JSON for next requests
        if let Ok(json_claims) = serde_json::to_value(&token_data) {
            // Use token expiry from claims.exp
            let expiry = chrono::Utc
                .timestamp_opt(token_data.exp as i64, 0)
                .single()
                .unwrap_or_else(|| chrono::Utc::now());
            state
                .cache_repository
                .save_token_claims(token.clone(), Some(json_claims), expiry)
                .await;
        }

        // 4. Masukkan claims ke request extensions sehingga handler dapat mengaksesnya
        request.extensions_mut().insert(Arc::new(token_data));

        // 5. Lanjutkan ke handler berikutnya
        Ok(next.run(request).await)
    }
}
