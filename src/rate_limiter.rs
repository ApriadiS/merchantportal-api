use axum::{
    body::Body,
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use dashmap::DashMap;
use sha2::{Sha256, Digest};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tracing::{info, warn};

use crate::constants::PUBLIC_ENDPOINTS;

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<DashMap<String, Vec<Instant>>>,
    max_requests: usize,
    window: Duration,
    enabled: bool,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        let enabled = std::env::var("RATE_LIMIT_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true);

        Self {
            requests: Arc::new(DashMap::new()),
            max_requests,
            window,
            enabled,
        }
    }

    fn generate_fingerprint(&self, req: &Request) -> String {
        let ip = req
            .extensions()
            .get::<std::net::SocketAddr>()
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let user_agent = req
            .headers()
            .get(header::USER_AGENT)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown");

        let accept_language = req
            .headers()
            .get(header::ACCEPT_LANGUAGE)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown");

        let fingerprint_data = format!("{}-{}-{}", ip, user_agent, accept_language);
        
        let mut hasher = Sha256::new();
        hasher.update(fingerprint_data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub async fn middleware(&self, req: Request, next: Next) -> Response {
        if !self.enabled {
            return next.run(req).await;
        }

        let path = req.uri().path();
        
        // Skip rate limiting for public endpoints only
        if PUBLIC_ENDPOINTS.iter().any(|&endpoint| path.starts_with(endpoint)) {
            return next.run(req).await;
        }

        let start = Instant::now();
        let fingerprint = self.generate_fingerprint(&req);
        let fingerprint_gen_ms = start.elapsed().as_micros() as f64 / 1000.0;

        let now = Instant::now();
        let mut entry = self.requests.entry(fingerprint.clone()).or_insert_with(Vec::new);

        entry.retain(|&time| now.duration_since(time) < self.window);

        if entry.len() >= self.max_requests {
            warn!(
                fingerprint = %fingerprint,
                requests = entry.len(),
                "Rate limit exceeded"
            );
            return (StatusCode::TOO_MANY_REQUESTS, "Too many requests").into_response();
        }

        entry.push(now);
        let request_count = entry.len();
        drop(entry);

        info!(
            fingerprint_gen_ms = fingerprint_gen_ms,
            request_count = request_count,
            "Rate limit check passed"
        );

        next.run(req).await
    }
}
