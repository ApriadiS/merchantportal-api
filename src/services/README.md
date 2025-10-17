# ⚙️ Services

Business logic layer, orchestrate repository operations.

## 📋 Files

### **promo_service.rs**
- `ser_get_all_promos()` - Fetch all promos
- `ser_get_promo_by_voucher()` - Fetch by voucher_code
- `ser_create_promo()` - Create new promo
- `ser_update_promo()` - Update by voucher_code
- `ser_delete_promo()` - Delete by voucher_code

### **store_service.rs**
- `ser_get_all_stores()` - Fetch all stores
- `ser_get_store_by_route()` - Fetch by route
- `ser_create_store()` - Create new store
- `ser_update_store()` - Update by route
- `ser_delete_store()` - Delete by route

### **promo_store_service.rs**
- `ser_get_all_promo_stores()` - Fetch all promo_stores
- `ser_get_promo_store_by_id()` - Fetch by id
- `ser_create_promo_store()` - Create new promo_store
- `ser_update_promo_store()` - Update by id
- `ser_delete_promo_store()` - Delete by id

## 🔑 Responsibilities

- Business logic validation
- Orchestrate repository calls
- Transform data between layers
- Error handling
