use insta::assert_snapshot;
use serde::{Deserialize, Serialize};
use serde_kdl::{from_str, to_string_pretty};

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

#[test]
fn test_unit_enum_variant_red() {
    let color = Color::Red;
    let serialized = to_string_pretty(&color).unwrap();
    assert_snapshot!(serialized, @"(Red)-
");

    let deserialized: Color = from_str(&serialized).unwrap();
    assert_eq!(color, deserialized);
}

#[test]
fn test_unit_enum_variant_green() {
    let color = Color::Green;
    let serialized = to_string_pretty(&color).unwrap();
    assert_snapshot!(serialized, @"(Green)-
");

    let deserialized: Color = from_str(&serialized).unwrap();
    assert_eq!(color, deserialized);
}

#[test]
fn test_unit_enum_variant_blue() {
    let color = Color::Blue;
    let serialized = to_string_pretty(&color).unwrap();
    assert_snapshot!(serialized, @"(Blue)-
");

    let deserialized: Color = from_str(&serialized).unwrap();
    assert_eq!(color, deserialized);
}

#[test]
fn test_newtype_enum_variants() {
    let msg = Message::Text("Hello, world!".to_string());

    let serialized = to_string_pretty(&msg).unwrap();
    assert_snapshot!(serialized, @r#"(Text)- "Hello, world!"
"#);

    let deserialized: Message = from_str(&serialized).unwrap();
    assert_eq!(msg, deserialized);
}

#[test]
fn test_tuple_enum_variants() {
    let msg = Message::ChangeColor(255, 128, 0);

    let serialized = to_string_pretty(&msg).unwrap();
    assert_snapshot!(serialized, @"(ChangeColor)- 255 128 0
");

    let deserialized: Message = from_str(&serialized).unwrap();
    assert_eq!(msg, deserialized);
}

#[test]
fn test_struct_enum_variants() {
    let msg = Message::Move { x: 10, y: 20 };

    let serialized = to_string_pretty(&msg).unwrap();
    assert_snapshot!(serialized, @r#"
    (Move)- {
        x 10
        y 20
    }
    "#);

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

    let serialized = to_string_pretty(&config).unwrap();
    assert_snapshot!(serialized, @r#"
    name "My App"
    (Blue)primary_color
    (Red)secondary_color
    "#);

    let deserialized: Config = from_str(&serialized).unwrap();
    assert_eq!(config, deserialized);
}

#[test]
fn test_enum_in_vec() {
    let colors = vec![Color::Red, Color::Green, Color::Blue];

    let serialized = to_string_pretty(&colors).unwrap();
    assert_snapshot!(serialized, @r#"
    (Red)-
    (Green)-
    (Blue)-
    "#);

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

    let serialized = to_string_pretty(&value).unwrap();
    assert_snapshot!(serialized, @r#"
    (Array)- {
        (Number)- 42.0
        (Text)- hello
        (Boolean)- #true
        (Array)- {
            (Number)- 1.0
            (Number)- 2.0
        }
    }
    "#);

    let deserialized: Value = from_str(&serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn test_enum_as_property_value() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Wrapper {
        message: Message,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Message {
        content: String,
        r#type: MessageType,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum MessageType {
        Audio,
        Video,
        Text,
    }

    let wrapper = Wrapper {
        message: Message {
            content: "Hello".to_string(),
            r#type: MessageType::Text,
        },
    };

    let serialized = to_string_pretty(&wrapper).unwrap();
    assert_snapshot!(serialized, @"
    message {
        content Hello
        (Text)type
    }
    ");
}
