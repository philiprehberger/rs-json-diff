# rs-json-diff

[![CI](https://github.com/philiprehberger/rs-json-diff/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-json-diff/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-json-diff.svg)](https://crates.io/crates/philiprehberger-json-diff)
[![GitHub release](https://img.shields.io/github/v/release/philiprehberger/rs-json-diff)](https://github.com/philiprehberger/rs-json-diff/releases)
[![Last updated](https://img.shields.io/github/last-commit/philiprehberger/rs-json-diff)](https://github.com/philiprehberger/rs-json-diff/commits/main)
[![License](https://img.shields.io/github/license/philiprehberger/rs-json-diff)](LICENSE)
[![Bug Reports](https://img.shields.io/github/issues/philiprehberger/rs-json-diff/bug)](https://github.com/philiprehberger/rs-json-diff/issues?q=is%3Aissue+is%3Aopen+label%3Abug)
[![Feature Requests](https://img.shields.io/github/issues/philiprehberger/rs-json-diff/enhancement)](https://github.com/philiprehberger/rs-json-diff/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)
[![Sponsor](https://img.shields.io/badge/sponsor-GitHub%20Sponsors-ec6cb9)](https://github.com/sponsors/philiprehberger)

Structural JSON diff with path tracking for Rust

## Installation

```toml
[dependencies]
philiprehberger-json-diff = "0.1.8"
```

## Usage

```rust
use philiprehberger_json_diff::{diff, diff_summary};
use serde_json::json;

let a = json!({
    "name": "Alice",
    "age": 30,
    "tags": ["rust", "dev"]
});

let b = json!({
    "name": "Alice",
    "age": 31,
    "tags": ["rust", "senior"],
    "active": true
});

let changes = diff(&a, &b);
for change in &changes {
    println!("{}", change);
}

let summary = diff_summary(&changes);
println!("Added: {}, Removed: {}, Modified: {}", summary.added, summary.removed, summary.modified);
```

## API

| Item | Description |
|------|-------------|
| `diff(a: &Value, b: &Value) -> Vec<Change>` | Compute structural diff between two JSON values |
| `diff_summary(changes: &[Change]) -> DiffSummary` | Summarize a list of changes by type counts |
| `ChangeType` | Enum: `Added`, `Removed`, `Modified` |
| `Change` | Struct with `path`, `change_type`, `old_value`, `new_value` |
| `DiffSummary` | Struct with `added`, `removed`, `modified` counts |


## Development

```bash
cargo test
cargo clippy -- -D warnings
```

## Support

If you find this package useful, consider giving it a star on GitHub — it helps motivate continued maintenance and development.

[![LinkedIn](https://img.shields.io/badge/Philip%20Rehberger-LinkedIn-0A66C2?logo=linkedin)](https://www.linkedin.com/in/philiprehberger)
[![More packages](https://img.shields.io/badge/more-open%20source%20packages-blue)](https://philiprehberger.com/open-source-packages)

## License

[MIT](LICENSE)
