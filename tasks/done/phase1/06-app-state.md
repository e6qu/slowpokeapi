# Task: Implement AppState

## Status
[ ] Pending

## Description

Create the shared application state for all handlers.

## Requirements

1. AppState struct with configuration
2. Arc for thread-safe sharing
3. Include startup time for uptime tracking

## Structure

```rust
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Settings>,
    pub startup_time: Instant,
}
```

## Files
- `src/server/state.rs`

## Notes
- Add more fields in future phases (cache, db, sync)
