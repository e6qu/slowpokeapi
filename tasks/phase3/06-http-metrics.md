# Task: Add HTTP Request Metrics

## Status
[x] Done

## Description

Configure HTTP request metrics collection.

## Files
- `src/metrics/definitions.rs`

## Details

Configure standard metrics:
- `http_requests_total` - Counter
- `http_request_duration_seconds` - Histogram
- `http_requests_in_flight` - Gauge
