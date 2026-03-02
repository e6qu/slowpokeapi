# Agent Instructions

This document defines the workflow and conventions for AI agents working on SlowPokeAPI.

## Branch Management Requirements

**CRITICAL:** Before starting any work on a phase or opening a PR:

1. **Sync local `main` with `origin/main`:**
   ```bash
   git checkout main
   git fetch origin
   git reset --hard origin/main
   ```

2. **Always create feature branch from synced `main`:**
   ```bash
   git checkout -b phase{N}/feature-name
   ```

3. **Rebase on `origin/main` before opening PR:**
   ```bash
   git fetch origin
   git rebase origin/main
   # Resolve any conflicts
   git push origin <branch> --force-with-lease
   ```

4. **Delete old branches after PR merge:**
   ```bash
   git branch -D <old-branch>
   git push origin --delete <old-branch>  # if pushed
   ```

## Workflow

### Phase Execution

Each implementation phase follows this sequence:

```
1. Plan the phase
   ├── Sync main: git checkout main && git reset --hard origin/main
   ├── Read specs/implementation/phases.md for phase details
   ├── Update PLAN.md, STATUS.md, WHAT_WE_DID.md, DO_NEXT.md
   ├── Create task files in tasks/phase{N}/
   └── Identify any ambiguities → ASK before proceeding

2. Implement
   ├── Write code following conventions (see below)
   ├── Write/update tests
   ├── Run tests locally: cargo test
   ├── Run lints locally: cargo clippy --all-targets --all-features -- -D warnings
   ├── Run format check: cargo fmt --check
   └── Fix all issues before committing

3. Verify against spec
   ├── Re-read relevant specs in specs/
   ├── Ensure implementation matches specification
   └── Fix any deviations

4. Update crucial files
   ├── PLAN.md - Update phase status
   ├── STATUS.md - Update current state
   ├── WHAT_WE_DID.md - Document what was done
   └── DO_NEXT.md - Set up next phase

5. Manage tasks
   ├── Mark completed tasks in tasks/phase{N}/*.md
   └── Move completed phase tasks to tasks/done/phase{N}/

6. Create PR
   ├── Rebase on origin/main: git fetch origin && git rebase origin/main
   ├── Resolve any merge conflicts
   ├── Commit all changes
   ├── Push to origin using: git push origin <branch>
   ├── Create PR using: gh pr create
   ├── Wait for CI to pass
   ├── Fix CI issues if any, push again
   └── Phase done when PR is open AND CI passes

7. Sync main
   ├── Switch to main: git checkout main
   └── Sync: git reset --hard origin/main (use reset after squash merge)
```

### Git Commands

Use SSH mode for all git operations:

```bash
# Rebase from origin/main
git fetch origin
git rebase origin/main

# Push to origin
git push origin <branch-name>

# Create PR
gh pr create --title "Phase N: <name>" --body "$(cat <<'EOF'
## Summary
<Brief summary of changes>

## Changes
- <List of changes>

## Testing
- <How tested>

## Checklist
- [ ] Tests pass
- [ ] Clippy passes
- [ ] Format check passes
- [ ] Crucial files updated
- [ ] Matches specification
EOF
)"

# View PR status
gh pr status

# View CI checks
gh pr checks

# Merge PR (after review/approval)
gh pr merge --squash
```

## Development Conventions

### Code Style

1. **No useless comments**
   ```rust
   // BAD: Increment counter
   counter += 1;
   
   // GOOD: counter += 1;
   ```

2. **Prefer early exits** to reduce indentation
   ```rust
   // BAD
   fn process(value: Option<i32>) -> Result<i32> {
       if let Some(v) = value {
           if v > 0 {
               Ok(v * 2)
           } else {
               Err(Error::Negative)
           }
       } else {
           Err(Error::Missing)
       }
   }
   
   // GOOD
   fn process(value: Option<i32>) -> Result<i32> {
       let v = value.ok_or(Error::Missing)?;
       if v <= 0 {
           return Err(Error::Negative);
       }
       Ok(v * 2)
   }
   ```

3. **Invert if-statements** when it simplifies logic
   ```rust
   // BAD
   if user.is_admin {
       grant_access();
   } else {
       deny_access();
   }
   
   // GOOD
   if !user.is_admin {
       deny_access();
       return;
   }
   grant_access();
   ```

4. **Strong types everywhere**
   ```rust
   // BAD
   fn convert(amount: f64, from: &str, to: &str) -> f64
   
   // GOOD
   #[derive(Debug, Clone)]
   struct Amount(f64);
   #[derive(Debug, Clone)]
   struct CurrencyCode(String);
   
   fn convert(amount: Amount, from: CurrencyCode, to: CurrencyCode) -> Result<Amount, Error>
   ```

5. **No magic numbers/strings**
   ```rust
   // BAD
   if status == 404 { ... }
   
   // GOOD
   const NOT_FOUND: u16 = 404;
   if status == NOT_FOUND { ... }
   ```

### Error Handling

- Use `Result<T, Error>` for all fallible operations
- Use `thiserror` for error types
- Provide meaningful error messages
- Never panic in production code

### Testing

- Write unit tests for all business logic
- Write integration tests for API endpoints
- Use `#[tokio::test]` for async tests
- Aim for high coverage on critical paths

### Logging

- Use `tracing` crate
- Log levels: ERROR, WARN, INFO, DEBUG, TRACE
- Include context in log messages
- Use structured logging where appropriate

## CI Requirements

CI must run the same commands we run locally:

```yaml
# .github/workflows/ci.yml
jobs:
  check:
    - cargo fmt --check
    - cargo clippy --all-targets --all-features -- -D warnings
  
  test:
    - cargo test --all-features
  
  security:
    - cargo audit
```

A phase is **done** when:
1. All 4 crucial files are updated
2. All tests pass locally
3. All CI checks pass
4. PR is open and ready for review

## Task Management

### Directory Structure

```
tasks/
├── phase1/
│   ├── 01-cargo-init.md
│   ├── 02-directory-structure.md
│   └── ...
├── phase2/
│   └── ...
├── done/
│   └── phase1/
│       ├── 01-cargo-init.md
│       └── ...
└── README.md
```

### Task File Format

```markdown
# Task: <Name>

## Status
[ ] Pending
[ ] In Progress
[ ] Done

## Description
<Brief description>

## Files
- <file1>
- <file2>

## Notes
<Any notes>
```

## Questions & Ambiguities

**STOP AND ASK** if:

1. Specification is unclear or missing details
2. Multiple implementation approaches seem valid
3. You're unsure about a design decision
4. Something seems wrong or inconsistent
5. You need clarification on requirements

Use the question tool to ask the user before proceeding.

## Checks for Everything

All of these should be checked and pass:

1. **Code Quality**
   - `cargo fmt --check` - Formatting
   - `cargo clippy --all-targets --all-features -- -D warnings` - Linting

2. **Testing**
   - `cargo test --all-features` - Unit and integration tests
   - `cargo test --doc` - Doc tests

3. **Security**
   - `cargo audit` - Dependency vulnerabilities
   - No secrets in code

4. **Documentation**
   - Public APIs have doc comments
   - README is up to date

5. **Crucial Files**
   - PLAN.md updated
   - STATUS.md updated
   - WHAT_WE_DID.md updated
   - DO_NEXT.md updated

6. **Task Files**
   - Created in `tasks/phase{N}/`
   - Moved to `tasks/done/phase{N}/` when complete

## Parallel Agents

Use parallel agents when:
- Multiple independent tasks can run simultaneously
- Searching for information across multiple files
- Running independent tests or checks

Example:
```
Task 1: Search for all uses of deprecated function
Task 2: Check if tests exist for module X
Task 3: Verify spec matches implementation for endpoint Y
```

## Summary Checklist

Before marking a phase complete:

- [ ] All code written
- [ ] All tests pass (`cargo test`)
- [ ] Clippy passes (`cargo clippy -- -D warnings`)
- [ ] Format check passes (`cargo fmt --check`)
- [ ] Implementation matches spec
- [ ] PLAN.md updated
- [ ] STATUS.md updated
- [ ] WHAT_WE_DID.md updated
- [ ] DO_NEXT.md updated
- [ ] Task files created and updated
- [ ] Committed to feature branch
- [ ] Rebased from origin/main
- [ ] Pushed to origin
- [ ] PR created with `gh pr create`
- [ ] CI passes
