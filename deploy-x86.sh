#!/bin/bash
# Deploy script for x86_64 (Intel/AMD)

set -e

echo "🚀 Deploying Merchant Portal API (x86_64)..."

# Check if .env exists
if [ ! -f .env ]; then
    echo "❌ Error: .env file not found!"
    echo "📝 Please copy .env.example to .env and fill in your credentials"
    exit 1
fi

# Stop and remove existing container
echo "🛑 Stopping existing container..."
docker-compose -f docker-compose.x86_64.yml down 2>&1 | while IFS= read -r line; do
    echo "   $line"
done || true
echo "✅ Container stopped!"

# Build image first (separate from docker-compose to save memory)
echo "🔨 Building Docker image..."
echo "⏳ This may take a few minutes..."
docker build -f Dockerfile.x86_64 -t merchantportal-api-x86 . 2>&1 | while IFS= read -r line; do
    echo "   $line"
done
echo "✅ Image built successfully!"

# Start container with docker-compose
echo ""
echo "🚀 Starting container..."
docker-compose -f docker-compose.x86_64.yml up -d 2>&1 | while IFS= read -r line; do
    echo "   $line"
done
echo "✅ Container started successfully!"

echo "✅ Deployment complete!"
echo "📊 Container status:"
docker ps -f name=merchantportal-api

echo ""
echo "📝 View logs with: docker logs -f merchantportal-api"
echo "🔍 Test API with: curl http://localhost:3000/get-promo"
