# ⚡ Quick Start Guide

## 🚀 Local Development

```bash
# 1. Clone repository
git clone https://github.com/your-username/merchantportal-api.git
cd merchantportal-api

# 2. Setup environment
cp .env.example .env
nano .env  # Edit dengan credentials Anda

# 3. Run dengan Docker
chmod +x deploy-arm64.sh  # untuk ARM64
# atau
chmod +x deploy-x86.sh    # untuk x86_64

./deploy-arm64.sh  # jalankan sesuai arsitektur

# 4. Test
curl http://localhost:3000/get-promo
```

## ☁️ AWS EC2 Deployment (ARM64)

```bash
# 1. SSH ke EC2
ssh -i your-key.pem ec2-user@your-ec2-ip

# 2. Install Docker
sudo yum update -y
sudo yum install docker git -y
sudo systemctl start docker
sudo usermod -a -G docker ec2-user
# Logout & login lagi

# 3. Clone & Deploy
git clone https://github.com/your-username/merchantportal-api.git
cd merchantportal-api
cp .env.example .env
nano .env  # Edit credentials
chmod +x deploy-arm64.sh
./deploy-arm64.sh

# 4. Verify
docker ps
docker logs -f merchantportal-api
```

## 📝 Environment Variables

```env
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_KEY=your-supabase-key
JWT_SECRET=your-jwt-secret
RUST_LOG=info
MODE=prod
RUST_BACKTRACE=0
```

## 🔧 Common Commands

```bash
# View logs
docker logs -f merchantportal-api

# Restart
docker restart merchantportal-api

# Stop
docker stop merchantportal-api

# Update & redeploy
git pull origin main
./deploy-arm64.sh

# Check status
docker ps
docker stats merchantportal-api
```

## 📚 Documentation

- [README.md](README.md) - Project overview & performance
- [README-DOCKER.md](README-DOCKER.md) - Docker deployment details
- [AWS-EC2-SETUP.md](AWS-EC2-SETUP.md) - AWS EC2 setup guide

## 🆘 Troubleshooting

**Container won't start:**
```bash
docker logs merchantportal-api
```

**Port already in use:**
```bash
sudo lsof -i :3000
sudo kill -9 <PID>
```

**ENV not loaded:**
```bash
docker exec merchantportal-api env | grep SUPABASE
```

## 🎯 Architecture Support

| Platform | Dockerfile | Deploy Script |
|----------|-----------|---------------|
| AWS EC2 Graviton | Dockerfile.arm64 | deploy-arm64.sh |
| Apple Silicon (M1/M2/M3) | Dockerfile.arm64 | deploy-arm64.sh |
| Intel/AMD Servers | Dockerfile.x86_64 | deploy-x86.sh |

---

**Need help?** Check the full documentation or open an issue on GitHub.
