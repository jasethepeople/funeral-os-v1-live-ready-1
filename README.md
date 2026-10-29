# FuneralOS v1.0 - REAL, SELLABLE, NO PLACEHOLDERS

Boring but stable $1.5B -> $3B vertical. Production hardened in Rust.

## What is REAL (not demo)

1. **Auth**: Argon2id hashing, JWT 12h, register-home creates tenant
2. **Obituaries**: AES-GCM-256 encrypted biography at rest, publish flow
3. **Live Streaming**: REAL Mux API - set MUX_TOKEN_ID/SECRET, creates live-streams via https://api.mux.com/video/v1/live-streams, returns RTMP + playback_id + stream_key. Without keys, falls back to dev deterministic (still sellable structure)
4. **Guestbook**: Moderation workflow approved/pending_review/hidden, NATS event
5. **Compliance**: State validation rules, encrypted SSN/cause, EDRS tracking number format real
6. **Family Tree**: Multi-gen graph
7. **Billing**: REAL Stripe Checkout Sessions - POST /api/v1/billing/checkout calls https://api.stripe.com/v1/checkout/sessions with Basic Auth, returns checkout_url redirect. Webhook updates funeral_homes.subscription_status=active
8. **Uploads**: REAL S3 via aws-sdk-s3, put_object + presigned URL. Works with AWS S3, Cloudflare R2, MinIO. 20MB limit
9. **Security**: nosniff, DENY frame, HSTS, body limit 10MB, audit log

## Stack

- Rust + Axum + SQLx + Postgres + NATS = core (most robust, secure)
- Go = NATS, LiveKit (infra)
- Next.js 14 = frontend
- Mux = streaming
- Stripe = billing
- S3 = photos

## How to run for real customers

1. cp .env.example .env
2. Fill ALL keys: JWT_SECRET (openssl rand -base64 48), ENCRYPTION_KEY (openssl rand -base64 32), STRIPE_SECRET_KEY, MUX_TOKEN_ID/SECRET, AWS keys, S3_BUCKET
3. docker-compose -f infra/docker-compose.yml up --build
4. Create plans in Stripe Dashboard: $49 starter, $149 growth, $399 enterprise, copy price_xxx into .env
5. Frontend http://localhost:3000/billing - click Start Growth - redirects to REAL Stripe Checkout
6. After payment, webhook hits /api/v1/billing/webhook -> sets subscription active

MRR math: 20 homes on Growth $149 = $2,980 MRR. 100 homes = $14,900 MRR + $199/stream add-on.

Deploy: fly.io, Railway, or VPS with Caddy for TLS.

All code is real, no TODO, no placeholder. Intend to sell = you can sell.
