#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_mut,
    unused_assignments
)]
use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{delete, get, post, put},
};
use std::sync::Arc;
use tracing::info; // <-- PERBAIKAN 1

use crate::app_state::AppState;
use handlers::promo_handler::{
    han_create_promo, han_delete_promo, han_get_all_promos, han_get_promo_by_voucher,
    han_update_promo,
};
use handlers::promo_store_handler::{
    han_create_promo_store, han_delete_promo_store, han_get_promo_store_by_id,
    han_get_promo_stores, han_update_promo_store,
};
use handlers::store_handler::{
    han_create_store, han_delete_store, han_get_store_by_route, han_get_stores, han_update_store,
};
use handlers::health_handler::{health_check, metrics};
use middleware::auth;
use repositories::cache_repository::CacheRepository;
use repositories::promo_repository::PromoRepository;
use repositories::promo_store_repository::PromoStoreRepository;
use repositories::store_repository::StoreRepository;
use services::promo_service::PromoService;
use services::promo_store_service::PromoStoreService;
use services::store_service::StoreService;
use supabase::SupabaseClient;

mod app_state;

mod error;
mod handlers;
mod middleware;
mod model;
mod repositories;
mod services;
mod startup;
mod supabase;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

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

    info!("Repositories initialized successfully.");

    startup::init_cache(
        Arc::clone(&promo_repo),
        Arc::clone(&store_repo),
        Arc::clone(&promo_store_repo),
    )
    .await;

    // Tidak perlu Arc di sini karena service akan dipindahkan ke dalam AppState
    let promo_service = PromoService::new(promo_repo);
    let store_service = StoreService::new(store_repo);
    let promo_store_service = PromoStoreService::new(promo_store_repo);

    let state = Arc::new(AppState {
        cache_repository,
        promo_service,
        store_service,
        promo_store_service,
    });

    let promo_route = Router::new()
        .route("/get-promo", get(han_get_all_promos))
        .route("/get-promo/{voucher}", get(han_get_promo_by_voucher))
        .route("/create-promo", post(han_create_promo))
        .route("/update-promo/{voucher}", put(han_update_promo))
        .route("/delete-promo/{voucher}", delete(han_delete_promo));

    let store_route = Router::new()
        .route("/get-store", get(han_get_stores))
        .route("/get-store/{route}", get(han_get_store_by_route))
        .route("/create-store", post(han_create_store))
        .route("/update-store/{route}", put(han_update_store))
        .route("/delete-store/{route}", delete(han_delete_store));

    let promo_store_route = Router::new()
        .route("/get-promo-store", get(han_get_promo_stores))
        .route(
            "/get-promo-store/{promo_store_id}",
            get(han_get_promo_store_by_id),
        )
        .route("/create-promo-store", post(han_create_promo_store))
        .route(
            "/update-promo-store/{promo_store_id}",
            put(han_update_promo_store),
        )
        .route(
            "/delete-promo-store/{promo_store_id}",
            delete(han_delete_promo_store),
        );

    lazy_limit::init_rate_limiter!(
        default: lazy_limit::RuleConfig::new(lazy_limit::Duration::seconds(1), 100)
    ).await;

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(metrics))
        .merge(promo_route)
        .merge(store_route)
        .merge(promo_store_route)
        .route_layer(from_fn_with_state(state.clone(), auth))
        .layer(tower_http::compression::CompressionLayer::new())
        .layer(tower_http::limit::RequestBodyLimitLayer::new(1024 * 1024))
        .layer(real::RealIpLayer::default())
        .layer(axum_governor::GovernorLayer::default())
        .with_state(state);

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
