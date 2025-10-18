# 🎯 Handlers

HTTP request handlers untuk setiap endpoint.

## 📋 Files

### **promo_handler.rs**
- `han_get_all_promos()` - GET /get-promo (with ?store_id filter)
- `han_get_promo_by_id()` - GET /get-promo/{id_promo}
- `han_create_promo()` - POST /create-promo
- `han_update_promo()` - PUT /update-promo/{id_promo}
- `han_delete_promo()` - DELETE /delete-promo/{id_promo}

### **promo_tenor_handler.rs**
- `han_get_all_promo_tenors()` - GET /get-promo-tenor (with filters)
- `han_get_promo_tenor_by_id()` - GET /get-promo-tenor/{id}
- `han_create_promo_tenor()` - POST /create-promo-tenor
- `han_update_promo_tenor()` - PUT /update-promo-tenor/{id}
- `han_delete_promo_tenor()` - DELETE /delete-promo-tenor/{id}

### **store_handler.rs**
- `han_get_stores()` - GET /get-store
- `han_get_store_by_route()` - GET /get-store/{route}
- `han_create_store()` - POST /create-store
- `han_update_store()` - PUT /update-store/{route}
- `han_delete_store()` - DELETE /delete-store/{route}

### **promo_store_handler.rs**
- `han_get_promo_stores()` - GET /get-promo-store (with filters)
- `han_get_promo_store_by_key()` - GET /get-promo-store/{promo_id}-{store_id}
- `han_create_promo_store()` - POST /create-promo-store
- `han_update_promo_store()` - PUT /update-promo-store/{promo_id}-{store_id}
- `han_delete_promo_store()` - DELETE /delete-promo-store/{promo_id}-{store_id}

### **health_handler.rs**
- `health_check()` - GET /health (detailed status)
- `ready_check()` - GET /ready (Kubernetes probe)
- `metrics()` - GET /metrics (cache statistics)

## 🔑 Responsibilities

- Extract & validate request parameters
- Call service layer
- Handle domain-specific errors
- Return JSON responses with proper status codes
