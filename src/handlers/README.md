# 🎯 Handlers

HTTP request handlers untuk setiap endpoint.

## 📋 Files

### **promo_handler.rs**
- `han_get_all_promos()` - GET /get-promo
- `han_get_promo_by_voucher()` - GET /get-promo/{voucher_code}
- `han_create_promo()` - POST /create-promo
- `han_update_promo()` - PUT /update-promo/{voucher_code}
- `han_delete_promo()` - DELETE /delete-promo/{voucher_code}

### **store_handler.rs**
- `han_get_stores()` - GET /get-store
- `han_get_store_by_route()` - GET /get-store/{route}
- `han_create_store()` - POST /create-store
- `han_update_store()` - PUT /update-store/{route}
- `han_delete_store()` - DELETE /delete-store/{route}

### **promo_store_handler.rs**
- `han_get_promo_stores()` - GET /get-promo-store
- `han_get_promo_store_by_id()` - GET /get-promo-store/{id}
- `han_create_promo_store()` - POST /create-promo-store
- `han_update_promo_store()` - PUT /update-promo-store/{id}
- `han_delete_promo_store()` - DELETE /delete-promo-store/{id}

### **health_handler.rs**
- `health_check()` - GET /health
- `metrics()` - GET /metrics

## 🔑 Responsibilities

- Extract & validate request parameters
- Call service layer
- Handle errors
- Return JSON responses
