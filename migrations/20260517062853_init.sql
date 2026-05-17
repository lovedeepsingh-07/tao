CREATE TABLE IF NOT EXISTS report (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	created_at timestamptz NOT NULL DEFAULT NOW(),
	body TEXT NOT NULL,
	kind TEXT CHECK (kind IN ('debug', 'info', 'warn', 'error')) NOT NULL
);
