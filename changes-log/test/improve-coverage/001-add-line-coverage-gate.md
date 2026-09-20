# 001 — Add Line Coverage Gate

**Date**: 2026-09-21
**Tool**: GitHub Copilot

## Overview

Added stable `cargo-llvm-cov` line coverage enforcement with an 85% minimum to local pre-commit, Rust CI, and release verification.

## Affected Areas

- `.pre-commit-config.yaml`
- `.github/workflows/rust.yml`
- `README.md`, `AGENTS.md`, and `.github/copilot-instructions.md`
- Paired diary and changes-log entries

## Validation

- `cargo llvm-cov --workspace --all-features --all-targets --summary-only --fail-under-lines 85`
