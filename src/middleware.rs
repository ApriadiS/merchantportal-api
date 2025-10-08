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

use crate::{app_state::AppState, error::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user_id)
    pub aud: String, // Audience
    pub exp: usize,  // Expiration time
}

// SIGNATURE FUNGSI LEBIH SEDERHANA
pub async fn auth(
    State(_state): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // 1. Ekstrak token dari header secara manual
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
        .ok_or(AppError::Unauthorized)?;

    // 2. Dapatkan JWT_SECRET dari env
    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::Internal("JWT_SECRET not set".to_string()))?;

    // 3. Buat aturan validasi
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["authenticated"]);

    // 4. Decode dan validasi token
    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &validation,
    )
    .map_err(|e| {
        eprintln!("JWT Validation Error: {:?}", e);
        AppError::Unauthorized
    })?
    .claims;

    // 5. Masukkan claims ke request extensions
    request.extensions_mut().insert(Arc::new(claims));

    // 6. Lanjutkan ke handler berikutnya
    Ok(next.run(request).await)
}
