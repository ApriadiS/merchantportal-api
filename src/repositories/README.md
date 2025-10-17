# 💾 Repositories

Data access layer dengan caching strategy.

## 📋 Files

### **promo_repository.rs**
- `rep_fetch_all()` - Fetch all from cache/DB
- `rep_get_by_voucher()` - Fetch by voucher_code
- `rep_insert()` - Insert to DB, clear cache
- `rep_update_by_voucher()` - voucher_code → ID → update
- `rep_delete_by_voucher()` - voucher_code → ID → delete

### **store_repository.rs**
- `rep_fetch_all()` - Fetch all from cache/DB
- `rep_fetch_by_route()` - Fetch by route
- `rep_create()` - Insert to DB, clear cache
- `rep_update()` - route → ID → update
- `rep_delete()` - route → ID → delete

### **promo_store_repository.rs**
- `rep_fetch_all()` - Fetch all from cache/DB
- `rep_fetch_by_id()` - Fetch by id
- `rep_insert()` - Insert to DB, clear cache
- `rep_update_by_id()` - Update by id (direct)
- `rep_delete_by_id()` - Delete by id (direct)

### **cache_repository.rs**
In-memory caching dengan RwLock.
- JWT token caching
- Promo data caching (all + by voucher)
- Store data caching (all + by route)
- PromoStore data caching (all + by id)

## 🔑 Responsibilities

- Cache management (read/write/clear)
- Database operations via Supabase client
- ID lookup for unique identifiers
- Cache invalidation after mutations
