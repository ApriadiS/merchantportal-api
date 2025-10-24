# 🚀 Merchant Portal API v1.2.2

Backend Rust yang dibangun dengan Axum, dirancang untuk menangani traffic tinggi dengan resource terbatas. Membuktikan bahwa dengan 2 core CPU dan 1GB RAM, kita bisa melayani ribuan user bersamaan tanpa masalah.

<span style="color: gray;">Digunakan untuk melayani request data dari <a href="https://github.com/ApriadiS/merchantportal-client" style="color: #007acc; text-decoration: none;">Merchant Portal Client</a></span>

---

## 📌 Tentang Proyek

API backend yang awalnya dibuat untuk eksplorasi Rust, berkembang menjadi solusi production-ready dengan performa mengesankan. Fokus pada read-heavy operations dengan caching yang optimal.

**Tech Stack:**
- **Bahasa**: Rust 2024 Edition
- **Web Framework**: Axum
- **Runtime**: Tokio (multi-threaded)
- **Authentication**: JWT dengan caching
- **Database**: Supabase (PostgreSQL)
- **Caching**: In-memory dengan RwLock

---

## 🎯 Hasil Performance

Ringkasan hasil load testing dengan k6 pada environment lokal:

| Metric | Hasil | Keterangan |
|--------|-------|------------|
| **Max Concurrent Users** | 3,000 VUs | Beban puncak |
| **Throughput** | 6,318 req/s | Request per detik pada puncak |
| **95th Percentile Latency** | 136 ms | Optimal di 1,500 VUs |
| **Error Rate** | 0.00% | Stabil tanpa error |
| **Memory Usage** | ~60 MB | Sangat hemat memory |
| **CPU Utilization** | 170-183% | Utilisasi optimal di 2 vCPU |

**Sweet spot:** 1,500-2,000 user bersamaan dengan throughput ~4,700 req/s dan latency 136ms.

---

## 🆕 What's New in v1.2.2

### 🐛 Medium Priority Bug Fixes

**1. Rate Limiter Now Applies to Protected Routes**
- ✅ Removed skip logic for protected routes
- ✅ Mutations (create/update/delete) now properly rate-limited
- ✅ Only public endpoints bypass rate limiting

**2. Public Endpoints Consistency**
- ✅ Created `src/constants.rs` with shared PUBLIC_ENDPOINTS
- ✅ Eliminates duplicated logic between middleware and rate limiter
- ✅ Single source of truth for public endpoint definitions

**3. JWT Cache Auto-Expiry**
- ✅ Expired tokens automatically removed from cache
- ✅ Prevents revoked tokens from working until restart
- ✅ Better security and cache hygiene

**4. Tenor Value Validation**
- ✅ Tenor must be between 1-60 months
- ✅ Returns proper error for invalid values
- ✅ Prevents data corruption from invalid input

**5. Cache Invalidation Strategy**
- ✅ PromoStore mutations invalidate PromoTenor cache
- ✅ Ensures fresh data after link/unlink operations
- ✅ Fixed in v1.2.1, documented here

### 🔄 Breaking Changes
- None - All changes backward compatible

---

## 📜 Changelog v1.2.0

### 🎯 Production Hardening

**1. Domain-Specific Error Handling**
- ✅ Organized errors per domain (Store, Promo, PromoTenor, PromoStore)
- ✅ Proper HTTP status codes (404, 409, 400, 500)
- ✅ Clear error messages untuk better UX

**2. CORS Whitelist**
- ✅ Environment-based configuration
- ✅ Support multiple origins (IP + domain)
- ✅ Configurable methods & max age
- ✅ Wildcard support

**3. Structured Logging + Performance Metrics**
- ✅ JSON format support untuk production
- ✅ Request correlation IDs (UUID)
- ✅ Performance tracking:
  - JWT validation duration
  - Cache hit/miss dengan latency
  - Request duration
  - Fingerprint generation time

**4. Enhanced Health Checks**
- ✅ `/health` - Detailed health status
- ✅ `/ready` - Kubernetes readiness probe
- ✅ `/metrics` - Cache statistics
- ✅ Timestamp pada semua responses

**5. Fingerprint-Based Rate Limiting**
- ✅ SHA256 hash (IP + User-Agent + Accept-Language)
- ✅ Configurable limits dari environment
- ✅ Skip protected routes (sudah ada JWT)
- ✅ Performance tracking

### 🔄 Breaking Changes
- None - All changes backward compatible

---

## 📊 API Endpoints

### 🌐 Public Endpoints (No Auth)
| Method | Endpoint | Function |
|--------|----------|----------|
| GET | `/health` | Health check with detailed status |
| GET | `/ready` | Kubernetes readiness probe |
| GET | `/metrics` | Cache statistics |
| GET | `/get-store` | List all stores |
| GET | `/get-store/{route}` | Store details |
| GET | `/get-promo?store_id={id}` | Promos for store |
| GET | `/get-promo-tenor` | List all promo tenors |
| GET | `/get-promo-tenor?promo_id={id}` | Filter by promo |
| GET | `/get-promo-tenor?tenor={n}` | Filter by tenor |
| GET | `/get-promo-tenor?voucher={code}` | Filter by voucher |
| GET | `/get-promo-tenor-by-store/{store_id}` | Get tenors by store (optimized) |

### 🔐 Protected Endpoints (JWT Required)

#### Promo Endpoints
| Method | Endpoint | Function |
|--------|----------|----------|
| GET | `/get-promo` | List all promos |
| GET | `/get-promo/{id_promo}` | Promo by ID (UUID) |
| POST | `/create-promo` | Create promo |
| PUT | `/update-promo/{id_promo}` | Update promo |
| DELETE | `/delete-promo/{id_promo}` | Delete promo |

#### PromoTenor Endpoints
| Method | Endpoint | Function |
|--------|----------|----------|
| GET | `/get-promo-tenor/{id}` | Get tenor by ID (UUID) |
| POST | `/create-promo-tenor` | Create promo tenor |
| PUT | `/update-promo-tenor/{id}` | Update promo tenor |
| DELETE | `/delete-promo-tenor/{id}` | Delete promo tenor |

#### Store Endpoints
| Method | Endpoint | Function |
|--------|----------|----------|
| POST | `/create-store` | Create store |
| PUT | `/update-store/{route}` | Update store |
| DELETE | `/delete-store/{route}` | Delete store |

#### PromoStore Endpoints
| Method | Endpoint | Function |
|--------|----------|----------|
| GET | `/get-promo-store` | List all relations |
| GET | `/get-promo-store?promo_id={id}` | Filter by promo |
| GET | `/get-promo-store?store_id={id}` | Filter by store |
| GET | `/get-promo-store/{promo_id}-{store_id}` | Get specific relation |
| POST | `/create-promo-store` | Create relation |
| PUT | `/update-promo-store/{promo_id}-{store_id}` | Update relation |
| DELETE | `/delete-promo-store/{promo_id}-{store_id}` | Delete relation |

**Total**: 11 public + 23 protected = 34 endpoints

---

## 🛠️ Optimasi yang Dilakukan

### **Domain-Specific Error Handling**
Error handling terorganisir per domain dengan proper HTTP status codes:
- `StoreError`, `PromoError`, `PromoTenorError`, `PromoStoreError`
- Automatic conversion ke `AppError`
- Clear error messages untuk better debugging

### **JWT Caching dengan Performance Tracking**
Token JWT tidak didecode ulang setiap request:
- Claims disimpan sebagai JSON di cache
- Expiry time berdasarkan token expiration
- Track JWT validation duration (~2-5ms)
- Cache hit rate logging

### **In-Memory Data Caching**
Data dari Supabase di-cache dalam memory:
- Cache semua data (promo, store, promo_store, promo_tenor) di startup
- Lookup cache per item (by ID, route, composite key)
- Automatic cache warming saat aplikasi mulai
- Cache status monitoring via `/health`

### **Fingerprint-Based Rate Limiting**
Rate limiting dengan fingerprinting untuk bypass proxy:
- SHA256 hash dari IP + User-Agent + Accept-Language
- Configurable limits (default: 50 req/60s)
- Skip protected routes (sudah ada JWT auth)
- Performance tracking (~0.05ms per fingerprint)

### **Structured Logging dengan Metrics**
Production-ready logging:
- JSON format support untuk log aggregation
- Request correlation IDs (UUID)
- Performance metrics tracking
- Environment-based log format (pretty/json)

### **Tokio Multi-thread Configuration**
```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
```
Konfigurasi worker thread yang sesuai dengan jumlah CPU core.

---

## 📦 Arsitektur

```
Client Request 
    ↓
[Request Logging] ← Correlation ID + timing
    ↓
[Rate Limiter] ← Fingerprint-based (public routes only)
    ↓
[CORS Layer] ← Whitelist validation
    ↓
[JWT Middleware] ← Token validation dengan cache (skip untuk public routes)
    ↓  
[Handler Layer] ← Request handling & domain errors
    ↓
[Service Layer] ← Business logic
    ↓
[Repository Layer] ← Data access dengan caching
    ↓
[Supabase Client] ← Database operations (HTTP REST API)
```

**Struktur Folder:**
```
src/
├── handlers/       # HTTP request handlers
├── services/       # Business logic layer
├── repositories/   # Data access + caching
├── model/          # Domain models & DTOs
├── supabase/       # Supabase client
├── app_state.rs    # Application state
├── error.rs        # Domain-specific error handling
├── middleware.rs   # JWT auth + CORS + request logging
├── rate_limiter.rs # Fingerprint-based rate limiting
├── startup.rs      # Cache warming
└── main.rs         # Entry point
```

📖 **Detail:** Lihat [src/README.md](src/README.md)

---

## 🧪 Quick Start

### 1. Setup Environment
```bash
cp .env.example .env
nano .env  # Edit dengan credentials Anda
```

### 2. Run with Docker

**ARM64 (AWS Graviton, Apple Silicon):**
```bash
chmod +x deploy-arm64.sh
./deploy-arm64.sh
```

**x86_64 (Intel/AMD):**
```bash
chmod +x deploy-x86.sh
./deploy-x86.sh
```

**Docker Compose:**
```bash
# ARM64
docker-compose -f docker-compose.arm64.yml up -d

# x86_64
docker-compose -f docker-compose.x86_64.yml up -d
```

### 3. Test API

**Public endpoints (no JWT):**
```bash
curl http://localhost:3000/get-store
curl http://localhost:3000/get-store/toko-elektronik-jakarta
curl http://localhost:3000/get-promo?store_id=1
```

**Protected endpoints (JWT required):**
```bash
curl http://localhost:3000/get-promo \
  -H "Authorization: Bearer <jwt_token>"
```

---

## ✅ Deployment

**Minimum Requirements:**
- **vCPUs**: 2
- **RAM**: 1GB  
- **Storage**: 10GB SSD

**Supported Architectures:**
- ✅ ARM64 (AWS EC2 Graviton, Apple Silicon)
- ✅ x86_64 (Intel/AMD)

**Docker Images:**
- `Dockerfile.arm64` - Optimized for ARM64
- `Dockerfile.x86_64` - Optimized for x86_64

🔐 **Security:** ENV variables di-inject saat runtime, tidak hardcoded di image

---

## 🎯 Features Checklist

### ✅ Implemented
- [x] JWT Authentication dengan caching + performance tracking
- [x] In-memory data caching (RwLock + HashMap)
- [x] Response compression (gzip, 60-70% reduction)
- [x] Fingerprint-based rate limiting (SHA256)
- [x] Health check & metrics endpoints
- [x] Public routes support (no JWT for read-only)
- [x] Query filtering endpoints
- [x] Composite key support (PromoStore)
- [x] Multi-architecture Docker images
- [x] Domain-specific error handling
- [x] CORS whitelist configuration
- [x] Structured logging dengan JSON format
- [x] Request correlation IDs
- [x] Performance metrics tracking
- [x] Kubernetes readiness probe

### 🔄 Planned
- [ ] Cache invalidation strategy (TTL-based)
- [ ] Background cache warming
- [ ] Graceful shutdown
- [ ] Load shedding & circuit breaker
- [ ] Horizontal scaling prep (Redis session)
- [ ] Database connection pooling (if migrate from HTTP to native driver)

---

## 📝 Environment Variables

```bash
# Database
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_KEY=your-anon-key

# Authentication
JWT_SECRET=your-jwt-secret

# Application
MODE=prod  # or 'dev' to bypass JWT
RUST_LOG=info
RUST_BACKTRACE=0

# CORS Configuration
CORS_ALLOWED_ORIGINS=http://localhost:3000,https://yourdomain.com
CORS_ALLOWED_METHODS=GET,POST,PUT,DELETE
CORS_MAX_AGE=3600

# Logging Configuration
LOG_FORMAT=pretty  # or 'json' for production

# Rate Limiting Configuration
RATE_LIMIT_ENABLED=true
RATE_LIMIT_REQUESTS=50
RATE_LIMIT_WINDOW_SECONDS=60
```

---

## 🚀 Performance Tips

1. **Cache Warming**: Data di-cache saat startup untuk zero cold-start latency
2. **JWT Caching**: Token claims di-cache untuk menghindari decode berulang (~95% hit rate)
3. **Public Routes**: Bypass JWT untuk read-only endpoints, reduce overhead
4. **Compression**: Gzip enabled untuk semua responses (60-70% reduction)
5. **Fingerprint Rate Limiting**: Protect dari abuse dengan fingerprinting (default: 50 req/60s)
6. **Structured Logging**: JSON format untuk production, performance metrics tracking
7. **Domain Errors**: Organized error handling dengan proper HTTP status codes

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/ApriadiS/merchantportal-api/issues)
- **Frontend**: [Merchant Portal Client](https://github.com/ApriadiS/merchantportal-client)

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

---

**Version**: 1.2.2  
**Last Updated**: 2025-01-20  
**Status**: ✅ Production Ready

*Dibangun dengan ❤️ menggunakan Rust + Axum + Tokio. Tested dengan k6.*
