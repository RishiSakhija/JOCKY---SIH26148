# Scripts

**Status**: PLACEHOLDER — Not implemented

---

## Purpose

Operational and development scripts for JOCKY.

---

## Planned Scripts

| Script | Purpose | Language |
|--------|---------|----------|
| `generate_test_data.rs` | Generate synthetic evidence for testing | Rust (binary) |
| `download_samples.sh` | Download public domain forensic datasets | Bash |
| `build_release.sh` | Cross-compile release binaries | Bash |
| `run_demo.sh` | Orchestrate SIH demo sequence | Bash |
| `verify_installation.sh` | Verify JOCKY installation | Bash |
| `migrate_db.rs` | Database schema migrations | Rust (binary) |

---

## Script Template

```bash
#!/usr/bin/env bash
# Script: <name>
# Purpose: <description>
# Usage: ./<name> [options]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Implementation here
```

---

## Current State

No scripts implemented yet. This directory exists for future organization.