// Public endpoints that bypass JWT authentication
pub const PUBLIC_ENDPOINTS: &[&str] = &[
    "/health",
    "/ready",
    "/metrics",
    "/get-store",
    "/get-promo",
    "/get-promo-tenor",
    "/get-promo-tenor-by-store",
];
