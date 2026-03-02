# Task: Set Up CI Workflow

## Status
[ ] Pending

## Description

Create GitHub Actions CI workflow for automated checks.

## Requirements

1. Run on push to main and pull requests
2. Check formatting: cargo fmt --check
3. Run clippy: cargo clippy -- -D warnings
4. Run tests: cargo test
5. Run security audit: cargo audit

## Files
- `.github/workflows/ci.yml`

## Notes
- CI must match local checks
- Cache cargo dependencies
