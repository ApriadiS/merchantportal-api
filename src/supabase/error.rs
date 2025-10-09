// src/supabase/error.rs
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SupabaseError {
    #[error("HTTP error {status}: {message}")]
    HttpError {
        status: reqwest::StatusCode,
        message: String,
        details: Option<Value>,
    },

    #[error("JSON parse error: {source}")]
    JsonError {
        #[from]
        source: serde_json::Error,
    },

    #[error("Network error: {source}")]
    NetworkError {
        #[from]
        source: reqwest::Error,
    },

    #[error("Validation error: {field} - {reason}")]
    ValidationError { field: String, reason: String },

    #[error("Authentication error: {message}")]
    AuthError { message: String },

    #[error("Table not found: {table}")]
    TableNotFound { table: String },

    #[error("Column not found: {column} in table {table}")]
    ColumnNotFound { table: String, column: String },

    #[error("Query error: {message}")]
    QueryError {
        message: String,
        details: Option<Value>,
    },

    #[error("Insert conflict: {message}")]
    InsertConflict { message: String },

    #[error("Rate limited: {message}")]
    RateLimited { message: String },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },

    #[error("No results found")]
    NotFound,

    #[error("Multiple results found when expecting single result")]
    MultipleResults,

    #[error("Configuration error: {message}")]
    ConfigError { message: String },
}

impl SupabaseError {
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            SupabaseError::ValidationError { .. }
                | SupabaseError::AuthError { .. }
                | SupabaseError::TableNotFound { .. }
                | SupabaseError::ColumnNotFound { .. }
                | SupabaseError::QueryError { .. }
                | SupabaseError::InsertConflict { .. }
                | SupabaseError::SerializationError { .. }
                | SupabaseError::NotFound
                | SupabaseError::MultipleResults
                | SupabaseError::ConfigError { .. }
        )
    }

    pub fn is_server_error(&self) -> bool {
        matches!(self, SupabaseError::HttpError { status, .. } if status.is_server_error())
    }

    pub fn is_network_error(&self) -> bool {
        matches!(self, SupabaseError::NetworkError { .. })
    }

    pub fn is_auth_error(&self) -> bool {
        matches!(self, SupabaseError::AuthError { .. })
    }

    pub fn is_not_found(&self) -> bool {
        matches!(
            self,
            SupabaseError::NotFound | SupabaseError::TableNotFound { .. }
        )
    }
}

// Result type alias untuk konsistensi
pub type SupabaseResult<T> = Result<T, SupabaseError>;

// Helper functions untuk extract error information
pub(crate) fn extract_table_name(message: &str) -> Option<String> {
    message
        .split('"')
        .nth(1)
        .and_then(|s| s.split('.').last())
        .map(|s| s.to_string())
}

pub(crate) fn extract_table_and_column(message: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = message.split('"').collect();
    if parts.len() >= 4 {
        let column = parts[1].to_string();
        let table = parts[3].split('.').last().unwrap_or("").to_string();
        if !table.is_empty() && !column.is_empty() {
            return Some((table, column));
        }
    }
    None
}
