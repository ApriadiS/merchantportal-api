# 📂 Source Code Structure

## 📋 Overview

Arsitektur clean dengan separation of concerns yang jelas.

```
src/
├── handlers/       # HTTP request handlers
├── services/       # Business logic layer
├── repositories/   # Data access layer
├── model/          # Domain models & DTOs
├── supabase/       # Supabase client & error handling
├── app_state.rs    # Application state
├── error.rs        # Domain-specific error handling
├── middleware.rs   # JWT auth + CORS + request logging
├── rate_limiter.rs # Fingerprint-based rate limiting
├── startup.rs      # Cache warming
└── main.rs         # Application entry point
```

## 🔄 Request Flow

```
HTTP Request
    ↓
[Request Logging] ← Correlation ID + timing
    ↓
[Rate Limiter] ← Fingerprint-based (public routes)
    ↓
[CORS Layer] ← Whitelist validation
    ↓
[JWT Middleware] ← Token validation + cache
    ↓
[Handler] ← Validate & extract params + domain errors
    ↓
[Service] ← Business logic
    ↓
[Repository] ← Data access + caching
    ↓
[Supabase Client] ← Database operations
```

## 📁 Directory Details

### **handlers/**
HTTP request handlers untuk setiap endpoint.
- `promo_handler.rs` - Promo CRUD endpoints
- `promo_tenor_handler.rs` - PromoTenor CRUD endpoints
- `store_handler.rs` - Store CRUD endpoints
- `promo_store_handler.rs` - PromoStore CRUD endpoints
- `health_handler.rs` - Health, ready & metrics endpoints

### **services/**
Business logic layer, orchestrate repository calls.
- `promo_service.rs` - Promo business logic
- `promo_tenor_service.rs` - PromoTenor business logic
- `store_service.rs` - Store business logic
- `promo_store_service.rs` - PromoStore business logic

### **repositories/**
Data access layer dengan caching strategy.
- `promo_repository.rs` - Promo data access
- `promo_tenor_repository.rs` - PromoTenor data access
- `store_repository.rs` - Store data access
- `promo_store_repository.rs` - PromoStore data access
- `cache_repository.rs` - In-memory caching

### **model/**
Domain models dan DTOs.
- `promo_model.rs` - Promo struct & payloads
- `promo_tenor_model.rs` - PromoTenor struct & payloads
- `store_model.rs` - Store struct & payloads
- `promo_store_model.rs` - PromoStore struct & payloads

### **supabase/**
Supabase client implementation.
- `supabase_client.rs` - Query builder & HTTP client
- `error.rs` - Supabase error types

## 🎯 Key Files

- **main.rs** - Application entry, router setup, middleware
- **app_state.rs** - Shared application state (services, cache)
- **middleware.rs** - JWT auth + CORS + request logging
- **rate_limiter.rs** - Fingerprint-based rate limiting
- **startup.rs** - Cache warming on application start
- **error.rs** - Domain-specific error handling
