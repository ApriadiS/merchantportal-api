use supabase_rs::SupabaseClient;
/// Creates a configured Supabase client instance
///
/// # Panics
/// Panics if SUPABASE_URL or SUPABASE_KEY environment variables are not set
pub fn create_server() -> SupabaseClient {
    SupabaseClient::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .expect("Failed to create Supabase client")
}
