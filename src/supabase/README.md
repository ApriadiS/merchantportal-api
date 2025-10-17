# 🗄️ Supabase Client

Custom Supabase client implementation dengan query builder pattern.

## 📋 Files

### **supabase_client.rs**
Type-safe query builder untuk Supabase REST API.

**Features:**
- Query builder pattern
- Type-safe operations
- Connection reuse (reqwest Client)
- Comprehensive error handling

**Methods:**
```rust
// Query builder
from<T>(table: &str) -> QueryBuilder<T>

// Filters
.eq(column, value)
.neq(column, value)
.gt(column, value)
.like(column, pattern)
// ... more filters

// Operations
.execute() -> Vec<T>
.insert(payload) -> T
.update(payload) -> Vec<T>
.delete() -> Vec<T>

// Health checks
.health_check()
.check_auth()
```

### **error.rs**
Supabase-specific error types.

**Error Types:**
- `NotFound` - Record not found
- `TableNotFound` - Table doesn't exist
- `ColumnNotFound` - Column doesn't exist
- `AuthError` - Authentication failed
- `ValidationError` - Data validation failed
- `InsertConflict` - Duplicate key
- `QueryError` - Invalid query syntax
- `NetworkError` - Connection issues
- `HttpError` - HTTP status errors
- `JsonError` - JSON parsing errors

## 🔑 Usage Example

```rust
// Fetch all
let promos = client
    .from::<Promo>("promo")
    .execute()
    .await?;

// Fetch with filter
let promo = client
    .from::<Promo>("promo")
    .eq("voucher_code", "DISKON50")
    .execute_single()
    .await?;

// Update
let updated = client
    .from::<Promo>("promo")
    .eq("id_promo", "123")
    .update(&payload)
    .await?;

// Delete
client
    .from::<Promo>("promo")
    .eq("id_promo", "123")
    .delete()
    .await?;
```
