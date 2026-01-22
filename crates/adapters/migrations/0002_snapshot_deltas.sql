CREATE TABLE IF NOT EXISTS snapshot_deltas (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  repo_id           TEXT NOT NULL,
  snapshot_date     TEXT NOT NULL,

  prev_snapshot_date TEXT,

  stars_delta       INTEGER,
  forks_delta       INTEGER,
  open_issues_delta INTEGER,
  watchers_delta    INTEGER,

  created_at        TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at        TEXT NOT NULL DEFAULT (datetime('now')),

  FOREIGN KEY(repo_id) REFERENCES repos(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_snapshot_deltas_repo_day
  ON snapshot_deltas(repo_id, snapshot_date);

CREATE INDEX IF NOT EXISTS idx_snapshot_deltas_repo_id
  ON snapshot_deltas(repo_id);

CREATE INDEX IF NOT EXISTS idx_snapshot_deltas_snapshot_date
  ON snapshot_deltas(snapshot_date);
