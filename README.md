# 🚀 Merchant Portal API

Backend Rust yang dibangun dengan Axum, dirancang untuk menangani traffic tinggi dengan resource terbatas. Membuktikan bahwa dengan 2 core CPU dan 1GB RAM, kita bisa melayani ribuan user bersamaan tanpa masalah.

<span style="color: gray;">Digunakan untuk melayani request data dari <a href="https://github.com/ApriadiS/merchantportal-client" style="color: #007acc; text-decoration: none;">Merchant Portal Client</a></span>

## 📌 Tentang Proyek

API backend yang awalnya dibuat untuk eksplorasi Rust, berkembang menjadi solusi production-ready dengan performa mengesankan. Fokus pada read-heavy operations dengan caching yang optimal.

**Tech Stack:**
- **Bahasa**: Rust 2024 Edition
- **Web Framework**: Axum
- **Runtime**: Tokio (multi-threaded)
- **Authentication**: JWT dengan caching
- **Database**: Supabase (PostgreSQL)
- **Caching**: In-memory dengan RwLock

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

## 🏆 Detail Load Testing

### Skrip Testing K6

Berikut adalah skrip k6 yang digunakan untuk testing:

```javascript
import http from "k6/http";
import { check, sleep } from "k6";
import { SharedArray } from "k6/data";

const API_BASE_URL = "http://localhost:3000"; // Ganti dengan URL API kamu
const JWT_TOKEN = "your_jwt_token_here";      // Ganti dengan token JWT valid

const HEADERS = {
   Authorization: `Bearer ${JWT_TOKEN}`,
   "Content-Type": "application/json",
};

export const options = {
   stages: [
      { duration: "30s", target: 200 },   // Warm up
      { duration: "1m", target: 500 },    // Sustainable load
      { duration: "1m", target: 1000 },   // High load
      { duration: "1m", target: 1500 },   // Peak load
      { duration: "30s", target: 0 },     // Cool down
   ],
   thresholds: {
      http_req_duration: ["p(95)<1000"],  // 95% requests under 1 second
      http_req_failed: ["rate<0.1"],      // Less than 10% failures
   },
};

// Load test data from CSV files
const storeRoutes = new SharedArray("store routes", function () {
   return loadAndExtractCSV("./store_rows.csv", "route");
});

const promoVouchers = new SharedArray("promo vouchers", function () {
   return loadAndExtractCSV("./promo_rows.csv", "voucher_code");
});

const promoStoreIds = new SharedArray("promo store ids", function () {
   return loadAndExtractCSV("./promo_store_rows.csv", "id");
});

// Helper function to load CSV data
function loadAndExtractCSV(filename, columnName) {
   const csvData = open(filename);
   const lines = csvData.split("\n").filter((line) => line.trim() !== "");

   if (lines.length === 0) {
      console.error(`ERROR: File ${filename} is empty.`);
      return [];
   }

   const rawHeaders = lines[0].split(",");
   const headers = rawHeaders.map((h) =>
      h.trim().toLowerCase().replace(/"/g, "")
   );
   const dataRows = lines.slice(1);

   const targetColumnName = columnName.toLowerCase();
   const columnIndex = headers.indexOf(targetColumnName);

   if (columnIndex === -1) {
      console.error(
         `ERROR: Column '${columnName}' not found in headers: ${rawHeaders.join(", ")}`
      );
      return [];
   }

   const extractedData = dataRows
      .map((line) => {
         const columns = line.split(/,(?=(?:(?:[^"]*"){2})*[^"]*$)/);
         if (columns[columnIndex]) {
            return columns[columnIndex].trim().replace(/"/g, "");
         }
         return null;
      })
      .filter((value) => value && String(value).trim() !== "");

   return extractedData;
}

// Define API routes with parameterized endpoints
const ROUTES = [
   { name: "/get-promo", handler: () => "/get-promo" },
   { name: "/get-store", handler: () => "/get-store" },
   { name: "/get-promo-store", handler: () => "/get-promo-store" },
   {
      name: "/get-store/{route}",
      handler: () => {
         if (storeRoutes.length === 0) return "/get-store/placeholder";
         const randomRoute = storeRoutes[Math.floor(Math.random() * storeRoutes.length)];
         return `/get-store/${randomRoute}`;
      },
   },
   {
      name: "/get-promo/{voucher}",
      handler: () => {
         if (promoVouchers.length === 0) return "/get-promo/placeholder";
         const randomVoucher = promoVouchers[Math.floor(Math.random() * promoVouchers.length)];
         return `/get-promo/${randomVoucher}`;
      },
   },
   {
      name: "/get-promo-store/{promo_store_id}",
      handler: () => {
         if (promoStoreIds.length === 0) return "/get-promo-store/placeholder";
         const randomPromoStoreId = promoStoreIds[Math.floor(Math.random() * promoStoreIds.length)];
         return `/get-promo-store/${randomPromoStoreId}`;
      },
   },
];

// Select random route for each virtual user
function getRandomRoute() {
   const routeObject = ROUTES[Math.floor(Math.random() * ROUTES.length)];
   const finalPath = routeObject.handler();
   return {
      route: routeObject.name,
      url: `${API_BASE_URL}${finalPath}`,
   };
}

// Main test function
export default function () {
   const { route: selectedRouteName, url } = getRandomRoute();
   const res = http.get(url, { headers: HEADERS });

   // Error logging for failed requests
   if (res.status !== 200) {
      console.error(`ERROR: Request to ${selectedRouteName} failed. Status: ${res.status}. URL: ${url}`);
   }

   // Performance checks
   check(res, {
      [`Accessed ${selectedRouteName} - Status is 200`]: (r) => r.status === 200,
      "Authentication Success (not 401/403)": (r) => r.status !== 401 && r.status !== 403,
      "Response time is acceptable": (r) => r.timings.duration < 1000,
   });

   sleep(0.1); // Small delay between requests
}
```

### Data Testing

Skrip ini membutuhkan file CSV berikut di folder yang sama:
- `store_rows.csv` - dengan kolom `route`
- `promo_rows.csv` - dengan kolom `voucher_code` 
- `promo_store_rows.csv` - dengan kolom `id`

**Cara menjalankan:**
```bash
k6 run load_test.js
```

## 🛠️ Optimasi yang Dilakukan

### **JWT Caching dengan Serialized Claims**
Token JWT tidak didecode ulang setiap request. Claims disimpan sebagai JSON dan di-cache dengan expiry time berdasarkan token expiration.

```rust
// Cache lookup di middleware
if let Some(cached) = state.cache_repository.get_cached_claims(&token).await {
    if let Ok(claims) = serde_json::from_value::<Claims>(cached) {
        request.extensions_mut().insert(Arc::new(claims));
        return Ok(next.run(request).await);
    }
}
```

### **In-Memory Data Caching**
Data dari Supabase di-cache dalam memory menggunakan `RwLock` dan `HashMap`:

- Cache semua data (promo, store, promo_store) di startup
- Lookup cache per item (by voucher, route, ID)
- Automatic cache warming saat aplikasi mulai

### **Tokio Multi-thread Configuration**
```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
```
Konfigurasi worker thread yang sesuai dengan jumlah CPU core.

## 📦 Arsitektur

Struktur yang clean dan maintainable:

```
Client Request 
    ↓
[Axum Router] 
    ↓
[JWT Middleware] ← Token validation dengan cache
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
├── middleware.rs   # JWT auth
├── startup.rs      # Cache warming
└── main.rs         # Entry point
```

📖 **Detail:** Lihat [src/README.md](src/README.md)

## 🧪 Cara Menjalankan

1. **Setup environment variables**:
```bash
cp .env.example .env
nano .env  # Edit dengan credentials Anda
```

2. **Jalankan aplikasi**:

**Untuk ARM64 (AWS EC2 Graviton, Apple Silicon):**
```bash
chmod +x deploy-arm64.sh
./deploy-arm64.sh
```

**Untuk x86_64 (Intel/AMD):**
```bash
chmod +x deploy-x86.sh
./deploy-x86.sh
```

**Atau gunakan Docker Compose:**
```bash
# ARM64
docker-compose -f docker-compose.arm64.yml up -d

# x86_64
docker-compose -f docker-compose.x86_64.yml up -d
```

3. **Jalankan k6 test**:
```bash
k6 run load_test.js
```

📖 **Dokumentasi lengkap:**
- [README-DOCKER.md](README-DOCKER.md) - Docker deployment
- [AWS-EC2-SETUP.md](AWS-EC2-SETUP.md) - AWS deployment
- [CRUD-FLOW.md](CRUD-FLOW.md) - CRUD operations
- [API-ENDPOINTS.md](API-ENDPOINTS.md) - API reference
- [QUICK-START.md](QUICK-START.md) - Quick start
- [src/README.md](src/README.md) - Source code structure

## ✅ Deployment

**Minimum requirements:**
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

## 🎯 Checklist Optimasi Lanjutan

### 🚀 **High Priority**
- [x] **Implementasi Response Compression** ✅
  - Gunakan `tower-http` compression layer
  - Reduce network traffic 60-70%

- [ ] **Connection Pooling untuk Supabase**
  - Gunakan `reqwest` Client dengan connection pool
  - Reduce database connection overhead

- [x] **Rate Limiting Middleware** ✅
  - Implementasi `tower-governor` atau custom rate limiter
  - Protect dari abuse dan DDoS (100 req/s, burst 50)

### 📈 **Medium Priority**  
- [x] **Metrics & Monitoring** ✅
  - `/metrics` endpoint dengan cache statistics
  - Ready untuk Prometheus integration

- [ ] **Cache Invalidation Strategy**
  - TTL-based invalidation untuk dynamic data
  - Background cache refresh

- [x] **Health Check & Readiness Probes** ✅
  - `/health` endpoint untuk load balancer
  - Version info included

### 🔧 **Architecture Improvements**
- [ ] **Background Cache Warming**
  - Periodic cache refresh tanpa blocking requests
  - Configurable refresh intervals

- [ ] **Configuration Management**
  - Environment-based config dengan validation
  - Hot-reloadable configuration

### 🛡️ **Production Ready**
- [ ] **Graceful Shutdown**
  - Proper signal handling (SIGTERM, SIGINT)
  - In-flight request completion

- [ ] **Security Hardening**
  - CORS configuration
  - Security headers middleware

### 📊 **Advanced Performance**
- [ ] **Load Shedding**
  - Request queuing dengan max limits
  - Circuit breaker pattern

- [ ] **CDN Integration**
  - Cache static responses at edge
  - Reduce origin server load

- [ ] **Horizontal Scaling Prep**
  - Stateless application design
  - External session storage (Redis)

---

*Setiap checklist item bisa diimplementasikan secara incremental berdasarkan kebutuhan traffic dan resource constraints.* 

✅ Production Ready untuk 1,500-2,000 concurrent users  

---

*Dibangun dengan ❤️ menggunakan Rust + Axum + Tokio. Tested dengan k6.*
