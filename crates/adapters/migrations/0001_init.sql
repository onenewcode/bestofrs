CREATE TABLE IF NOT EXISTS projects (
  id                   TEXT PRIMARY KEY NOT NULL,
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
  updated_at           TEXT,
  repo_id              TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_projects_repo_id ON projects(repo_id);

CREATE TABLE IF NOT EXISTS repos (
  id             TEXT PRIMARY KEY NOT NULL,
  github_repo_id INTEGER,
  full_name      TEXT,
  stars          INTEGER NOT NULL DEFAULT 0,
  forks          INTEGER NOT NULL DEFAULT 0,
  open_issues    INTEGER NOT NULL DEFAULT 0,
  watchers       INTEGER NOT NULL DEFAULT 0,
  last_fetched_at TEXT,
  etag           TEXT,
  created_at     TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at     TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_repos_github_repo_id ON repos(github_repo_id);
CREATE INDEX IF NOT EXISTS idx_repos_full_name ON repos(full_name);

CREATE TABLE IF NOT EXISTS snapshots (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  repo_id       TEXT NOT NULL,
  snapshot_date TEXT NOT NULL,
  stars         INTEGER NOT NULL,
  forks         INTEGER NOT NULL,
  open_issues   INTEGER NOT NULL,
  watchers      INTEGER NOT NULL,
  fetched_at    TEXT NOT NULL,
  created_at    TEXT NOT NULL DEFAULT (datetime('now')),
  FOREIGN KEY(repo_id) REFERENCES repos(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_snapshots_repo_day ON snapshots(repo_id, snapshot_date);
CREATE INDEX IF NOT EXISTS idx_snapshots_repo_id ON snapshots(repo_id);
CREATE INDEX IF NOT EXISTS idx_snapshots_snapshot_date ON snapshots(snapshot_date);
