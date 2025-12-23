use insta::assert_snapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    database: DatabaseConfig,
    servers: Vec<Server>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    ssl: bool,
    flags: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Server {
    routes: Vec<Route>,
    config: ServerConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Route {
    path: String,
    method: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ServerConfig {
    use_tls: bool,
    features: Vec<String>,
}

#[test]
fn test_roundtrip_nested_structs() {
    let original = Config {
        database: DatabaseConfig {
            host: "db.example.com".to_string(),
            port: 5432,
            ssl: false,
            flags: vec!["read_only".to_string(), "replica".to_string()],
        },
        servers: vec![Server {
            routes: vec![Route {
                path: "/api".to_string(),
                method: "GET".to_string(),
            }],
            config: ServerConfig {
                use_tls: true,
                features: vec!["http2".to_string(), "compression".to_string()],
            },
        }],
    };

    let kdl_string = serde_kdl::to_string_pretty(&original).expect("Failed to serialize");

    assert_snapshot!(kdl_string, @r#"
    database {
        host db.example.com
        port 5432
        ssl #false
        flags {
            - read_only
            - replica
        }
    }
    servers {
        - {
            routes {
                - {
                    path "/api"
                    method GET
                }
            }
            config {
                use_tls #true
                features {
                    - http2
                    - compression
                }
            }
        }
    }
    "#);

    let deserialized: Config = serde_kdl::from_str(&kdl_string).expect("Failed to deserialize");
    assert_eq!(original, deserialized);
}

#[test]
fn test_serialize_simple_array() {
    let numbers = vec![1, 2, 3, 4, 5];
    let kdl_string = serde_kdl::to_string_pretty(&numbers).expect("Failed to serialize");
    assert_snapshot!(kdl_string, @r#"
    - {
        - 1
        - 2
        - 3
        - 4
        - 5
    }
    "#);

    let deserialized: Vec<i32> = serde_kdl::from_str(&kdl_string).expect("Failed to deserialize");
    assert_eq!(numbers, deserialized);
}

#[test]
fn test_serialize_string_array() {
    let words = vec![
        "hello".to_string(),
        "world".to_string(),
        "from".to_string(),
        "kdl".to_string(),
    ];
    let kdl_string = serde_kdl::to_string_pretty(&words).expect("Failed to serialize");
    assert_snapshot!(kdl_string, @r#"
    - {
        - hello
        - world
        - from
        - kdl
    }
    "#);

    let deserialized: Vec<String> =
        serde_kdl::from_str(&kdl_string).expect("Failed to deserialize");
    assert_eq!(words, deserialized);
}
