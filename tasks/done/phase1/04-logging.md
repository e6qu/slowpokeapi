# Task: Set Up Logging

## Status
[ ] Pending

## Description

Configure structured logging with tracing and tracing-subscriber.

## Requirements

1. Initialize tracing subscriber on startup
2. Support configurable log level
3. Support JSON format for production
4. Include request/response logging

## Files
- `src/logging.rs`

## Notes
- Use `tracing_subscriber::fmt::format::JsonFields`
- Include source code location in logs
