# Agents Instructions

See `CLAUDE.md` for project workflow, specs, and contribution guidelines.

## Cursor Cloud specific instructions

### Environment Overview

SkillLite is a Rust workspace (Cargo) with a Python SDK bridge. No Docker, no external databases — entirely self-contained with embedded SQLite.

### Key Services

| Service | How to run | Notes |
|---------|-----------|-------|
| Rust binary (CLI) | `cargo build -p skilllite` then `./target/debug/skilllite` | Main dev artifact |
| Python SDK | `pip install -e python-sdk` | Thin wrapper calling the Rust binary |

### Running the Sandbox (Linux gotcha)

The default `RLIMIT_NPROC` (50) is too low in Cloud Agent VMs where ~40+ processes already run under the same UID. Set `SKILLLITE_MAX_PROCESSES=200` (or higher) when running skills with sandbox level 2 or 3:

```bash
SKILLLITE_MAX_PROCESSES=200 ./target/debug/skilllite run .skills/calculator '{"operation":"add","a":1,"b":2}'
```

Without this, bwrap fails with "Creating new namespace failed: Resource temporarily unavailable".

### Lint / Test / Build Commands

- **Rust format check**: `cargo fmt --check`
- **Rust lint**: `cargo clippy --all-targets -- -D warnings`
- **Rust tests**: `cargo test`
- **Rust build**: `cargo build` (debug) or `cargo build --release`
- **Python lint**: `ruff check python-sdk/`
- **Python tests**: `cd python-sdk && pytest`

See `docs/en/CONTRIBUTING.md` for full PR checklist.

### Rust Toolchain

This project requires Rust 1.85+ (edition 2024 support). The update script runs `rustup update stable` to ensure this. If `rustc --version` shows < 1.85, run `rustup update stable && rustup default stable`.

### Sandbox Binary (bwrap)

`bubblewrap` must be installed for sandbox tests (`sudo apt-get install -y bubblewrap`). Without it, only `SKILLLITE_SANDBOX_LEVEL=1` (no sandbox) works.

### Python venv

`python3-venv` (or `python3.12-venv`) is required for skill environment setup. Skills with dependencies create virtualenvs via `python3 -m venv`. Install with `sudo apt-get install -y python3-venv`.

### Non-TTY skill execution

When running skills with Level 3 (scan+confirm), set `SKILLLITE_AUTO_APPROVE=1` to skip the interactive confirmation prompt in non-TTY environments (CI, Cloud Agents).

### LLM API (optional for most dev work)

Agent/chat features need `BASE_URL`, `API_KEY`, `MODEL` env vars (see `.env.example`). Unit tests and sandbox execution do NOT require an LLM key.
