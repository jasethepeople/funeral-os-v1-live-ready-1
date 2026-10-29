
CREATE TABLE billing_sessions (
    id UUID PRIMARY KEY,
    home_id UUID NOT NULL REFERENCES funeral_homes(id),
    stripe_session_id TEXT UNIQUE NOT NULL,
    plan TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE streams ADD COLUMN IF NOT EXISTS mux_id TEXT;
ALTER TABLE streams ADD COLUMN IF NOT EXISTS ended_at TIMESTAMPTZ;
ALTER TABLE streams ADD COLUMN IF NOT EXISTS stream_key TEXT;

CREATE TABLE uploads (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    home_id UUID REFERENCES funeral_homes(id),
    obituary_id UUID REFERENCES obituaries(id),
    s3_key TEXT NOT NULL,
    s3_url TEXT NOT NULL,
    file_size INT,
    content_type TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
