# Binary Build Specification

## Overview

SlowPokeAPI is distributed as a statically-linked binary for Linux, macOS, and Windows.

## Build Requirements

### Rust Version
- Minimum: 1.75.0
- Recommended: Latest stable

### Target Triples

| Platform | Target Triple |
|----------|---------------|
| Linux x86_64 | `x86_64-unknown-linux-gnu` |
| Linux ARM64 | `aarch64-unknown-linux-gnu` |
| macOS x86_64 | `x86_64-apple-darwin` |
| macOS ARM64 | `aarch64-apple-darwin` |
| Windows x86_64 | `x86_64-pc-windows-msvc` |

## Build Commands

### Development Build

```bash
cargo build
```

### Release Build (Optimized)

```bash
cargo build --release
```

### Cross-Compilation

```bash
# Add target
rustup target add x86_64-unknown-linux-musl

# Build for musl (static linking)
cargo build --release --target x86_64-unknown-linux-musl
```

## Binary Features

### Feature Flags

```toml
[features]
default = ["sync", "metrics"]
sync = ["automerge"]
metrics = ["prometheus"]
crypto = []  # Enable cryptocurrency support
historical = []  # Enable historical data endpoints
auth = []  # Enable API key authentication
```

### Build with Features

```bash
# Minimal build
cargo build --release --no-default-features

# Full build
cargo build --release --all-features
```

## Optimization

### Cargo.toml Settings

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### Binary Size Reduction

```bash
# Build with optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Further strip symbols (if not using strip = true)
strip target/release/slowpokeapi

# UPX compression (optional, for smaller binaries)
upx --best target/release/slowpokeapi
```

## Output

### Binary Name

```
slowpokeapi
slowpokeapi.exe  # Windows
```

### Expected Size

| Build | Size |
|-------|------|
| Debug | ~50-100 MB |
| Release (stripped) | ~10-20 MB |
| Release + UPX | ~5-10 MB |

## Running the Binary

### Basic Usage

```bash
./slowpokeapi
```

### With Configuration

```bash
# Environment variables
SLOWPOKEAPI_SERVER_PORT=8080 ./slowpokeapi

# Config file
./slowpokeapi --config /etc/slowpokeapi/config.toml
```

### Command-Line Arguments

```
slowpokeapi [OPTIONS]

OPTIONS:
    -c, --config <FILE>    Path to configuration file
    -p, --port <PORT>      Server port [default: 8080]
    -h, --host <HOST>      Server host [default: 0.0.0.0]
    -d, --data-dir <DIR>   Data directory [default: ./data]
    -v, --verbose          Increase verbosity
    -q, --quiet            Suppress output
        --version          Print version
        --help             Print help
```

## Installation

### Linux (systemd)

```bash
# Install binary
sudo cp target/release/slowpokeapi /usr/local/bin/
sudo chmod +x /usr/local/bin/slowpokeapi

# Create data directory
sudo mkdir -p /var/lib/slowpokeapi
sudo chown slowpokeapi:slowpokeapi /var/lib/slowpokeapi

# Install systemd service
sudo cp deploy/slowpokeapi.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable slowpokeapi
sudo systemctl start slowpokeapi
```

### macOS (launchd)

```bash
# Install binary
sudo cp target/release/slowpokeapi /usr/local/bin/

# Install launchd plist
sudo cp deploy/com.slowpokeapi.plist /Library/LaunchDaemons/
sudo launchctl load /Library/LaunchDaemons/com.slowpokeapi.plist
```

### Windows (NSSM)

```powershell
# Install binary
copy target\release\slowpokeapi.exe C:\Program Files\SlowPokeAPI\

# Install as Windows service using NSSM
nssm install SlowPokeAPI "C:\Program Files\SlowPokeAPI\slowpokeapi.exe"
nssm start SlowPokeAPI
```

## systemd Service Unit

```ini
[Unit]
Description=SlowPokeAPI Currency Exchange Service
After=network.target

[Service]
Type=simple
User=slowpokeapi
Group=slowpokeapi
ExecStart=/usr/local/bin/slowpokeapi
Restart=on-failure
RestartSec=5
Environment=SLOWPOKEAPI_STORAGE_PATH=/var/lib/slowpokeapi/slowpokeapi.db
Environment=SLOWPOKEAPI_LOG_LEVEL=info

# Security
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/slowpokeapi

[Install]
WantedBy=multi-user.target
```

## Health Check Script

```bash
#!/bin/bash
# /usr/local/bin/slowpokeapi-health

set -e

response=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/healthz)

if [ "$response" = "200" ]; then
    echo "Healthy"
    exit 0
else
    echo "Unhealthy: HTTP $response"
    exit 1
fi
```

## Build Pipeline

### GitHub Actions

```yaml
name: Build Binary

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: aarch64-apple-darwin
            os: macos-latest
          - target: x86_64-pc-windows-msvc
            os: windows-latest

    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
      
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: slowpokeapi-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/slowpokeapi*
```

## Verification

### Test Binary

```bash
# Check binary runs
./slowpokeapi --version

# Run integration tests
cargo test --release

# Smoke test
./slowpokeapi &
curl http://localhost:8080/healthz
kill %1
```

### Check Dependencies

```bash
# Linux
ldd target/release/slowpokeapi

# macOS
otool -L target/release/slowpokeapi

# Windows
dumpbin /dependents target\release\slowpokeapi.exe
```
