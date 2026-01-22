-- Project identity is repo_id (owner/name). Enforce it at the database level.
--
-- SQLite cannot change PRIMARY KEY constraints in-place, so we rebuild the table.

CREATE TABLE projects_new (
  repo_id              TEXT PRIMARY KEY NOT NULL,
  name                 TEXT NOT NULL UNIQUE,
  slug                 TEXT NOT NULL UNIQUE,
  description          TEXT NOT NULL,
  override_description INTEGER NOT NULL DEFAULT 0,
  url                  TEXT,
  override_url         INTEGER NOT NULL DEFAULT 0,
  status               TEXT,
  logo                 TEXT,
  twitter              TEXT,
  comments             TEXT,
  created_at           TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at           TEXT
);

INSERT INTO projects_new (
  repo_id,
  name, slug, description,
  override_description,
  url, override_url,
  status, logo, twitter, comments,
  created_at, updated_at
)
SELECT
  repo_id,
  name, slug, description,
  override_description,
  url, override_url,
  status, logo, twitter, comments,
  created_at, updated_at
FROM projects;

DROP TABLE projects;
ALTER TABLE projects_new RENAME TO projects;
