CREATE TABLE IF NOT EXISTS project (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS report (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	project_id UUID NOT NULL,

	reported_at timestamptz NOT NULL,
	received_at timestamptz NOT NULL DEFAULT NOW(),

	body TEXT NOT NULL,
	location TEXT NOT NULL,
	level TEXT CHECK (level IN ('TRACE', 'DEBUG', 'INFO', 'WARN', 'ERROR')) NOT NULL,

	used_memory INT,
	total_memory INT,
	cpu_percent FLOAT,

	CONSTRAINT FOREIGN_KEY_PROJECT FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);

CREATE INDEX index_reported_at ON report(reported_at);
CREATE INDEX index_received_at ON report(received_at);
