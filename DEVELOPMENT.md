# Development Guide

This document describes the verified, step-by-step workflow for developing **Best Of RS**.

## 1. Prerequisites

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Dioxus CLI**: `cargo install dioxus-cli`
- **Podman**: Required for Redis (session storage)
- **uv**: `pip install uv` (for setup scripts)

## 2. Environment Configuration

All settings are managed via `crates/infra/src/config/toml/development.toml`. Ensure:
- `[database].url = "sqlite://bestofrs.db"`
- `[redis].url = "redis://127.0.0.1:6379"`

## 3. Verified Startup Workflow

### Step 1: Start Redis
The application requires Redis to handle sessions.
```powershell
podman run -d --name redis-bestofrs -p 127.0.0.1:6379:6379 redis
```

### Step 2: Run Server (Initialization)
Run the following to initialize the database schema and start the service.
```powershell
cd crates/ui
dx serve --platform server
```
*Wait for: `🚀 Server running on :127.0.0.1:8080`.*

### Step 3: Initial API Check
Before inserting data, verify that the server is responding correctly.
```powershell
# Run from project root in a new terminal
uv run scripts/api_requests.py
```
**Expected Output**: `✅ Server is UP` followed by `ℹ️  API is working, but the database is currently EMPTY.`

### Step 4: Populate Data
Now that the server is healthy, insert the mock data.
```powershell
uv run scripts/insert_data.py
```

### Step 5: Final API Verification
Run the API script again to confirm data is correctly returned.
```powershell
uv run scripts/api_requests.py
```
**Expected Output**: `✨ API is working! Found X items in database.`

## Troubleshooting

- **500 Internal Server Error**: Check if Redis is running and that you haven't run `insert_data.py` *before* Step 2.
- **Port Conflict**: Kill any process on port 8080 or update `development.toml` and `Dioxus.toml`.
