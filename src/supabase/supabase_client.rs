// src/supabase/client.rs
use crate::supabase::error::{
    SupabaseError, SupabaseResult, extract_table_and_column, extract_table_name,
};
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use tracing::{debug, error, info, warn};
use urlencoding::encode;

#[derive(Clone, Debug)]
pub struct SupabaseClient {
    pub base_url: String,
    pub api_key: String,
    pub client: Client,
}

#[derive(Clone, Debug)]
pub struct QueryBuilder<'a, T> {
    client: &'a SupabaseClient,
    table: String,
    filters: Vec<String>,
    select_columns: Option<String>,
    limit_value: Option<usize>,
    order_by: Option<(String, bool)>, // (column, ascending)
    _phantom: std::marker::PhantomData<T>,
}

impl SupabaseClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        let base_url_str = base_url.into();
        let api_key_str = api_key.into();

        // Validasi input
        if base_url_str.trim().is_empty() {
            panic!("Base URL cannot be empty");
        }
        if api_key_str.trim().is_empty() {
            panic!("API key cannot be empty");
        }

        debug!("Creating SupabaseClient with base_url: {}", base_url_str);

        let base_url = if base_url_str.ends_with("/rest/v1") {
            base_url_str
        } else {
            format!("{}/rest/v1", base_url_str.trim_end_matches('/'))
        };

        debug!("Final base_url: {}", base_url);

        SupabaseClient {
            base_url,
            api_key: api_key_str,
            client: Client::new(),
        }
    }

    /// Memulai query builder untuk tabel tertentu dengan type safety
    pub fn from<T: DeserializeOwned>(&self, table: &str) -> QueryBuilder<'_, T> {
        QueryBuilder {
            client: self,
            table: table.to_string(),
            filters: Vec::new(),
            select_columns: None,
            limit_value: None,
            order_by: None,
            _phantom: std::marker::PhantomData,
        }
    }

    // Ubah method check_auth untuk return Result<()>
    pub async fn check_auth(&self) -> SupabaseResult<()> {
        match self.get("").await {
            Ok(_) => {
                info!("✅ Authentication successful");
                Ok(())
            }
            Err(SupabaseError::AuthError { message }) => {
                error!("❌ Authentication failed: {}", message);
                Err(SupabaseError::AuthError { message })
            }
            Err(e) => {
                error!("❌ Error during auth check: {}", e);
                Err(e)
            }
        }
    }

    // Health check juga bisa diubah menjadi Result<()>
    pub async fn health_check(&self) -> SupabaseResult<()> {
        match self.get("").await {
            Ok(_) => Ok(()),
            Err(SupabaseError::AuthError { .. }) => {
                // Server merespon tapi dengan auth error - masih terhubung
                warn!("Connected to Supabase but authentication failed");
                Ok(())
            }
            Err(SupabaseError::TableNotFound { .. }) => {
                // Server merespon dengan table not found - masih terhubung
                info!("Connected to Supabase successfully");
                Ok(())
            }
            Err(SupabaseError::HttpError { status, .. }) if status.is_client_error() => {
                // Client error (4xx) berarti server merespon
                info!("Connected to Supabase (server responded with client error)");
                Ok(())
            }
            Err(e) => {
                // Network error atau server error (5xx) - koneksi gagal
                error!("Failed to connect to Supabase: {}", e);
                Err(e)
            }
        }
    }

    // Method HTTP dasar dengan better error handling
    async fn get(&self, path: &str) -> SupabaseResult<Value> {
        let url = self.build_url(path);
        debug!("GET request to: {}", url);

        let response = self
            .build_request(reqwest::Method::GET, &url)
            .send()
            .await
            .map_err(|e| SupabaseError::NetworkError { source: e })?;

        self.handle_response_json("GET", path, response).await
    }

    async fn post(&self, path: &str, payload: &Value) -> SupabaseResult<Value> {
        let url = self.build_url(path);
        debug!("POST request to: {}", url);

        let response = self
            .build_request(reqwest::Method::POST, &url)
            .json(payload)
            .send()
            .await
            .map_err(|e| SupabaseError::NetworkError { source: e })?;

        self.handle_response_json("POST", path, response).await
    }

    async fn patch(&self, path: &str, payload: &Value) -> SupabaseResult<Value> {
        let url = self.build_url(path);
        debug!("PATCH request to: {}", url);

        let response = self
            .client
            .request(reqwest::Method::PATCH, &url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(payload)
            .send()
            .await
            .map_err(|e| SupabaseError::NetworkError { source: e })?;

        self.handle_response_json("PATCH", path, response).await
    }

    async fn delete(&self, path: &str) -> SupabaseResult<Value> {
        let url = self.build_url(path);
        debug!("DELETE request to: {}", url);

        let response = self
            .build_request(reqwest::Method::DELETE, &url)
            .send()
            .await
            .map_err(|e| SupabaseError::NetworkError { source: e })?;

        self.handle_response_json("DELETE", path, response).await
    }

    // Helper methods
    fn build_url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    fn build_request(&self, method: reqwest::Method, url: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
    }

    async fn handle_response_json(
        &self,
        method: &str,
        path: &str,
        response: reqwest::Response,
    ) -> SupabaseResult<Value> {
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| SupabaseError::NetworkError { source: e })?;

        if status.is_success() {
            info!("{} {} - Success", method, path);
            serde_json::from_str(&body).map_err(|e| SupabaseError::JsonError { source: e })
        } else {
            warn!(
                "{} {} - Failed: {} - Response: {}",
                method, path, status, body
            );

            // Parse error response dari Supabase
            let error_details: Option<Value> = serde_json::from_str(&body).ok();

            match status.as_u16() {
                400 => self.handle_bad_request(&body, error_details),
                401 => Err(SupabaseError::AuthError {
                    message: "Unauthorized - check your API key".to_string(),
                }),
                403 => Err(SupabaseError::AuthError {
                    message: "Forbidden - insufficient permissions".to_string(),
                }),
                404 => self.handle_not_found(path),
                409 => Err(SupabaseError::InsertConflict {
                    message: "Insert conflict - duplicate key or constraint violation".to_string(),
                }),
                422 => self.handle_unprocessable_entity(&body, error_details),
                429 => Err(SupabaseError::RateLimited {
                    message: "Rate limit exceeded".to_string(),
                }),
                500..=599 => Err(SupabaseError::HttpError {
                    status,
                    message: "Server error".to_string(),
                    details: error_details,
                }),
                _ => Err(SupabaseError::HttpError {
                    status,
                    message: body.clone(),
                    details: error_details,
                }),
            }
        }
    }

    fn handle_bad_request(&self, body: &str, details: Option<Value>) -> SupabaseResult<Value> {
        // Parse Supabase-specific error messages
        if body.contains("relation") && body.contains("does not exist") {
            if let Some(details) = &details {
                if let Some(message) = details.get("message").and_then(|m| m.as_str()) {
                    if let Some(table) = extract_table_name(message) {
                        return Err(SupabaseError::TableNotFound { table });
                    }
                }
            }
        }

        if body.contains("column") && body.contains("does not exist") {
            if let Some(details) = &details {
                if let Some(message) = details.get("message").and_then(|m| m.as_str()) {
                    if let Some((table, column)) = extract_table_and_column(message) {
                        return Err(SupabaseError::ColumnNotFound { table, column });
                    }
                }
            }
        }

        if body.contains("failed to parse filter") {
            return Err(SupabaseError::QueryError {
                message: "Invalid filter syntax".to_string(),
                details,
            });
        }

        Err(SupabaseError::QueryError {
            message: body.to_string(),
            details,
        })
    }

    fn handle_not_found(&self, path: &str) -> SupabaseResult<Value> {
        // Extract table name from path
        let table = path.split('?').next().unwrap_or(path).to_string();
        Err(SupabaseError::TableNotFound { table })
    }

    fn handle_unprocessable_entity(
        &self,
        body: &str,
        details: Option<Value>,
    ) -> SupabaseResult<Value> {
        // Handle validation errors
        if let Some(details) = &details {
            if let Some(message) = details.get("message").and_then(|m| m.as_str()) {
                if message.contains("failed to parse filter") {
                    return Err(SupabaseError::QueryError {
                        message: "Invalid filter syntax".to_string(),
                        details: Some(details.clone()),
                    });
                }
            }
        }

        Err(SupabaseError::ValidationError {
            field: "unknown".to_string(),
            reason: body.to_string(),
        })
    }
}

impl<'a, T: DeserializeOwned> QueryBuilder<'a, T> {
    /// SELECT columns tertentu
    pub fn select(mut self, columns: &str) -> Self {
        self.select_columns = Some(columns.to_string());
        self
    }

    /// Filter: equals (eq) - untuk string
    pub fn eq(mut self, column: &str, value: &str) -> Self {
        let encoded_value = encode(value).to_string();
        self.filters
            .push(format!("{}={}.{}", column, "eq", encoded_value));
        self
    }

    /// Filter: equals (eq) - untuk boolean
    pub fn eq_bool(mut self, column: &str, value: bool) -> Self {
        let bool_str = if value { "true" } else { "false" };
        self.filters
            .push(format!("{}={}.{}", column, "eq", bool_str));
        self
    }

    /// Filter: equals (eq) - untuk angka
    pub fn eq_num(mut self, column: &str, value: i64) -> Self {
        self.filters.push(format!("{}={}.{}", column, "eq", value));
        self
    }

    /// Filter: not equals (neq)
    pub fn neq(mut self, column: &str, value: &str) -> Self {
        let encoded_value = encode(value).to_string();
        self.filters
            .push(format!("{}={}.{}", column, "neq", encoded_value));
        self
    }

    /// Filter: greater than (gt)
    pub fn gt(mut self, column: &str, value: i64) -> Self {
        self.filters.push(format!("{}={}.{}", column, "gt", value));
        self
    }

    /// Filter: greater than or equal (gte)
    pub fn gte(mut self, column: &str, value: i64) -> Self {
        self.filters.push(format!("{}={}.{}", column, "gte", value));
        self
    }

    /// Filter: less than (lt)
    pub fn lt(mut self, column: &str, value: i64) -> Self {
        self.filters.push(format!("{}={}.{}", column, "lt", value));
        self
    }

    /// Filter: less than or equal (lte)
    pub fn lte(mut self, column: &str, value: i64) -> Self {
        self.filters.push(format!("{}={}.{}", column, "lte", value));
        self
    }

    /// Filter: like (pattern matching)
    pub fn like(mut self, column: &str, pattern: &str) -> Self {
        let encoded_pattern = encode(pattern).to_string();
        self.filters
            .push(format!("{}={}.{}", column, "like", encoded_pattern));
        self
    }

    /// Filter: ilike (case-insensitive pattern matching)
    pub fn ilike(mut self, column: &str, pattern: &str) -> Self {
        let encoded_pattern = encode(pattern).to_string();
        self.filters
            .push(format!("{}={}.{}", column, "ilike", encoded_pattern));
        self
    }

    /// Filter: in (multiple values)
    pub fn r#in(mut self, column: &str, values: &[&str]) -> Self {
        let encoded_values: Vec<String> = values.iter().map(|v| encode(v).to_string()).collect();
        let values_str = encoded_values.join(",");
        self.filters.push(format!(
            "{}={}.{}",
            column,
            "in",
            format!("({})", values_str)
        ));
        self
    }

    /// Filter: is null
    pub fn is_null(mut self, column: &str) -> Self {
        self.filters.push(format!("{}={}", column, "is.null"));
        self
    }

    /// Filter: is not null
    pub fn is_not_null(mut self, column: &str) -> Self {
        self.filters.push(format!("{}={}", column, "not.is.null"));
        self
    }

    /// Limit jumlah hasil
    pub fn limit(mut self, count: usize) -> Self {
        self.limit_value = Some(count);
        self
    }

    /// Order by column
    pub fn order(mut self, column: &str, ascending: bool) -> Self {
        let encoded_column = encode(column).to_string();
        self.order_by = Some((encoded_column, ascending));
        self
    }

    /// Offset untuk pagination
    pub fn offset(mut self, count: usize) -> Self {
        self.filters.push(format!("offset={}", count));
        self
    }

    /// Eksekusi query dan dapatkan hasil sebagai Vec<T>
    pub async fn execute(self) -> SupabaseResult<Vec<T>> {
        let mut query_params = Vec::new();

        // Tambahkan select jika ada
        if let Some(columns) = self.select_columns {
            query_params.push(format!("select={}", columns));
        }

        // Tambahkan filter
        if !self.filters.is_empty() {
            query_params.extend(self.filters);
        }

        // Tambahkan limit
        if let Some(limit) = self.limit_value {
            query_params.push(format!("limit={}", limit));
        }

        // Tambahkan order by
        if let Some((column, ascending)) = self.order_by {
            let order_dir = if ascending { "asc" } else { "desc" };
            query_params.push(format!("order={}.{}", column, order_dir));
        }

        // Build path dengan query parameters yang benar
        let path = if query_params.is_empty() {
            self.table
        } else {
            format!("{}?{}", self.table, query_params.join("&"))
        };

        debug!("Final URL: {}", path);
        let response = self.client.get(&path).await?;

        // Convert JSON response ke Vec<T>
        let results: Vec<T> =
            serde_json::from_value(response).map_err(|e| SupabaseError::JsonError { source: e })?;

        Ok(results)
    }

    /// Eksekusi query dan dapatkan single result
    pub async fn execute_single(self) -> SupabaseResult<T> {
        let results = self.limit(1).execute().await?;

        match results.len() {
            0 => Err(SupabaseError::NotFound),
            1 => Ok(results.into_iter().next().unwrap()),
            _ => Err(SupabaseError::MultipleResults),
        }
    }

    /// Insert data
    pub async fn insert(self, payload: &impl Serialize) -> SupabaseResult<T> {
        let json =
            serde_json::to_value(payload).map_err(|e| SupabaseError::SerializationError {
                message: format!("Failed to serialize payload: {}", e),
            })?;

        let response = self.client.post(&self.table, &json).await?;

        // Parse response sebagai single item
        let result: Vec<T> =
            serde_json::from_value(response).map_err(|e| SupabaseError::JsonError { source: e })?;

        result.into_iter().next().ok_or(SupabaseError::NotFound)
    }

    /// Insert multiple data
    pub async fn insert_many(self, payload: &[impl Serialize]) -> SupabaseResult<Vec<T>> {
        let json =
            serde_json::to_value(payload).map_err(|e| SupabaseError::SerializationError {
                message: format!("Failed to serialize payload: {}", e),
            })?;

        let response = self.client.post(&self.table, &json).await?;

        // Parse response sebagai Vec<T>
        let results: Vec<T> =
            serde_json::from_value(response).map_err(|e| SupabaseError::JsonError { source: e })?;

        Ok(results)
    }

    /// Update data dengan filter yang sudah ditentukan
    pub async fn update(self, payload: &impl Serialize) -> SupabaseResult<Vec<T>> {
        let json =
            serde_json::to_value(payload).map_err(|e| SupabaseError::SerializationError {
                message: format!("Failed to serialize payload: {}", e),
            })?;

        let path = if self.filters.is_empty() {
            self.table
        } else {
            format!("{}?{}", self.table, self.filters.join("&"))
        };

        let response = self.client.patch(&path, &json).await?;

        // Parse response sebagai Vec<T>
        let results: Vec<T> =
            serde_json::from_value(response).map_err(|e| SupabaseError::JsonError { source: e })?;

        Ok(results)
    }

    /// Delete data dengan filter yang sudah ditentukan
    pub async fn delete(self) -> SupabaseResult<Vec<T>> {
        let path = if self.filters.is_empty() {
            self.table
        } else {
            format!("{}?{}", self.table, self.filters.join("&"))
        };

        let response = self.client.delete(&path).await?;

        // Parse response sebagai Vec<T> (deleted items)
        let results: Vec<T> =
            serde_json::from_value(response).map_err(|e| SupabaseError::JsonError { source: e })?;

        Ok(results)
    }

    /// Count records
    pub async fn count(self) -> SupabaseResult<usize> {
        let mut query_params = vec!["count=exact".to_string()];

        // Tambahkan filter
        if !self.filters.is_empty() {
            query_params.extend(self.filters);
        }

        let path = format!("{}?{}", self.table, query_params.join("&"));
        let response = self.client.get(&path).await?;

        // Extract count from response
        if let Some(count) = response
            .get(0)
            .and_then(|v| v.as_array())
            .map(|arr| arr.len())
        {
            Ok(count)
        } else {
            // Fallback: count the results manually
            let results: Vec<T> = serde_json::from_value(response)
                .map_err(|e| SupabaseError::JsonError { source: e })?;
            Ok(results.len())
        }
    }
}

// Convenience methods untuk common use cases
impl<'a, T: DeserializeOwned> QueryBuilder<'a, T> {
    /// Filter by ID
    pub fn id(self, id: &str) -> Self {
        self.eq("id", id)
    }

    /// Filter by ID (numeric)
    pub fn id_num(self, id: i64) -> Self {
        self.eq_num("id", id)
    }

    /// Filter by boolean column
    pub fn is_verified(self, verified: bool) -> Self {
        self.eq_bool("verified", verified)
    }

    /// Filter by created_at range
    pub fn created_after(self, timestamp: &str) -> Self {
        let encoded_ts = encode(timestamp).to_string();
        self.gt("created_at", timestamp.parse().unwrap_or(0))
    }

    /// Filter by created_at range
    pub fn created_before(self, timestamp: &str) -> Self {
        let encoded_ts = encode(timestamp).to_string();
        self.lt("created_at", timestamp.parse().unwrap_or(0))
    }

    /// Ascending order
    pub fn order_asc(self, column: &str) -> Self {
        self.order(column, true)
    }

    /// Descending order  
    pub fn order_desc(self, column: &str) -> Self {
        self.order(column, false)
    }

    /// Get all records
    pub async fn all(self) -> SupabaseResult<Vec<T>> {
        self.execute().await
    }

    /// Find by ID
    pub async fn find(self, id: &str) -> SupabaseResult<T> {
        self.id(id).execute_single().await
    }

    /// Find by numeric ID
    pub async fn find_num(self, id: i64) -> SupabaseResult<T> {
        self.id_num(id).execute_single().await
    }

    /// Check if any records exist matching the filters
    pub async fn exists(self) -> SupabaseResult<bool> {
        self.limit(1).count().await.map(|count| count > 0)
    }
}
