# commonsense

[![Crates.io](https://img.shields.io/crates/v/commonsense.svg)](https://crates.io/crates/commonsense)
[![Docs.rs](https://docs.rs/commonsense/badge.svg)](https://docs.rs/commonsense)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

**commonsense** is a collection of ergonomic rituals for Rust: macros and utilities that encode ownership and intent without hiding their cost.

---

## Features
- `svec!` — create a `Vec<String>` from string literals or expressions.
---

## Example

```rust
use commonsense as cs;

fn main() {
    let words = cs::svec!["hello", "world"];
    assert_eq!(words, vec!["hello".to_string(), "world".to_string()]);
}
