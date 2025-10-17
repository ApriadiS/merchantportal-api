# 🔄 CRUD Flow Documentation

## 📋 Overview

Project ini menggunakan **unique identifiers** yang user-friendly untuk CRUD operations, tapi tetap menggunakan **ID** untuk operasi Supabase karena requirement database.

## 🎯 Unique Identifiers per Table

| Table | User Input | Database Primary Key | Notes |
|-------|-----------|---------------------|-------|
| **Store** | `route` (string) | `id` (integer) | User tidak perlu tahu ID |
| **Promo** | `voucher_code` (string) | `id_promo` (integer) | User tidak perlu tahu ID |
| **PromoStore** | `id` (integer) | `id` (integer) | Langsung pakai ID |

## 🔍 CRUD Flow Pattern

### **READ Operations**
User memberikan unique identifier → Backend cek cache → Jika miss, query Supabase

### **CREATE Operations**
User memberikan data → Backend insert ke Supabase → Clear cache

### **UPDATE Operations**
1. User memberikan unique identifier + data update
2. Backend lookup ID dari cache (jika ada)
3. Jika tidak ada di cache, query Supabase untuk dapat ID
4. Backend update ke Supabase menggunakan ID
5. Clear cache

### **DELETE Operations**
1. User memberikan unique identifier
2. Backend lookup ID dari cache (jika ada)
3. Jika tidak ada di cache, query Supabase untuk dapat ID
4. Backend delete dari Supabase menggunakan ID
5. Clear cache

## 📝 Implementation Details

### **Store CRUD**

**Endpoint Pattern:**
```
GET    /get-store/{route}
POST   /create-store
PUT    /update-store/{route}
DELETE /delete-store/{route}
```

**Update Flow:**
```rust
// 1. Get ID from cache or DB
let id = if let Some(cached) = cache.get_store_by_route(route).await {
    cached.id
} else {
    let store = fetch_from_db(route).await?;
    store.id
};

// 2. Update using ID
supabase.from("store")
    .eq("id", id.to_string())
    .update(payload)
    .await?;
```

**Delete Flow:**
```rust
// 1. Get ID from cache or DB
let id = if let Some(cached) = cache.get_store_by_route(route).await {
    cached.id
} else {
    let store = fetch_from_db(route).await?;
    store.id
};

// 2. Delete using ID
supabase.from("store")
    .eq("id", id.to_string())
    .delete()
    .await?;
```

### **Promo CRUD**

**Endpoint Pattern:**
```
GET    /get-promo/{voucher_code}
POST   /create-promo
PUT    /update-promo/{voucher_code}
DELETE /delete-promo/{voucher_code}
```

**Update Flow:**
```rust
// 1. Get ID from cache or DB
let id_promo = if let Some(cached) = cache.get_promo_by_voucher(voucher).await {
    cached.id_promo
} else {
    let promo = fetch_from_db(voucher).await?;
    promo.id_promo
};

// 2. Update using ID
supabase.from("promo")
    .eq("id_promo", id_promo.to_string())
    .update(payload)
    .await?;
```

**Delete Flow:**
```rust
// 1. Get ID from cache or DB
let id_promo = if let Some(cached) = cache.get_promo_by_voucher(voucher).await {
    cached.id_promo
} else {
    let promo = fetch_from_db(voucher).await?;
    promo.id_promo
};

// 2. Delete using ID
supabase.from("promo")
    .eq("id_promo", id_promo.to_string())
    .delete()
    .await?;
```

### **PromoStore CRUD**

**Endpoint Pattern:**
```
GET    /get-promo-store/{id}
POST   /create-promo-store
PUT    /update-promo-store/{id}
DELETE /delete-promo-store/{id}
```

**Note:** PromoStore langsung menggunakan ID karena hanya memiliki 3 kolom dan tidak ada unique identifier lain yang user-friendly.

## ⚡ Performance Optimization

### **Cache Strategy**

1. **Cache Hit (Fast Path)**
   - Lookup ID dari in-memory cache
   - Langsung gunakan ID untuk operasi Supabase
   - **Latency:** ~1-2ms

2. **Cache Miss (Slow Path)**
   - Query Supabase untuk dapat full record
   - Extract ID dari record
   - Gunakan ID untuk operasi Supabase
   - **Latency:** ~50-100ms (tergantung network)

### **Why This Approach?**

✅ **User-Friendly:** User tidak perlu tahu/hafal ID internal
✅ **Database Requirement:** Supabase butuh ID untuk UPDATE/DELETE
✅ **Cache Optimization:** Mayoritas request hit cache (fast path)
✅ **Consistency:** Single source of truth untuk ID mapping

## 🔒 Error Handling

### **Not Found Scenarios**

1. **Unique identifier tidak ada di cache & DB**
   ```
   Error: "Store dengan route 'xxx' tidak ditemukan"
   Error: "Promo dengan voucher_code 'xxx' tidak ditemukan"
   ```

2. **ID tidak ditemukan saat UPDATE/DELETE**
   ```
   Error: "Failed to update/delete - record not found"
   ```

### **Conflict Scenarios**

1. **Duplicate unique identifier saat CREATE**
   ```
   Error: "Store dengan route 'xxx' sudah ada"
   Error: "Promo dengan voucher_code 'xxx' sudah ada"
   ```

## 📊 Example Request/Response

### **Update Store by Route**

**Request:**
```bash
PUT /update-store/toko-elektronik-jakarta
Content-Type: application/json

{
  "name": "Toko Elektronik Jakarta Pusat",
  "address": "Jl. Sudirman No. 123"
}
```

**Backend Flow:**
```
1. Extract route: "toko-elektronik-jakarta"
2. Check cache for route → Found! ID = 42
3. Update Supabase: eq("id", "42")
4. Clear cache
5. Return updated store
```

**Response:**
```json
{
  "id": 42,
  "route": "toko-elektronik-jakarta",
  "name": "Toko Elektronik Jakarta Pusat",
  "address": "Jl. Sudirman No. 123",
  ...
}
```

### **Delete Promo by Voucher Code**

**Request:**
```bash
DELETE /delete-promo/DISKON50
```

**Backend Flow:**
```
1. Extract voucher: "DISKON50"
2. Check cache for voucher → Miss!
3. Query Supabase: eq("voucher_code", "DISKON50") → ID = 123
4. Delete from Supabase: eq("id_promo", "123")
5. Clear cache
6. Return success
```

**Response:**
```json
{}
```

## 🎯 Best Practices

1. ✅ **Always validate unique identifier exists before UPDATE/DELETE**
2. ✅ **Use cache for ID lookup to minimize DB queries**
3. ✅ **Clear cache after any mutation (CREATE/UPDATE/DELETE)**
4. ✅ **Return meaningful error messages with the unique identifier**
5. ✅ **Log cache hit/miss for monitoring**

## 🔄 Cache Invalidation Strategy

**Current:** Clear entire cache after mutation
**Future:** Selective cache invalidation per item

```rust
// Current
cache.clear_promo_cache_all().await;

// Future (optimization)
cache.invalidate_promo_by_voucher(voucher).await;
```

---

**Built with ❤️ for optimal user experience**
