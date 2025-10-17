# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2024-01-XX

### Added
- ✅ Health check endpoint (`/health`)
- ✅ Metrics endpoint (`/metrics`) - cache statistics
- ✅ Response compression (gzip)
- ✅ Rate limiting (100 req/s with axum-governor)
- ✅ Request body size limit (1MB)
- ✅ JWT authentication with caching
- ✅ In-memory caching with RwLock
- ✅ Cache warming on startup
- ✅ Multi-architecture Docker support (ARM64 & x86_64)
- ✅ CRUD operations for Promo, Store, PromoStore
- ✅ Unique identifier mapping (voucher_code, route → ID)
- ✅ Comprehensive documentation

### Features
- **Promo Management**: CRUD with voucher_code as key
- **Store Management**: CRUD with route as key
- **PromoStore Management**: CRUD with ID as key
- **Performance**: 6,318 req/s, 0% error rate, ~60MB memory
- **Concurrency**: Handles 1,500-2,000 concurrent users

### Infrastructure
- Docker multi-stage builds (Alpine-based)
- Docker Compose for ARM64 and x86_64
- Deployment scripts for both architectures
- AWS EC2 Graviton deployment guide

### Documentation
- README.md - Project overview
- README-DOCKER.md - Docker deployment
- AWS-EC2-SETUP.md - AWS deployment guide
- CRUD-FLOW.md - CRUD operation flows
- API-ENDPOINTS.md - Complete API reference
- QUICK-START.md - Quick start guide

### Performance
- JWT caching with serialized claims
- In-memory data caching (promo, store, promo_store)
- Tokio multi-thread (2 workers)
- Optimal for 2 vCPU, 1GB RAM

### Security
- JWT authentication
- Environment variable injection at runtime
- No hardcoded secrets in Docker images
- Rate limiting protection

## [Unreleased]

### Planned
- [ ] Prometheus metrics integration
- [ ] External cache (Redis) for horizontal scaling
- [ ] Background cache refresh
- [ ] Graceful shutdown
- [ ] CORS configuration
- [ ] Circuit breaker pattern
- [ ] CDN integration

---

**Format:** [Semantic Versioning](https://semver.org/)
