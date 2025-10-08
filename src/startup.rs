use crate::repositories::{
    promo_repository::PromoRepository, promo_store_repository::PromoStoreRepository,
    store_repository::StoreRepository,
};
use std::sync::Arc;
use tracing::info;

/// Fungsi ini akan "memanaskan" cache dengan mengambil semua data
/// dari Supabase saat aplikasi pertama kali dijalankan.
pub async fn init_cache(
    promo_repo: Arc<PromoRepository>,
    store_repo: Arc<StoreRepository>,
    promo_store_repo: Arc<PromoStoreRepository>,
) {
    info!("🚀 Memulai proses inisialisasi cache (Cache Warming)...");
    info!(
        "Check env SUPABASE_URL: {}",
        std::env::var("SUPABASE_URL").unwrap_or_else(|_| "Not Set".to_string())
    );
    info!(
        "Check env SUPABASE_KEY: {}",
        std::env::var("SUPABASE_KEY").unwrap_or_else(|_| "Not Set".to_string())
    );
    info!("Mengambil data dari Supabase untuk mengisi cache...");

    // Jalankan semua proses fetching secara bersamaan
    let (promo_result, store_result, promo_store_result) = tokio::join!(
        promo_repo.rep_fetch_all(),
        store_repo.rep_fetch_all(),
        promo_store_repo.rep_fetch_all()
    );

    // Periksa hasil dari setiap proses. Jika ada yang gagal, hentikan aplikasi.
    if let Err(e) = promo_result {
        panic!("❌ FATAL: Gagal memuat cache promo saat startup: {}", e);
    }
    if let Err(e) = store_result {
        panic!("❌ FATAL: Gagal memuat cache store saat startup: {}", e);
    }
    if let Err(e) = promo_store_result {
        panic!(
            "❌ FATAL: Gagal memuat cache promo_store saat startup: {}",
            e
        );
    }

    info!("✅ SUCCESS: Semua cache berhasil diinisialisasi dari Supabase.");
}
