# serde-kdl

Serde serialization and deserialization for KDL (Kindly Document Language).

This crate provides integration between Rust's serde framework and KDL documents, including a convenient `kdl!()` macro for constructing KDL documents at compile time.

## Features

- **Serde Integration**: Serialize and deserialize Rust types to/from KDL format
- **`kdl!()` Macro**: Construct KDL documents at compile time with KDL-like syntax
- **Type Safety**: Full compile-time validation and type checking
- **Zero-Cost**: Efficient serialization with minimal runtime overhead

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde-kdl = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
```

## Basic Serde Example

```rust
use serde::{Deserialize, Serialize};
use serde_kdl::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Config {
    name: String,
    port: u16,
    debug: bool,
}

let config = Config {
    name: "my-app".to_string(),
    port: 8080,
    debug: true,
};

// Serialize to KDL
let kdl_string = to_string(&config)?;
// Output: Config name="my-app" port=8080 debug=true

// Deserialize from KDL
let parsed: Config = from_str(&kdl_string)?;
assert_eq!(config, parsed);
```

## KDL Macro Examples

The `kdl!()` macro provides a convenient way to construct KDL documents at compile time, similar to how `serde_json::json!()` works for JSON.

### Basic Usage

```rust
use serde_kdl::kdl;

// Simple node with value
let doc = kdl! {
    node 42
};

// Node with properties
let doc = kdl! {
    server host="localhost" port=8080
};

// Multiple nodes
let doc = kdl! {
    name "my-app"
    version "1.0.0"
    debug true
};
```


## KDL Macro specs compliance

See [SPECS_COMPLIANCE.md](serde-kdl-macro/SPECS_COMPLIANCE.md) for details on compliance with the KDL specification.

## Limitations

The following limitations are imposed by the underlying `kdl` crate 4.7:

- **Integer range**: Numbers are limited to the i64 range (-9,223,372,036,854,775,807 to 9,223,372,036,854,775,807)
  - `u64` values > `i64::MAX` will produce serialization errors with descriptive messages
  - `i128` and `u128` values outside the i64 range will produce serialization errors
  - The extreme value `i64::MIN` (-9,223,372,036,854,775,808) cannot be parsed by the KDL parser
- **Special float values**: `#inf`, `#-inf`, and `#nan` keywords are not supported in kdl 4.7
  - `f64::INFINITY`, `f64::NEG_INFINITY`, and `f64::NAN` serialize as very large numbers or 0.0

These limitations reflect the actual capabilities of the kdl crate rather than artificial restrictions in serde-kdl.

## Requirements

- Rust 1.70+
- serde 1.0+
- kdl 4.7+

## License

This project is licensed under either of Apache License, Version 2.0 or MIT license at your option.