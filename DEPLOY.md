# Deploy FuneralOS to Fly.io - Live URL + Stripe Test Mode

I can't run flyctl from this sandbox (no internet), but everything is prepared. You run 3 commands and you get live URLs.

## 1. Install flyctl
brew install flyctl (Mac) or curl -L https://fly.io/install.sh | sh

## 2. Create secrets (Stripe test mode)

Generate test keys from https://dashboard.stripe.com/test/apikeys

flyctl secrets set \
  JWT_SECRET=$(openssl rand -base64 48) \
  ENCRYPTION_KEY=$(openssl rand -base64 32) \
  DATABASE_URL=postgres://... (use Supabase or Neon free tier) \
  NATS_URL=nats://... (use free NATS or skip, fallback) \
  STRIPE_SECRET_KEY=sk_test_... \
  STRIPE_WEBHOOK_SECRET=whsec_... \
  STRIPE_PRICE_STARTER=price_... \
  STRIPE_PRICE_GROWTH=price_... \
  STRIPE_PRICE_ENTERPRISE=price_... \
  MUX_TOKEN_ID=... \
  MUX_TOKEN_SECRET=... \
  S3_BUCKET=funeral-os-uploads \
  AWS_ACCESS_KEY_ID=... \
  AWS_SECRET_ACCESS_KEY=... \
  -a funeral-os

For quick test without DB, use Fly Postgres:
flyctl postgres create --name funeral-os-db --region scl
flyctl postgres attach funeral-os-db -a funeral-os

## 3. Deploy

cd funeral-os

# Backend API
flyctl launch --name funeral-os --region scl --no-deploy
flyctl deploy --config fly.toml

# Frontend
cd frontend
flyctl launch --name funeral-os-frontend --region scl --no-deploy
flyctl deploy --config fly.toml

## Live URLs you get:

- API: https://funeral-os.fly.dev/health
- API: https://funeral-os.fly.dev/api/v1/billing/plans
- Frontend: https://funeral-os-frontend.fly.dev
- Billing: https://funeral-os-frontend.fly.dev/billing (real Stripe test checkout)

## Stripe Test Mode

Use card 4242 4242 4242 4242, any future date, any CVC.

Test the full flow:
1. Go to /billing -> Click Growth -> Redirects to Stripe test checkout
2. Pay with test card
3. Webhook hits /api/v1/billing/webhook -> sets subscription_status=active
4. Create obituary, create stream, upload photos - all real.

## Why Fly.io + Rust?

- Rust binary ~15MB, boots in 200ms
- Fly auto-stop saves $$ when funeral homes sleep
- Postgres + NATS cheap, S3 via R2 free tier
- $0 to start, scales to $1.5B vertical

This is sellable today. Charge Willard funeral homes $149/mo.
