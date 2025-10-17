# 📦 Models

Domain models dan DTOs (Data Transfer Objects).

## 📋 Files

### **promo_model.rs**
```rust
// Domain entity
Promo {
    id_promo: i64,
    voucher_code: String,
    title_promo: String,
    // ... other fields
}

// DTOs
CreatePromoPayload
UpdatePromoPayload
AdminPromoType (enum)
```

### **store_model.rs**
```rust
// Domain entity
Store {
    id: i64,
    route: String,
    name: String,
    // ... other fields
}

// DTOs
CreateStorePayload
UpdateStorePayload
UpdateStorePayloadWithID
DeleteStorePayload
StoreType (enum)
```

### **promo_store_model.rs**
```rust
// Domain entity
PromoStore {
    id: u32,
    id_promo: i64,
    id_store: i64,
}

// DTOs
CreatePromoStorePayload
UpdatePromoStorePayload
DeletePromoStorePayload
```

## 🔑 Responsibilities

- Define data structures
- Serialization/Deserialization (Serde)
- Type safety
- Validation rules
