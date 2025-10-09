#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_mut,
    unused_assignments
)]
pub mod error;
pub mod supabase_client;

pub use error::SupabaseError;
pub use supabase_client::{QueryBuilder, SupabaseClient};
