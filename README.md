# serde-kdl

Serde serialization and deserialization for KDL (Kindly Document Language).

This crate provides integration between Rust's serde framework and KDL documents.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde-kdl = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
```

## Basic Example

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

## Type Mapping

### Primitive Types

| Rust Type | KDL Representation |
|-----------|-------------------|
| `bool` | `true`, `false` |
| `i32`, `i64`, etc. | `42`, `-17` |
| `f32`, `f64` | `3.14`, `-2.5` |
| `String`, `&str` | `"hello"` |

### Collections

```rust
// Vec<T> with primitive values
let numbers = vec![1, 2, 3, 4];
// KDL: root 1 2 3 4

// Vec<T> with structs
let servers = vec![
    Server { name: "web".to_string(), port: 80 },
    Server { name: "api".to_string(), port: 3000 },
];
// KDL:
// root {
//     Server name="web" port=80
//     Server name="api" port=3000
// }
```

### Structs

```rust
#[derive(Serialize, Deserialize)]
struct Database {
    host: String,
    port: u16,
    ssl: bool,
}

// KDL: Database host="localhost" port=5432 ssl=true
```

### Nested Structures

```rust
#[derive(Serialize, Deserialize)]
struct AppConfig {
    name: String,
    database: Database,
    servers: Vec<Server>,
}

// KDL:
// AppConfig name="myapp" {
//     database host="localhost" port=5432 ssl=true
//     servers {
//         Server name="web" port=80
//         Server name="api" port=3000
//     }
// }
```

### Optional Fields

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    description: Option<String>,
}

// With Some: Config name="app" description="My application"
// With None: Config name="app"
```

## API Reference

### Serialization

- `to_string<T>(value: &T) -> Result<String>` - Serialize to KDL string
- `to_document<T>(value: &T) -> Result<kdl::KdlDocument>` - Serialize to KDL document

### Deserialization

- `from_str<T>(s: &str) -> Result<T>` - Deserialize from KDL string
- `from_document<T>(doc: &kdl::KdlDocument) -> Result<T>` - Deserialize from KDL document

### Enums

All Rust enum variant types are fully supported:

```rust
#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Message {
    // Unit variant
    Quit,

    // Newtype variant
    Text(String),

    // Tuple variant
    Move(i32, i32),

    // Struct variant
    ChangeColor { r: u8, g: u8, b: u8 },
}

// Unit variant
let msg = Message::Quit;
// KDL: root "Quit"

// Newtype variant
let msg = Message::Text("Hello".to_string());
// KDL: Text "Hello"

// Tuple variant
let msg = Message::Move(10, 20);
// KDL: Move {
//     tuple 10 20
// }

// Struct variant
let msg = Message::ChangeColor { r: 255, g: 0, b: 0 };
// KDL: ChangeColor r=255 g=0 b=0
```

#### Enums in Collections

```rust
let colors = vec![Color::Red, Color::Green, Color::Blue];
// KDL: root {
//     Red
//     Green
//     Blue
// }

let messages = vec![
    Message::Text("hello".to_string()),
    Message::Move(5, 10),
];
// KDL: root {
//     Text "hello"
//     Move {
//         tuple 5 10
//     }
// }
```

#### Enums in Structs

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    theme: Color,
    backup_message: Option<Message>,
}

// KDL: Config name="app" theme="Dark" backup_message="Quit"
```

#### Recursive Enums

```rust
#[derive(Serialize, Deserialize)]
enum Value {
    Number(f64),
    Text(String),
    Array(Vec<Value>),
}

let data = Value::Array(vec![
    Value::Number(42.0),
    Value::Text("hello".to_string()),
    Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
]);

// KDL: Array {
//     Number 42.0
//     Text "hello"
//     Array {
//         Number 1.0
//         Number 2.0
//     }
// }
```

## Byte Arrays

The `bytes` feature (enabled by default) provides support for serializing and deserializing byte arrays as hex strings.

### Default Behavior

When the `bytes` feature is enabled, `Vec<u8>` and `&[u8]` are automatically serialized as hex strings without requiring any special attributes:

```rust
use serde::{Deserialize, Serialize};
use serde_kdl::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Config {
    name: String,
    secret_key: Vec<u8>,  // No special attribute needed!
}

let config = Config {
    name: "my-app".to_string(),
    secret_key: vec![0xde, 0xad, 0xbe, 0xef],
};

// Serialize to KDL
let kdl_string = to_string(&config)?;
// Output: Config name="my-app" secret_key="deadbeef"

// Deserialize from KDL
let parsed: Config = from_str(&kdl_string)?;
assert_eq!(config, parsed);
```

### Using the hex_serde module for explicit control

You can still use the `#[serde(with = "serde_kdl::hex_serde")]` attribute for explicit control or backwards compatibility:

```rust
use serde::{Deserialize, Serialize};
use serde_kdl::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Config {
    name: String,
    #[serde(with = "serde_kdl::hex_serde")]
    secret_key: Vec<u8>,
}

let config = Config {
    name: "my-app".to_string(),
    secret_key: vec![0xde, 0xad, 0xbe, 0xef],
};

// Serialize to KDL
let kdl_string = to_string(&config)?;
// Output: Config name="my-app" secret_key="deadbeef"

// Deserialize from KDL
let parsed: Config = from_str(&kdl_string)?;
assert_eq!(config, parsed);
```

### For optional byte arrays

Optional byte arrays work by default with the `bytes` feature enabled:

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    optional_data: Option<Vec<u8>>,  // No special attribute needed!
}
```

For explicit control, you can still use `serde_kdl::hex_serde::option`:

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    #[serde(with = "serde_kdl::hex_serde::option")]
    optional_data: Option<Vec<u8>>,
}
```

### Hex string format

- Serialized as lowercase hex strings (e.g., `"deadbeef"`)
- Accepts both uppercase and lowercase hex during deserialization
- Empty byte arrays serialize as empty strings (`""`)
- Invalid hex strings or odd-length strings produce deserialization errors

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