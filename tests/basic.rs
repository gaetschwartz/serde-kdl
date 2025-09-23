use serde::{Deserialize, Serialize};
use serde_kdl::{from_str, to_string};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SimpleConfig {
    name: String,
    version: String,
    debug: bool,
    port: u16,
}


#[test]
fn test_serialize_simple_struct() {
    let config = SimpleConfig {
        name: "my-app".to_string(),
        version: "1.0.0".to_string(),
        debug: true,
        port: 8080,
    };

    let kdl_string = to_string(&config).expect("Failed to serialize");
    println!("Serialized: {}", kdl_string);

    // Basic validation - should contain the struct name and fields
    assert!(kdl_string.contains("SimpleConfig"));
    assert!(kdl_string.contains("my-app"));
    assert!(kdl_string.contains("1.0.0"));
    assert!(kdl_string.contains("true"));
    assert!(kdl_string.contains("8080"));
}

#[test]
fn test_deserialize_simple_struct() {
    let kdl_string = r#"SimpleConfig name="my-app" version="1.0.0" debug=true port=8080"#;

    let config: SimpleConfig = from_str(kdl_string).expect("Failed to deserialize");

    assert_eq!(config.name, "my-app");
    assert_eq!(config.version, "1.0.0");
    assert!(config.debug);
    assert_eq!(config.port, 8080);
}

#[test]
fn test_roundtrip_simple_struct() {
    let original = SimpleConfig {
        name: "test-app".to_string(),
        version: "2.1.3".to_string(),
        debug: false,
        port: 3000,
    };

    let kdl_string = to_string(&original).expect("Failed to serialize");
    let deserialized: SimpleConfig = from_str(&kdl_string).expect("Failed to deserialize");

    assert_eq!(original, deserialized);
}

#[test]
fn test_serialize_primitive_types() {
    // Test various primitive types
    assert_eq!(to_string(&42i32).unwrap(), "root 42\n");
    assert_eq!(to_string(&true).unwrap(), "root true\n");
    assert_eq!(to_string(&"hello").unwrap(), "root \"hello\"\n");
    assert_eq!(to_string(&2.5f64).unwrap(), "root 2.5\n");
}

#[test]
fn test_deserialize_primitive_types() {
    assert_eq!(from_str::<i32>("root 42").unwrap(), 42);
    assert!(from_str::<bool>("root true").unwrap());
    assert_eq!(from_str::<String>("root \"hello\"").unwrap(), "hello");
    assert_eq!(from_str::<f64>("root 2.5").unwrap(), 2.5);
}

#[test]
fn test_option_types() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct OptionalFields {
        required: String,
        optional: Option<String>,
    }

    let with_optional = OptionalFields {
        required: "must have".to_string(),
        optional: Some("maybe have".to_string()),
    };

    let without_optional = OptionalFields {
        required: "must have".to_string(),
        optional: None,
    };

    // Test serialization
    let kdl_with = to_string(&with_optional).unwrap();
    let kdl_without = to_string(&without_optional).unwrap();

    println!("With optional: {}", kdl_with);
    println!("Without optional: {}", kdl_without);

    // Test roundtrip
    let roundtrip_with: OptionalFields = from_str(&kdl_with).unwrap();
    let roundtrip_without: OptionalFields = from_str(&kdl_without).unwrap();

    assert_eq!(with_optional, roundtrip_with);
    assert_eq!(without_optional, roundtrip_without);
}