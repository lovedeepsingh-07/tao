CREATE TABLE IF NOT EXISTS report (
	id UUID PRIMARY KEY,
	body TEXT NOT NULL,
	kind TEXT CHECK (kind IN ('debug', 'info', 'warn', 'error')) NOT NULL
);
