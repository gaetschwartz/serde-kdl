use serde::{Deserialize, Serialize};
use serde_kdl::{from_str, to_string};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ConfigWithArray {
    name: String,
    tags: Vec<String>,
    ports: Vec<u16>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NestedConfig {
    database: DatabaseConfig,
    servers: Vec<ServerConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    ssl: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ServerConfig {
    name: String,
    endpoint: String,
}

#[test]
fn test_serialize_array_config() {
    let config = ConfigWithArray {
        name: "web-app".to_string(),
        tags: vec!["web".to_string(), "api".to_string(), "rest".to_string()],
        ports: vec![8080, 8443, 9090],
    };

    let kdl_string = to_string(&config).expect("Failed to serialize");
    println!("Serialized array config: {}", kdl_string);

    // Basic validation
    assert!(kdl_string.contains("ConfigWithArray"));
    assert!(kdl_string.contains("web-app"));
}

#[test]
fn test_roundtrip_array_config() {
    let original = ConfigWithArray {
        name: "test-service".to_string(),
        tags: vec!["test".to_string(), "service".to_string()],
        ports: vec![3000, 3001],
    };

    let kdl_string = to_string(&original).expect("Failed to serialize");
    println!("Serialized: {}", kdl_string);

    let deserialized: ConfigWithArray = from_str(&kdl_string).expect("Failed to deserialize");
    assert_eq!(original, deserialized);
}

#[test]
fn test_serialize_nested_structs() {
    let config = NestedConfig {
        database: DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            ssl: true,
        },
        servers: vec![
            ServerConfig {
                name: "api-server".to_string(),
                endpoint: "http://api.example.com".to_string(),
            },
            ServerConfig {
                name: "web-server".to_string(),
                endpoint: "http://web.example.com".to_string(),
            },
        ],
    };

    let kdl_string = to_string(&config).expect("Failed to serialize");
    println!("Serialized nested config: {}", kdl_string);

    // Basic validation
    assert!(kdl_string.contains("NestedConfig"));
    assert!(kdl_string.contains("localhost"));
}

#[test]
fn test_roundtrip_nested_structs() {
    let original = NestedConfig {
        database: DatabaseConfig {
            host: "db.example.com".to_string(),
            port: 5432,
            ssl: false,
        },
        servers: vec![ServerConfig {
            name: "server1".to_string(),
            endpoint: "http://server1.example.com".to_string(),
        }],
    };

    let kdl_string = to_string(&original).expect("Failed to serialize");
    let deserialized: NestedConfig = from_str(&kdl_string).expect("Failed to deserialize");

    assert_eq!(original, deserialized);
}

#[test]
fn test_serialize_simple_array() {
    let numbers = vec![1, 2, 3, 4, 5];
    let kdl_string = to_string(&numbers).expect("Failed to serialize");
    println!("Serialized array: {}", kdl_string);

    let deserialized: Vec<i32> = from_str(&kdl_string).expect("Failed to deserialize");
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
    let kdl_string = to_string(&words).expect("Failed to serialize");
    println!("Serialized string array: {}", kdl_string);

    let deserialized: Vec<String> = from_str(&kdl_string).expect("Failed to deserialize");
    assert_eq!(words, deserialized);
}
