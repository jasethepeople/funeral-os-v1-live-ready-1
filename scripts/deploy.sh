#!/bin/bash
set -e

echo "🚀 FuneralOS Production Deploy"

if ! command -v flyctl &> /dev/null; then
  echo "Installing flyctl..."
  curl -L https://fly.io/install.sh | sh
fi

echo "Setting up backend..."
cd "$(dirname "$0")/.."
flyctl postgres create --name funeral-os-db --region scl --vm-size shared-cpu-1x --volume-size 1 --initial-cluster-size 1 || true
flyctl secrets set JWT_SECRET=$(openssl rand -base64 48) ENCRYPTION_KEY=$(openssl rand -base64 32) RUST_LOG=info ENV=production -a funeral-os || true

echo "Deploying backend..."
flyctl deploy --config fly.toml -a funeral-os

echo "Deploying frontend..."
cd frontend
flyctl deploy --config fly.toml -a funeral-os-frontend

echo "✅ Live!"
echo "API: https://funeral-os.fly.dev/health"
echo "Frontend: https://funeral-os-frontend.fly.dev"
echo "Billing: https://funeral-os-frontend.fly.dev/billing"
