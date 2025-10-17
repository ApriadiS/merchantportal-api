# ☁️ AWS EC2 Deployment Guide (ARM64 Graviton)

## 🎯 Kenapa ARM64 Graviton?

- **40% lebih murah** dibanding x86_64
- **20% performa lebih baik** untuk workload tertentu
- **Efisiensi energi** lebih tinggi
- **Perfect untuk Rust** - native ARM64 support

## 📋 Prerequisites

- AWS Account
- EC2 instance type: **t4g.micro** (free tier) atau **t4g.small** (recommended)
- Security Group: Allow port 80, 443, 22

## 🚀 Step-by-Step Setup

### 1. Launch EC2 Instance

```bash
# Instance Type: t4g.micro atau t4g.small (ARM64)
# AMI: Amazon Linux 2023 (ARM64)
# Storage: 10GB gp3
# Security Group: Allow SSH (22), HTTP (80), HTTPS (443)
```

### 2. Connect to EC2

```bash
ssh -i your-key.pem ec2-user@your-ec2-public-ip
```

### 3. Install Docker

```bash
# Update system
sudo yum update -y

# Install Docker
sudo yum install docker -y

# Start Docker service
sudo systemctl start docker
sudo systemctl enable docker

# Add user to docker group
sudo usermod -a -G docker ec2-user

# Logout and login again for group changes
exit
# SSH again
ssh -i your-key.pem ec2-user@your-ec2-public-ip

# Verify Docker
docker --version
```

### 4. Install Git

```bash
sudo yum install git -y
```

### 5. Clone Repository

```bash
cd ~
git clone https://github.com/your-username/merchantportal-api.git
cd merchantportal-api
```

### 6. Setup Environment Variables

```bash
# Copy template
cp .env.example .env

# Edit with your credentials
nano .env
```

Isi dengan:
```env
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_KEY=your-actual-key
JWT_SECRET=your-actual-secret
RUST_LOG=info
MODE=prod
RUST_BACKTRACE=0
```

**Save:** `Ctrl+O`, Enter, `Ctrl+X`

### 7. Deploy Application

```bash
# Make script executable
chmod +x deploy-arm64.sh

# Deploy
./deploy-arm64.sh
```

### 8. Verify Deployment

```bash
# Check container status
docker ps

# Check logs
docker logs -f merchantportal-api

# Test API
curl http://localhost:3000/get-promo
```

### 9. Setup Nginx Reverse Proxy (Optional)

```bash
# Install Nginx
sudo yum install nginx -y

# Create config
sudo nano /etc/nginx/conf.d/api.conf
```

Paste:
```nginx
server {
    listen 80;
    server_name your-domain.com;  # Ganti dengan domain Anda

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

```bash
# Start Nginx
sudo systemctl start nginx
sudo systemctl enable nginx

# Test
curl http://your-ec2-public-ip
```

### 10. Setup SSL with Let's Encrypt (Optional)

```bash
# Install certbot
sudo yum install certbot python3-certbot-nginx -y

# Get certificate
sudo certbot --nginx -d your-domain.com

# Auto-renewal
sudo systemctl enable certbot-renew.timer
```

## 🔄 Update Application

```bash
cd ~/merchantportal-api

# Pull latest changes
git pull origin main

# Redeploy
./deploy-arm64.sh
```

## 📊 Monitoring

### Check Container Status
```bash
docker ps
docker stats merchantportal-api
```

### View Logs
```bash
# Real-time logs
docker logs -f merchantportal-api

# Last 100 lines
docker logs --tail 100 merchantportal-api
```

### Check Memory Usage
```bash
free -h
docker stats --no-stream
```

### Check Disk Usage
```bash
df -h
docker system df
```

## 🛑 Stop/Restart Application

```bash
# Stop
docker stop merchantportal-api

# Start
docker start merchantportal-api

# Restart
docker restart merchantportal-api

# Remove and redeploy
docker stop merchantportal-api
docker rm merchantportal-api
./deploy-arm64.sh
```

## 🔒 Security Best Practices

### 1. Firewall Configuration
```bash
# Only allow necessary ports
# In AWS Security Group:
# - SSH (22): Your IP only
# - HTTP (80): 0.0.0.0/0
# - HTTPS (443): 0.0.0.0/0
```

### 2. Disable Root Login
```bash
sudo nano /etc/ssh/sshd_config
# Set: PermitRootLogin no
sudo systemctl restart sshd
```

### 3. Setup Fail2Ban
```bash
sudo yum install fail2ban -y
sudo systemctl start fail2ban
sudo systemctl enable fail2ban
```

### 4. Regular Updates
```bash
# Create update script
cat > ~/update-system.sh << 'EOF'
#!/bin/bash
sudo yum update -y
docker system prune -f
EOF

chmod +x ~/update-system.sh

# Run weekly via cron
crontab -e
# Add: 0 2 * * 0 /home/ec2-user/update-system.sh
```

## 💰 Cost Estimation

| Instance Type | vCPU | RAM | Price/Month | Use Case |
|--------------|------|-----|-------------|----------|
| t4g.micro    | 2    | 1GB | ~$6         | Testing, low traffic |
| t4g.small    | 2    | 2GB | ~$12        | Production, 1-2K users |
| t4g.medium   | 2    | 4GB | ~$24        | High traffic, 3-5K users |

**Note:** Prices are approximate for us-east-1 region

## 🐛 Troubleshooting

### Problem: Docker permission denied
```bash
# Add user to docker group
sudo usermod -a -G docker $USER
# Logout and login again
```

### Problem: Port 3000 already in use
```bash
# Find process
sudo lsof -i :3000
# Kill process
sudo kill -9 <PID>
```

### Problem: Out of memory
```bash
# Check memory
free -h
# Upgrade to larger instance or add swap
sudo dd if=/dev/zero of=/swapfile bs=1M count=1024
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

### Problem: Container keeps restarting
```bash
# Check logs
docker logs merchantportal-api
# Check ENV variables
docker exec merchantportal-api env | grep SUPABASE
```

## 📈 Performance Tuning

### 1. Increase File Descriptors
```bash
sudo nano /etc/security/limits.conf
# Add:
* soft nofile 65536
* hard nofile 65536
```

### 2. Optimize Kernel Parameters
```bash
sudo nano /etc/sysctl.conf
# Add:
net.core.somaxconn = 1024
net.ipv4.tcp_max_syn_backlog = 2048
```

### 3. Docker Resource Limits
```bash
docker run -d \
  --name merchantportal-api \
  --memory="512m" \
  --cpus="1.5" \
  -p 3000:3000 \
  --restart unless-stopped \
  --env-file .env \
  merchantportal-api:arm64
```

## 🎯 Next Steps

- [ ] Setup CloudWatch monitoring
- [ ] Configure Auto Scaling
- [ ] Setup Load Balancer (for multiple instances)
- [ ] Implement CI/CD with GitHub Actions
- [ ] Setup backup strategy
- [ ] Configure CloudFront CDN

---

**Built with ❤️ for AWS EC2 Graviton (ARM64)**
