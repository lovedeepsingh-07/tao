CREATE TABLE IF NOT EXISTS project (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	slug TEXT UNIQUE NOT NULL,
	name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS report (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	project_id UUID NOT NULL,

	reported_at timestamptz NOT NULL,
	received_at timestamptz NOT NULL DEFAULT NOW(),

	release TEXT,

	body TEXT NOT NULL,
	stack_trace TEXT,
	kind TEXT CHECK (kind IN ('DEBUG', 'INFO', 'WARN', 'ERROR')) NOT NULL,

	memory_usage INT,
	cpu_percent FLOAT,
	disk_usage_percent FLOAT,

	CONSTRAINT FOREIGN_KEY_PROJECT FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);

CREATE INDEX index_reported_at ON report(reported_at);
CREATE INDEX index_received_at ON report(received_at);
