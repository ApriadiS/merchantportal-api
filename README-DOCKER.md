# 🐳 Docker Deployment Guide

## 📋 Arsitektur yang Didukung

Project ini menyediakan 2 Dockerfile untuk arsitektur berbeda:

- **Dockerfile.arm64** - Untuk AWS EC2 Graviton (ARM64), Apple Silicon (M1/M2/M3)
- **Dockerfile.x86_64** - Untuk server Intel/AMD standar

## 🔐 Setup Environment Variables

### 1. Copy template .env
```bash
cp .env.example .env
```

### 2. Edit .env dengan credentials Anda
```bash
nano .env
```

Isi dengan nilai yang sebenarnya:
```env
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_KEY=your-actual-supabase-key
JWT_SECRET=your-actual-jwt-secret
RUST_LOG=info
MODE=prod
RUST_BACKTRACE=0
```

### 3. Pastikan .env tidak di-commit
File `.env` sudah ada di `.gitignore`. Jangan pernah commit file ini!

## 🚀 Cara Menjalankan

### Opsi 1: Docker Compose (Recommended)

**ARM64:**
```bash
docker-compose -f docker-compose.arm64.yml up -d
```

**x86_64:**
```bash
docker-compose -f docker-compose.x86_64.yml up -d
```

### Opsi 2: Docker Build Manual

**ARM64:**
```bash
# Build
docker build -f Dockerfile.arm64 -t merchantportal-api:arm64 .

# Run dengan ENV injection
docker run -d \
  --name merchantportal-api \
  -p 3000:3000 \
  -e SUPABASE_URL="${SUPABASE_URL}" \
  -e SUPABASE_KEY="${SUPABASE_KEY}" \
  -e JWT_SECRET="${JWT_SECRET}" \
  -e RUST_LOG="${RUST_LOG:-info}" \
  -e MODE="${MODE:-prod}" \
  merchantportal-api:arm64
```

**x86_64:**
```bash
# Build
docker build -f Dockerfile.x86_64 -t merchantportal-api:x86 .

# Run dengan ENV injection
docker run -d \
  --name merchantportal-api \
  -p 3000:3000 \
  -e SUPABASE_URL="${SUPABASE_URL}" \
  -e SUPABASE_KEY="${SUPABASE_KEY}" \
  -e JWT_SECRET="${JWT_SECRET}" \
  -e RUST_LOG="${RUST_LOG:-info}" \
  -e MODE="${MODE:-prod}" \
  merchantportal-api:x86
```

## 🔍 Verifikasi

```bash
# Check logs
docker logs merchantportal-api

# Check health
curl http://localhost:3000/get-promo

# Check container status
docker ps
```

## 🛑 Stop & Remove

```bash
# Stop
docker stop merchantportal-api

# Remove
docker rm merchantportal-api

# Remove image
docker rmi merchantportal-api:arm64
# atau
docker rmi merchantportal-api:x86
```

## 📦 Deploy ke AWS EC2 (ARM64)

### 1. SSH ke EC2 instance
```bash
ssh -i your-key.pem ec2-user@your-ec2-ip
```

### 2. Install Docker
```bash
sudo yum update -y
sudo yum install docker -y
sudo systemctl start docker
sudo systemctl enable docker
sudo usermod -a -G docker ec2-user
```

### 3. Clone repository
```bash
git clone https://github.com/your-username/merchantportal-api.git
cd merchantportal-api
```

### 4. Setup environment
```bash
cp .env.example .env
nano .env  # Edit dengan credentials Anda
```

### 5. Build & Run
```bash
docker build -f Dockerfile.arm64 -t merchantportal-api:arm64 .

docker run -d \
  --name merchantportal-api \
  -p 3000:3000 \
  --restart unless-stopped \
  --env-file .env \
  merchantportal-api:arm64
```

### 6. Setup Nginx (Optional)
```bash
sudo yum install nginx -y
sudo nano /etc/nginx/conf.d/api.conf
```

Tambahkan:
```nginx
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

```bash
sudo systemctl start nginx
sudo systemctl enable nginx
```

## 🔒 Security Best Practices

1. ✅ **Jangan commit .env** - Sudah ada di .gitignore
2. ✅ **Gunakan .env.example** - Untuk dokumentasi
3. ✅ **Inject ENV saat runtime** - Bukan hardcode di Dockerfile
4. ✅ **Rotate secrets regularly** - Ganti JWT_SECRET & API keys secara berkala
5. ✅ **Use AWS Secrets Manager** - Untuk production (optional)

## 📊 Image Size Comparison

| Architecture | Image Size | Build Time |
|-------------|-----------|------------|
| ARM64       | ~25 MB    | ~3-5 min   |
| x86_64      | ~25 MB    | ~3-5 min   |

## 🐛 Troubleshooting

**Problem: Binary not found**
```bash
# Check binary location in builder
docker build --target builder -f Dockerfile.arm64 -t test .
docker run --rm test find /usr/src/app -name merchantportal-api
```

**Problem: ENV not loaded**
```bash
# Check ENV inside container
docker exec merchantportal-api env | grep SUPABASE
```

**Problem: Connection refused**
```bash
# Check if app is listening
docker exec merchantportal-api netstat -tlnp | grep 3000
```

---

**Built with ❤️ using Rust + Docker + Alpine**
