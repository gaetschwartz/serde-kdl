use rstest::rstest;
use serde::{Deserialize, Serialize};
use serde_kdl::{from_str, to_string};
use std::fs;

/// Test that we can at least read all example KDL files and handle parsing gracefully
#[rstest]
fn test_parse_examples(#[files("../../specs/examples/*.kdl")] path: std::path::PathBuf) {
    let content =
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read file {path:?}: {e}"));

    // Try to parse the KDL document - some examples may use older/different syntax
    let doc = content
        .parse::<kdl::KdlDocument>()
        .expect("Failed to parse KDL document");

    // Valid KDL - test roundtrip serialization
    let serialized = doc.to_string();
    assert!(
        !serialized.is_empty(),
        "Serialized document should not be empty for {path:?}"
    );

    // Parse the serialized version to ensure roundtrip works at KDL level
    let _reparsed: kdl::KdlDocument = serialized
        .parse()
        .unwrap_or_else(|e| panic!("Failed to reparse serialized KDL from {path:?}: {e}"));

    println!(
        "✓ Successfully processed valid KDL example: {:?}",
        path.file_name().unwrap()
    );
}

/// Test roundtrip serialization for a custom document structure
/// that represents common KDL patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DocumentTest {
    nodes: Vec<NodeTest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct NodeTest {
    name: String,
    properties: std::collections::HashMap<String, PropertyValue>,
    children: Vec<NodeTest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}

#[test]
fn test_roundtrip_document_structure() {
    let mut properties = std::collections::HashMap::new();
    properties.insert(
        "version".to_string(),
        PropertyValue::String("1.0".to_string()),
    );
    properties.insert("port".to_string(), PropertyValue::Integer(8080));
    properties.insert("enabled".to_string(), PropertyValue::Bool(true));

    let document = DocumentTest {
        nodes: vec![NodeTest {
            name: "config".to_string(),
            properties: properties.clone(),
            children: vec![NodeTest {
                name: "server".to_string(),
                properties: {
                    let mut server_props = std::collections::HashMap::new();
                    server_props.insert(
                        "host".to_string(),
                        PropertyValue::String("localhost".to_string()),
                    );
                    server_props
                },
                children: Vec::new(),
            }],
        }],
    };

    let serialized = to_string(&document).expect("Failed to serialize document");
    println!("Serialized document structure: {serialized}");

    let deserialized: DocumentTest = from_str(&serialized).expect("Failed to deserialize document");
    assert_eq!(document, deserialized);
}

/// Test various KDL node structures that might appear in examples
#[test]
fn test_complex_node_structures() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct ComplexConfig {
        package: PackageInfo,
        dependencies: Vec<Dependency>,
        metadata: std::collections::HashMap<String, String>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct PackageInfo {
        name: String,
        version: String,
        description: Option<String>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Dependency {
        name: String,
        version: String,
        optional: Option<bool>,
    }

    let config = ComplexConfig {
        package: PackageInfo {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: Some("A test package".to_string()),
        },
        dependencies: vec![
            Dependency {
                name: "serde".to_string(),
                version: "1.0".to_string(),
                optional: None,
            },
            Dependency {
                name: "tokio".to_string(),
                version: "1.0".to_string(),
                optional: Some(true),
            },
        ],
        metadata: {
            let mut meta = std::collections::HashMap::new();
            meta.insert("author".to_string(), "Test Author".to_string());
            meta.insert("license".to_string(), "MIT".to_string());
            meta
        },
    };

    let serialized = to_string(&config).expect("Failed to serialize complex config");
    println!("Serialized complex config: {serialized}");

    let deserialized: ComplexConfig =
        from_str(&serialized).expect("Failed to deserialize complex config");
    assert_eq!(config, deserialized);
}

/// Test that we can handle empty arrays and optional fields properly
#[test]
fn test_edge_cases() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct EdgeCaseConfig {
        name: String,
        tags: Vec<String>,
        servers: Vec<ServerConfig>,
        optional_field: Option<String>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct ServerConfig {
        name: String,
        port: u16,
    }

    // Test with empty arrays
    let config_empty = EdgeCaseConfig {
        name: "test".to_string(),
        tags: Vec::new(),
        servers: Vec::new(),
        optional_field: None,
    };

    let serialized = to_string(&config_empty).expect("Failed to serialize empty config");
    let deserialized: EdgeCaseConfig =
        from_str(&serialized).expect("Failed to deserialize empty config");
    assert_eq!(config_empty, deserialized);

    // Test with populated arrays
    let config_full = EdgeCaseConfig {
        name: "test-full".to_string(),
        tags: vec!["web".to_string(), "api".to_string()],
        servers: vec![
            ServerConfig {
                name: "web".to_string(),
                port: 80,
            },
            ServerConfig {
                name: "api".to_string(),
                port: 3000,
            },
        ],
        optional_field: Some("optional value".to_string()),
    };

    let serialized = to_string(&config_full).expect("Failed to serialize full config");
    let deserialized: EdgeCaseConfig =
        from_str(&serialized).expect("Failed to deserialize full config");
    assert_eq!(config_full, deserialized);
}
