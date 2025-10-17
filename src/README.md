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
├── error.rs        # Global error types
├── middleware.rs   # JWT authentication
├── startup.rs      # Cache warming
└── main.rs         # Application entry point
```

## 🔄 Request Flow

```
HTTP Request
    ↓
[Handler] ← Validate & extract params
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
- `store_handler.rs` - Store CRUD endpoints
- `promo_store_handler.rs` - PromoStore CRUD endpoints
- `health_handler.rs` - Health & metrics endpoints

### **services/**
Business logic layer, orchestrate repository calls.
- `promo_service.rs` - Promo business logic
- `store_service.rs` - Store business logic
- `promo_store_service.rs` - PromoStore business logic

### **repositories/**
Data access layer dengan caching strategy.
- `promo_repository.rs` - Promo data access
- `store_repository.rs` - Store data access
- `promo_store_repository.rs` - PromoStore data access
- `cache_repository.rs` - In-memory caching

### **model/**
Domain models dan DTOs.
- `promo_model.rs` - Promo struct & payloads
- `store_model.rs` - Store struct & payloads
- `promo_store_model.rs` - PromoStore struct & payloads

### **supabase/**
Supabase client implementation.
- `supabase_client.rs` - Query builder & HTTP client
- `error.rs` - Supabase error types

## 🎯 Key Files

- **main.rs** - Application entry, router setup, middleware
- **app_state.rs** - Shared application state (services, cache)
- **middleware.rs** - JWT authentication with caching
- **startup.rs** - Cache warming on application start
- **error.rs** - Global error handling & responses
