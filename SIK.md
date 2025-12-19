// =============================================================================
// KDL Serde Specification v0.2
// =============================================================================

// =============================================================================
// DESIGN PRINCIPLES
// =============================================================================
//
// 1. No properties - never use key=value syntax
// 2. Fields become child nodes - field name is node name
// 3. Scalar values as arguments - the value follows the node name
// 4. Collections use wrapper + dash - Vec<T> becomes field_name { - item; - item }
// 5. Type names raw - struct type names used as-is (e.g., Route, not route)
// 6. Consistent nesting - structure always reflected in tree depth

// =============================================================================
// CORE RULES
// =============================================================================

// Rule 1: Struct fields become child nodes
// Rule 2: Scalar values become arguments
// Rule 3: Nested structs become nested children
// Rule 4: Vec<T> uses wrapper node with "-" children
// Rule 5: Properties (key=value) are NEVER used

// =============================================================================
// SCALAR TYPES
// =============================================================================

// Rust: String, &str, i8-i128, u8-u128, f32, f64, bool
// KDL: child node with value as argument
//
// struct Example { name: String, count: i32, enabled: bool }
// -->
// Example {
//     name "hello"
//     count 42
//     enabled true
// }

// =============================================================================
// STRUCTS (NAMED FIELDS)
// =============================================================================

// Each field becomes a child node.
// Field name = node name, value = argument or children.
//
// struct Person { name: String, age: u32 }
// -->
// Person {
//     name "Alice"
//     age 30
// }

// Nested structs become nested children:
//
// struct Outer { tag: String, inner: Inner }
// struct Inner { value: i32 }
// -->
// Outer {
//     tag "foo"
//     inner {
//         value 42
//     }
// }

// =============================================================================
// TUPLE STRUCTS
// =============================================================================

// Multiple arguments in order.
//
// struct Point(f64, f64, f64);
// -->
// Point 1.0 2.0 3.0

// When used as a field:
// struct Config { origin: Point }
// -->
// Config {
//     origin 1.0 2.0 3.0
// }

// =============================================================================
// NEWTYPE STRUCTS
// =============================================================================

// Transparent - inner value as argument.
//
// struct UserId(u64);
// struct Config { owner: UserId }
// -->
// Config {
//     owner 12345
// }

// =============================================================================
// UNIT STRUCTS
// =============================================================================

// Node with no arguments or children.
//
// struct Marker;
// struct Config { flag: Marker }
// -->
// Config {
//     flag
// }

// =============================================================================
// OPTION<T>
// =============================================================================

// None: node omitted entirely
// Some(v): as if Option wrapper didn't exist
//
// struct Config { name: String, timeout: Option<u32> }
//
// Config { name: "x".into(), timeout: None }
// -->
// Config {
//     name "x"
// }
//
// Config { name: "x".into(), timeout: Some(30) }
// -->
// Config {
//     name "x"
//     timeout 30
// }

// =============================================================================
// VEC<T> - COLLECTIONS
// =============================================================================

// All Vec<T> use wrapper node with "-" children.
// Field name becomes wrapper node name.
//
// Vec<String>:
// struct Config { flags: Vec<String> }
// -->
// Config {
//     flags {
//         - "read_only"
//         - "replica"
//     }
// }

// Vec<Struct>:
// struct Server { routes: Vec<Route> }
// struct Route { path: String, method: String }
// -->
// Server {
//     routes {
//         - {
//             path "/api"
//             method "GET"
//         }
//         - {
//             path "/health"
//             method "GET"
//         }
//     }
// }

// Empty Vec: node omitted (or empty wrapper, configurable)
// struct Config { flags: Vec<String> }
// where flags = vec![]
// -->
// Config {
// }

// =============================================================================
// ENUMS
// =============================================================================

// Use KDL type annotations for variant names.

// Unit variant:
// enum Status { Pending, Complete }
// struct Task { status: Status }
// -->
// Task {
//     (Pending)status
// }

// Newtype variant:
// enum Value { Int(i32), Text(String) }
// struct Config { value: Value }
// -->
// Config {
//     (Int)value 42
// }
// or
// Config {
//     (Text)value "hello"
// }

// Struct variant:
// enum Endpoint { Http { host: String, port: u32 }, Unix { path: String } }
// struct Config { endpoint: Endpoint }
// -->
// Config {
//     (Http)endpoint {
//         host "localhost"
//         port 8080
//     }
// }
// or
// Config {
//     (Unix)endpoint {
//         path "/var/run/sock"
//     }
// }

// Tuple variant:
// enum Color { Rgb(u8, u8, u8) }
// -->
// Config {
//     (Rgb)color 255 128 0
// }

// =============================================================================
// HASHMAPS / BTREEMAPS
// =============================================================================

// Map<String, V>: wrapper node, each key becomes child node name
//
// struct Config { env: HashMap<String, String> }
// -->
// Config {
//     env {
//         HOME "/home/user"
//         PATH "/usr/bin"
//     }
// }

// Map<String, Struct>:
// struct Config { servers: HashMap<String, Server> }
// struct Server { port: u32, ssl: bool }
// -->
// Config {
//     servers {
//         main {
//             port 8080
//             ssl true
//         }
//         backup {
//             port 8081
//             ssl false
//         }
//     }
// }

// =============================================================================
// SERDE ATTRIBUTES
// =============================================================================

// #[serde(rename = "name")]     -> use "name" as node name
// #[serde(skip)]                -> omit field entirely
// #[serde(default)]             -> use Default::default() if missing
// #[serde(flatten)]             -> hoist children into parent

// =============================================================================
// FLATTEN BEHAVIOR
// =============================================================================

// Flattened struct's children become siblings in parent.
//
// struct Outer { name: String, #[serde(flatten)] inner: Inner }
// struct Inner { host: String, port: u32 }
// -->
// Outer {
//     name "x"
//     host "localhost"
//     port 8080
// }

// =============================================================================
// ROOT DOCUMENT
// =============================================================================

// Top-level struct fields become root-level nodes.
//
// struct Config { database: Database, server: Server }
// -->
// database {
//     ...
// }
// server {
//     ...
// }
//
// Note: root-level uses lowercase field names, not type names.

// =============================================================================
// FULL EXAMPLE
// =============================================================================

// struct Config {
//     database: DatabaseConfig,
//     servers: Vec<Server>,
// }
// struct DatabaseConfig {
//     host: String,
//     port: u32,
//     ssl: bool,
//     flags: Vec<String>,
// }
// struct Server {
//     routes: Vec<Route>,
//     config: ServerConfig,
// }
// struct Route {
//     path: String,
//     method: String,
// }
// struct ServerConfig {
//     use_tls: bool,
//     features: Vec<String>,
// }

database {
    host "db.example.com"
    port 5432
    ssl false
    flags {
        - "read_only"
        - "replica"
    }
}

servers {
    - {
        routes {
            - {
                path "/api"
                method "GET"
            }
            - {
                path "/health"
                method "GET"
            }
        }
        config {
            use_tls true
            features {
                - "http2"
                - "compression"
            }
        }
    }
}

// =============================================================================
// SUMMARY OF NODE STRUCTURE
// =============================================================================
//
// | Rust Type              | KDL Structure                        |
// |------------------------|--------------------------------------|
// | scalar field           | field_name value                     |
// | struct field           | field_name { children... }           |
// | tuple struct           | node arg1 arg2 arg3                  |
// | newtype struct         | field_name inner_value               |
// | unit struct            | field_name                           |
// | Option::None           | (omitted)                            |
// | Option::Some(v)        | (as if no Option)                    |
// | Vec<T>                 | field_name { - item; - item; ... }   |
// | HashMap<String, V>     | field_name { key1 val; key2 val }    |
// | enum (unit)            | (Variant)field_name                  |
// | enum (newtype)         | (Variant)field_name value            |
// | enum (tuple)           | (Variant)field_name v1 v2 v3         |
// | enum (struct)          | (Variant)field_name { children... }  |