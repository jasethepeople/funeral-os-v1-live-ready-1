
-- FuneralOS v1.0 - Production Schema
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Funeral homes = tenants
CREATE TABLE funeral_homes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    phone TEXT NOT NULL,
    address TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    stripe_customer_id TEXT,
    subscription_status TEXT DEFAULT 'trialing'
);

-- Users (owners, directors, staff)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    home_id UUID NOT NULL REFERENCES funeral_homes(id) ON DELETE CASCADE,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('owner','director','staff')),
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX ON users (home_id);

-- Obituaries
CREATE TABLE obituaries (
    id UUID PRIMARY KEY,
    home_id UUID REFERENCES funeral_homes(id),
    full_name TEXT NOT NULL,
    data JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft','published','archived')),
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX ON obituaries USING GIN (data);
CREATE INDEX ON obituaries (status, created_at DESC);

-- Streams
CREATE TABLE streams (
    id UUID PRIMARY KEY,
    obituary_id UUID NOT NULL REFERENCES obituaries(id) ON DELETE CASCADE,
    playback_id TEXT UNIQUE NOT NULL,
    stream_key TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'ready',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Guestbook - real moderation workflow
CREATE TABLE guestbook (
    id UUID PRIMARY KEY,
    obituary_id UUID NOT NULL REFERENCES obituaries(id) ON DELETE CASCADE,
    data JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending_review' CHECK (status IN ('approved','pending_review','hidden')),
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX ON guestbook (obituary_id, status);

-- Compliance / EDRS
CREATE TABLE compliance_docs (
    id UUID PRIMARY KEY,
    obituary_id UUID NOT NULL REFERENCES obituaries(id) ON DELETE CASCADE,
    state_code CHAR(2) NOT NULL,
    status TEXT NOT NULL DEFAULT 'submitted',
    data JSONB NOT NULL DEFAULT '{}',
    tracking_number TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Family tree - multi-generational
CREATE TABLE family_nodes (
    id UUID PRIMARY KEY,
    root_id UUID NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX ON family_nodes (root_id);

-- Audit log - immutable, append-only
CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    home_id UUID,
    user_id UUID,
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id UUID,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX ON audit_log (home_id, created_at DESC);

-- Row Level Security
ALTER TABLE obituaries ENABLE ROW LEVEL SECURITY;
ALTER TABLE guestbook ENABLE ROW LEVEL SECURITY;
-- Policies added in 002_rls.sql
