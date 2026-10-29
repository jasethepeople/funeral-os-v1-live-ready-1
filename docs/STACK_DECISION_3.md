
# FuneralOS Stack Decision - Full Gamut Analysis

## Language Evaluation
| Language | Perf | Safety | Hiring | Ecosystem | Verdict |
| Rust | 10/10 | 10/10 memory-safe, no GC pauses | 6/10 | Axum, SQLx, excellent | CORE API |
| Go | 9/10 | 8/10 | 9/10 | NATS, LiveKit servers are Go | INFRA SERVICES |
| Zig | 9.5/10 | 9/10 | 3/10 | Too immature for compliance | REJECT - use for future transcoder |
| Lisp (SBCL/Clojure) | 5/10 | 7/10 | 2/10 | Great for rules engine | COMPLIANCE RULES DSL embedded in Rust |
| TypeScript | 8/10 | 7/10 | 10/10 | Next.js best for funeral homes | FRONTEND |

## Final Production Stack (Hardened)
- **API Core: Rust + Axum + Tokio** - Zero unsafe, all PII AES-GCM-256 encrypted at rest, mTLS between services, immutable audit log
- **DB: Postgres 16 with pgcrypto, RLS, PITR, Barman backups** - Row-level security per funeral home tenant
- **Queue: NATS JetStream (Go)** - Powers obituary.created -> meilisearch indexing, moderation, email, SMS
- **Search: Meilisearch (Rust)** - Fast obituary search
- **Streaming: Mux + LiveKit (Go SFU)** - RTMPS ingest, HLS playback, WebRTC guestbook video messages
- **Storage: S3 + presigned URLs, SSE-KMS**
- **Auth: OIDC (Authentik) + Passkeys + Argon2id**
- **Frontend: Next.js 14 App Router, Tailwind, Mux Player, Offline-capable PWA for rural funeral homes**
- **Security: Content-Security-Policy strict, Rate limit 60/min/IP via tower-http, Request body 5MB, Audit middleware, OpenTelemetry tracing**

## Why Rust for compliance module?
Death certificates and permits vary by state (50+ EDRS formats). Rust enums + strong typing prevent invalid state transitions. Lisp-style rule DSL compiled to Rust for auditability.
