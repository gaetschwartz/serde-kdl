use rstest::rstest;
use serde::{Deserialize, Serialize};
use serde_kdl::{from_str, to_string};

// Test all enum variant types
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Color {
    // Unit variant
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Message {
    // Unit variant
    Quit,
    // Newtype variant
    Text(String),
    // Tuple variant
    Move { x: i32, y: i32 },
    // Struct variant
    ChangeColor(u8, u8, u8),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Value {
    Number(f64),
    Text(String),
    Boolean(bool),
    Array(Vec<Value>),
}

#[rstest]
fn test_unit_enum_variants(#[values(Color::Red, Color::Green, Color::Blue)] color: Color) {
    let serialized = to_string(&color).unwrap();
    println!("Serialized color: {serialized}");

    let deserialized: Color = from_str(&serialized).unwrap();
    assert_eq!(color, deserialized);
}

#[test]
fn test_newtype_enum_variants() {
    let msg = Message::Text("Hello, world!".to_string());

    let serialized = to_string(&msg).unwrap();
    println!("Serialized message: {serialized}");

    let deserialized: Message = from_str(&serialized).unwrap();
    assert_eq!(msg, deserialized);
}

#[test]
fn test_tuple_enum_variants() {
    let msg = Message::ChangeColor(255, 128, 0);

    let serialized = to_string(&msg).unwrap();
    println!("Serialized tuple variant: {serialized}");

    let deserialized: Message = from_str(&serialized).unwrap();
    assert_eq!(msg, deserialized);
}

#[test]
fn test_struct_enum_variants() {
    let msg = Message::Move { x: 10, y: 20 };

    let serialized = to_string(&msg).unwrap();
    println!("Serialized struct variant: {serialized}");

    let deserialized: Message = from_str(&serialized).unwrap();
    assert_eq!(msg, deserialized);
}

#[test]
fn test_nested_enum_in_struct() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Config {
        name: String,
        primary_color: Color,
        secondary_color: Option<Color>,
    }

    let config = Config {
        name: "My App".to_string(),
        primary_color: Color::Blue,
        secondary_color: Some(Color::Red),
    };

    let serialized = to_string(&config).unwrap();
    println!("Serialized config with enums: {serialized}");

    let deserialized: Config = from_str(&serialized).unwrap();
    assert_eq!(config, deserialized);
}

#[test]
fn test_enum_in_vec() {
    let colors = vec![Color::Red, Color::Green, Color::Blue];

    let serialized = to_string(&colors).unwrap();
    println!("Serialized enum vector: {serialized}");

    let deserialized: Vec<Color> = from_str(&serialized).unwrap();
    assert_eq!(colors, deserialized);
}

#[test]
fn test_recursive_enum() {
    let value = Value::Array(vec![
        Value::Number(42.0),
        Value::Text("hello".to_string()),
        Value::Boolean(true),
        Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
    ]);

    let serialized = to_string(&value).unwrap();
    println!("Serialized recursive enum: {serialized}");

    let deserialized: Value = from_str(&serialized).unwrap();
    assert_eq!(value, deserialized);
}
