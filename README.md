# 🚀 Merchant Portal API v1.1.0

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

## 🆕 What's New in v1.1.0

### 1. Query Filtering Endpoints
- ✅ `GET /get-promo?store_id={id}` - Filter promos by store (public)
- ✅ `GET /get-promo-store?promo_id={id}` - Filter by promo (auth)
- ✅ `GET /get-promo-store?store_id={id}` - Filter by store (auth)

### 2. Public Routes (No JWT Required)
- ✅ `GET /get-store` - List all stores (homepage)
- ✅ `GET /get-store/{route}` - Store details (store page)
- ✅ `GET /get-promo?store_id={id}` - Promos for store (store page)

### 3. Database Schema Update
- ✅ Promo fields renamed (removed `_promo` suffix)
- ✅ PromoStore now uses composite primary key `(promo_id, store_id)`
- ✅ Added discount fields: `discount`, `discount_type`, `max_discount`

### 4. Breaking Changes
- ⚠️ PromoStore endpoints now use format: `/get-promo-store/{promo_id}-{store_id}`
- ⚠️ All promo field names changed (see schema update)
- ⚠️ PromoStore response no longer includes `id` field

---

## 📊 API Endpoints

### 🌐 Public Endpoints (No Auth)
| Method | Endpoint | Function |
|--------|----------|----------|
| GET | `/health` | Health check |
| GET | `/metrics` | Monitoring metrics |
| GET | `/get-store` | List all stores |
| GET | `/get-store/{route}` | Store details |
| GET | `/get-promo?store_id={id}` | Promos for store |

### 🔐 Protected Endpoints (JWT Required)
| Method | Endpoint | Function |
|--------|----------|----------|
| GET | `/get-promo` | List all promos |
| GET | `/get-promo/{voucher}` | Promo by voucher |
| POST | `/create-promo` | Create promo |
| PUT | `/update-promo/{voucher}` | Update promo |
| DELETE | `/delete-promo/{voucher}` | Delete promo |
| POST | `/create-store` | Create store |
| PUT | `/update-store/{route}` | Update store |
| DELETE | `/delete-store/{route}` | Delete store |
| GET | `/get-promo-store` | List all relations |
| GET | `/get-promo-store?promo_id={id}` | Filter by promo |
| GET | `/get-promo-store?store_id={id}` | Filter by store |
| GET | `/get-promo-store/{promo_id}-{store_id}` | Get specific relation |
| POST | `/create-promo-store` | Create relation |
| PUT | `/update-promo-store/{promo_id}-{store_id}` | Update relation |
| DELETE | `/delete-promo-store/{promo_id}-{store_id}` | Delete relation |

**Total**: 5 public + 15 protected = 20 endpoints

---

## 🛠️ Optimasi yang Dilakukan

### **JWT Caching dengan Serialized Claims**
Token JWT tidak didecode ulang setiap request. Claims disimpan sebagai JSON dan di-cache dengan expiry time berdasarkan token expiration.

### **In-Memory Data Caching**
Data dari Supabase di-cache dalam memory menggunakan `RwLock` dan `HashMap`:
- Cache semua data (promo, store, promo_store) di startup
- Lookup cache per item (by voucher, route, composite key)
- Automatic cache warming saat aplikasi mulai

### **Public Routes Support**
Middleware bypass JWT validation untuk public endpoints, memungkinkan user browse stores dan promos tanpa login.

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
[Axum Router] 
    ↓
[JWT Middleware] ← Token validation dengan cache (skip untuk public routes)
    ↓  
[Handler Layer] ← Request handling & response
    ↓
[Service Layer] ← Business logic
    ↓
[Repository Layer] ← Data access dengan caching
    ↓
[Supabase Client] ← Database operations
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
├── error.rs        # Error handling
├── middleware.rs   # JWT auth + public routes
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

## 🧪 Load Testing

```bash
k6 run load_test.js
```

Skrip k6 sudah include di repository dengan support untuk:
- Multiple endpoints testing
- CSV data loading
- Performance thresholds
- Error logging

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
- [x] JWT Authentication dengan caching
- [x] In-memory data caching (RwLock + HashMap)
- [x] Response compression (gzip, 60-70% reduction)
- [x] Rate limiting (100 req/s per IP)
- [x] Health check & metrics endpoints
- [x] Public routes support (no JWT for read-only)
- [x] Query filtering endpoints
- [x] Composite key support (PromoStore)
- [x] Multi-architecture Docker images

### 🔄 Planned
- [ ] Connection pooling untuk Supabase
- [ ] Cache invalidation strategy (TTL-based)
- [ ] Background cache warming
- [ ] Graceful shutdown
- [ ] CORS configuration
- [ ] Load shedding & circuit breaker
- [ ] Horizontal scaling prep (Redis session)

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
```

---

## 🚀 Performance Tips

1. **Cache Warming**: Data di-cache saat startup untuk zero cold-start latency
2. **JWT Caching**: Token claims di-cache untuk menghindari decode berulang
3. **Public Routes**: Bypass JWT untuk read-only endpoints, reduce overhead
4. **Compression**: Gzip enabled untuk semua responses
5. **Rate Limiting**: Protect dari abuse dengan 100 req/s limit

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/ApriadiS/merchantportal-api/issues)
- **Frontend**: [Merchant Portal Client](https://github.com/ApriadiS/merchantportal-client)

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

---

**Version**: 1.1.0  
**Last Updated**: 2025-01-17  
**Status**: ✅ Production Ready

*Dibangun dengan ❤️ menggunakan Rust + Axum + Tokio. Tested dengan k6.*
