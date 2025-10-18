#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_mut,
    unused_assignments
)]
use axum::{
    Router,
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, post, put},
    http::{Method, header},
};
use std::{sync::Arc, time::Duration};
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::app_state::AppState;
use handlers::promo_handler::{
    han_create_promo, han_delete_promo, han_get_all_promos, han_get_promo_by_id,
    han_update_promo,
};
use handlers::promo_tenor_handler::{
    han_create_promo_tenor, han_delete_promo_tenor, han_get_all_promo_tenors,
    han_get_promo_tenor_by_id, han_update_promo_tenor, han_get_promo_tenors_by_store_id,
};
use handlers::promo_store_handler::{
    han_create_promo_store, han_delete_promo_store, han_get_promo_store_by_key,
    han_get_promo_stores, han_update_promo_store,
};
use handlers::store_handler::{
    han_create_store, han_delete_store, han_get_store_by_route, han_get_stores, han_update_store,
};
use handlers::health_handler::{health_check, ready_check, metrics};
use middleware::{auth, create_cors_layer};
use repositories::cache_repository::CacheRepository;
use repositories::promo_repository::PromoRepository;
use repositories::promo_store_repository::PromoStoreRepository;
use repositories::promo_tenor_repository::PromoTenorRepository;
use repositories::store_repository::StoreRepository;
use services::promo_service::PromoService;
use services::promo_store_service::PromoStoreService;
use services::promo_tenor_service::PromoTenorService;
use services::store_service::StoreService;
use supabase::SupabaseClient;

mod app_state;
mod error;
mod handlers;
mod middleware;
mod model;
mod rate_limiter;
mod repositories;
mod services;
mod startup;
mod supabase;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    dotenvy::dotenv().ok();
    
    let log_format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "pretty".to_string());
    
    if log_format == "json" {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    }

    let url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let api_key = std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");

    let supabase_client = Arc::new(SupabaseClient::new(&url, &api_key));
    info!("Supabase client created successfully."); // <-- PERBAIKAN 2
    let cache_repository = Arc::new(CacheRepository::new());

    let promo_repo = Arc::new(PromoRepository::new(
        Arc::clone(&supabase_client),
        Arc::clone(&cache_repository),
    ));
    let store_repo = Arc::new(StoreRepository::new(
        Arc::clone(&supabase_client),
        Arc::clone(&cache_repository),
    ));
    let promo_store_repo = Arc::new(PromoStoreRepository::new(
        Arc::clone(&supabase_client),
        Arc::clone(&cache_repository),
    ));
    let promo_tenor_repo = Arc::new(PromoTenorRepository::new(
        Arc::clone(&supabase_client),
        Arc::clone(&cache_repository),
    ));

    info!("Repositories initialized successfully.");

    startup::init_cache(
        Arc::clone(&promo_repo),
        Arc::clone(&store_repo),
        Arc::clone(&promo_store_repo),
        Arc::clone(&promo_tenor_repo),
    )
    .await;

    // Tidak perlu Arc di sini karena service akan dipindahkan ke dalam AppState
    let promo_service = PromoService::new(promo_repo);
    let promo_tenor_service = PromoTenorService::new(promo_tenor_repo);
    let store_service = StoreService::new(store_repo);
    let promo_store_service = PromoStoreService::new(promo_store_repo);

    let state = Arc::new(AppState {
        cache_repository,
        promo_service,
        promo_tenor_service,
        store_service,
        promo_store_service,
    });

    // Public promo routes
    let public_promo = Router::new()
        .route("/get-promo", get(han_get_all_promos));

    // Protected promo routes
    let protected_promo = Router::new()
        .route("/get-promo/{id_promo}", get(han_get_promo_by_id))
        .route("/create-promo", post(han_create_promo))
        .route("/update-promo/{id_promo}", put(han_update_promo))
        .route("/delete-promo/{id_promo}", delete(han_delete_promo));

    // Public promo_tenor routes
    let public_promo_tenor = Router::new()
        .route("/get-promo-tenor", get(han_get_all_promo_tenors))
        .route("/get-promo-tenor-by-store/{store_id}", get(han_get_promo_tenors_by_store_id));

    // Protected promo_tenor routes
    let protected_promo_tenor = Router::new()
        .route("/get-promo-tenor/{id}", get(han_get_promo_tenor_by_id))
        .route("/create-promo-tenor", post(han_create_promo_tenor))
        .route("/update-promo-tenor/{id}", put(han_update_promo_tenor))
        .route("/delete-promo-tenor/{id}", delete(han_delete_promo_tenor));

    // Public store routes
    let public_store = Router::new()
        .route("/get-store", get(han_get_stores))
        .route("/get-store/{route}", get(han_get_store_by_route));

    // Protected store routes
    let protected_store = Router::new()
        .route("/create-store", post(han_create_store))
        .route("/update-store/{route}", put(han_update_store))
        .route("/delete-store/{route}", delete(han_delete_store));

    // Protected promo_store routes (all protected)
    let protected_promo_store = Router::new()
        .route("/get-promo-store", get(han_get_promo_stores))
        .route("/get-promo-store/{key}", get(han_get_promo_store_by_key))
        .route("/create-promo-store", post(han_create_promo_store))
        .route("/update-promo-store/{key}", put(han_update_promo_store))
        .route("/delete-promo-store/{key}", delete(han_delete_promo_store));

    // Merge all protected routes
    let protected_routes = Router::new()
        .merge(protected_promo)
        .merge(protected_promo_tenor)
        .merge(protected_store)
        .merge(protected_promo_store)
        .route_layer(from_fn_with_state(state.clone(), auth));

    let rate_limit_requests = std::env::var("RATE_LIMIT_REQUESTS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(50);
    
    let rate_limit_window = std::env::var("RATE_LIMIT_WINDOW_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);

    let rate_limiter = rate_limiter::RateLimiter::new(rate_limit_requests, Duration::from_secs(rate_limit_window));
    info!("Rate limiter configured: {} requests per {} seconds", rate_limit_requests, rate_limit_window);

    let cors = create_cors_layer();
    info!("CORS configured with whitelist from environment");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(ready_check))
        .route("/metrics", get(metrics))
        .merge(public_promo)
        .merge(public_promo_tenor)
        .merge(public_store)
        .merge(protected_routes)
        .layer(cors)
        .layer(tower_http::compression::CompressionLayer::new())
        .layer(tower_http::limit::RequestBodyLimitLayer::new(1024 * 1024))
        .layer(from_fn(move |req, next| {
            let limiter = rate_limiter.clone();
            async move { limiter.middleware(req, next).await }
        }))
        .layer(from_fn(middleware::request_logging))
        .with_state(state);

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
