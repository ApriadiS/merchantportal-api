# 📡 API Endpoints Documentation

## 🎯 Overview

Semua endpoint memerlukan JWT authentication (kecuali MODE=dev).

**Base URL:** `http://localhost:3000`

---

## 📦 PROMO Endpoints

### **Unique Key:** `voucher_code` (string)

| Method | Endpoint | Function | Key Parameter | Description |
|--------|----------|----------|---------------|-------------|
| GET | `/get-promo` | `rep_fetch_all()` | - | Ambil semua promo |
| GET | `/get-promo/{voucher_code}` | `rep_get_by_voucher()` | `voucher_code` | Ambil promo by voucher |
| POST | `/create-promo` | `rep_insert()` | - | Buat promo baru |
| PUT | `/update-promo/{voucher_code}` | `rep_update_by_voucher()` | `voucher_code` | Update promo by voucher |
| DELETE | `/delete-promo/{voucher_code}` | `rep_delete_by_voucher()` | `voucher_code` | Hapus promo by voucher |

### **Repository Functions:**

```rust
// READ
rep_fetch_all() -> Vec<Promo>                    // Ambil semua
rep_get_by_voucher(voucher: &str) -> Promo       // Ambil by voucher_code

// CREATE
rep_insert(payload: &Value) -> Promo             // Insert baru

// UPDATE
rep_update_by_voucher(voucher: &str, payload: &Value) -> Promo
// Flow: voucher_code → lookup id_promo → update by id_promo

// DELETE
rep_delete_by_voucher(voucher: &str) -> ()
// Flow: voucher_code → lookup id_promo → delete by id_promo
```

### **Internal Flow:**
- **User Input:** `voucher_code` (e.g., "DISKON50")
- **Backend Lookup:** `voucher_code` → `id_promo` (dari cache/DB)
- **Supabase Operation:** Menggunakan `id_promo` untuk UPDATE/DELETE

---

## 🏪 STORE Endpoints

### **Unique Key:** `route` (string)

| Method | Endpoint | Function | Key Parameter | Description |
|--------|----------|----------|---------------|-------------|
| GET | `/get-store` | `rep_fetch_all()` | - | Ambil semua store |
| GET | `/get-store/{route}` | `rep_fetch_by_route()` | `route` | Ambil store by route |
| POST | `/create-store` | `rep_create()` | - | Buat store baru |
| PUT | `/update-store/{route}` | `rep_update()` | `route` | Update store by route |
| DELETE | `/delete-store/{route}` | `rep_delete()` | `route` | Hapus store by route |

### **Repository Functions:**

```rust
// READ
rep_fetch_all() -> Vec<Store>                    // Ambil semua
rep_fetch_by_route(route: &str) -> Store         // Ambil by route

// CREATE
rep_create(payload: CreateStorePayload) -> Store // Insert baru

// UPDATE
rep_update(route: &str, payload: UpdateStorePayload) -> Store
// Flow: route → lookup id → update by id

// DELETE
rep_delete(route: &str) -> ()
// Flow: route → lookup id → delete by id
```

### **Internal Flow:**
- **User Input:** `route` (e.g., "toko-elektronik-jakarta")
- **Backend Lookup:** `route` → `id` (dari cache/DB)
- **Supabase Operation:** Menggunakan `id` untuk UPDATE/DELETE

---

## 🔗 PROMO_STORE Endpoints

### **Unique Key:** `id` (integer)

| Method | Endpoint | Function | Key Parameter | Description |
|--------|----------|----------|---------------|-------------|
| GET | `/get-promo-store` | `rep_fetch_all()` | - | Ambil semua promo_store |
| GET | `/get-promo-store/{id}` | `rep_fetch_by_id()` | `id` | Ambil promo_store by id |
| POST | `/create-promo-store` | `rep_insert()` | - | Buat promo_store baru |
| PUT | `/update-promo-store/{id}` | `rep_update_by_id()` | `id` | Update promo_store by id |
| DELETE | `/delete-promo-store/{id}` | `rep_delete_by_id()` | `id` | Hapus promo_store by id |

### **Repository Functions:**

```rust
// READ
rep_fetch_all() -> Vec<PromoStore>               // Ambil semua
rep_fetch_by_id(id: &u32) -> PromoStore          // Ambil by id

// CREATE
rep_insert(payload: &Value) -> PromoStore        // Insert baru

// UPDATE
rep_update_by_id(id: &u32, payload: &Value) -> PromoStore
// Flow: Langsung pakai id (no lookup needed)

// DELETE
rep_delete_by_id(id: &u32) -> ()
// Flow: Langsung pakai id (no lookup needed)
```

### **Internal Flow:**
- **User Input:** `id` (e.g., 123)
- **Backend:** Langsung gunakan `id`
- **Supabase Operation:** Langsung menggunakan `id` (no lookup)

---

## 📊 Summary Table

| Table | User Key | DB Primary Key | Lookup Required? | Cache Key |
|-------|----------|----------------|------------------|-----------|
| **Promo** | `voucher_code` | `id_promo` | ✅ Yes | `voucher_code` |
| **Store** | `route` | `id` | ✅ Yes | `route` |
| **PromoStore** | `id` | `id` | ❌ No | `id` |

---

## 🔄 CRUD Operation Flow

### **Promo & Store (dengan lookup):**

```
┌─────────────┐
│ User Request│
│ (voucher/   │
│  route)     │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Check Cache │◄─── Fast Path (1-2ms)
└──────┬──────┘
       │
       ├─── Cache Hit ──► Get ID ──┐
       │                           │
       └─── Cache Miss ──► Query DB ──► Get ID ──┐
                                                  │
                                                  ▼
                                          ┌──────────────┐
                                          │ Supabase Op  │
                                          │ (using ID)   │
                                          └──────────────┘
```

### **PromoStore (tanpa lookup):**

```
┌─────────────┐
│ User Request│
│ (id)        │
└──────┬──────┘
       │
       ▼
┌──────────────┐
│ Supabase Op  │
│ (using ID)   │
└──────────────┘
```

---

## 🎯 Key Differences

### **Why Promo & Store need lookup?**
- User tidak tahu/hafal ID internal
- `voucher_code` dan `route` lebih user-friendly
- Supabase requirement: UPDATE/DELETE harus pakai primary key (ID)

### **Why PromoStore direct ID?**
- Hanya 3 kolom (id, id_promo, id_store)
- Tidak ada unique identifier lain yang meaningful
- ID sudah cukup simple untuk user

---

## 📝 Example Requests

### **Promo - Update by Voucher Code**
```bash
PUT /update-promo/DISKON50
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "title_promo": "Diskon 50% Akhir Tahun",
  "is_active": true
}
```

**Backend Flow:**
1. Extract `voucher_code`: "DISKON50"
2. Lookup `id_promo` from cache/DB
3. Update Supabase: `eq("id_promo", id)`

---

### **Store - Delete by Route**
```bash
DELETE /delete-store/toko-elektronik-jakarta
Authorization: Bearer <jwt_token>
```

**Backend Flow:**
1. Extract `route`: "toko-elektronik-jakarta"
2. Lookup `id` from cache/DB
3. Delete from Supabase: `eq("id", id)`

---

### **PromoStore - Update by ID**
```bash
PUT /update-promo-store/123
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "id_promo": 456,
  "id_store": 789
}
```

**Backend Flow:**
1. Extract `id`: 123
2. Update Supabase: `eq("id", "123")` (no lookup)

---

## 🔒 Authentication

**Header Required:**
```
Authorization: Bearer <jwt_token>
```

**Dev Mode (skip auth):**
```env
MODE=dev
```

---

## ⚡ Performance Notes

- **Cache Hit Rate:** ~95% untuk read operations
- **Lookup Overhead:** ~50-100ms (hanya saat cache miss)
- **Direct ID Operations:** ~20-50ms (PromoStore)

---

**Built with ❤️ for optimal API design**
