use axum::{Router, middleware::from_fn_with_state, routing::get};
use std::sync::Arc;
use tracing::info; // <-- PERBAIKAN 1

use crate::app_state::AppState;
use handlers::promo_handler::{han_get_all_promos, han_get_promo_by_voucher};
use handlers::promo_store_handler::{han_get_promo_store_by_id, han_get_promo_stores};
use handlers::store_handler::{han_get_store_by_route, han_get_stores};
use middleware::auth;
use repositories::cache_repository::CacheRepository;
use repositories::promo_repository::PromoRepository;
use repositories::promo_store_repository::PromoStoreRepository;
use repositories::store_repository::StoreRepository;
use services::promo_service::PromoService;
use services::promo_store_service::PromoStoreService;
use services::store_service::StoreService;

mod app_state;

mod error;
mod handlers;
mod middleware;
mod repositories;
mod services;
mod startup;
mod supabase;

use crate::supabase::server::create_server;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let supabase_client = Arc::new(create_server());
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
        promo_service,
        store_service,
        promo_store_service,
    });

    let app = Router::new()
        .route("/get-promo", get(han_get_all_promos))
        .route("/get-store", get(han_get_stores))
        .route("/get-promo-store", get(han_get_promo_stores))
        .route("/get-store/{route}", get(han_get_store_by_route))
        .route("/get-promo/{voucher}", get(han_get_promo_by_voucher))
        .route(
            "/get-promo-store/{promo_store_id}",
            get(han_get_promo_store_by_id),
        )
        .route_layer(from_fn_with_state(state.clone(), auth))
        .with_state(state);

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
