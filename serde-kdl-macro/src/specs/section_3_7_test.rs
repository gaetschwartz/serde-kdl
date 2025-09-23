//! Tests for Section 3.7: Value
//!
//! A value is either: a String (Section 3.9), a Number (Section 3.14),
//! a Boolean (Section 3.15), or Null (Section 3.16).
//!
//! Values MUST be either Arguments (Section 3.5) or values of Properties (Section 3.4).
//! Only String (Section 3.9) values may be used as Node (Section 3.2) names or
//! Property (Section 3.4) keys.
//!
//! Values (both as arguments and in properties) MAY be prefixed by a single
//! Type Annotation (Section 3.8).

use crate::assert_eq_tk;
use crate::specs::kdl_impl2;
use proc_macro2::TokenStream;
use quote::quote;
use serde_kdl_macro::kdl;
use seq_macro::seq;

#[test]
fn test_string_values_as_arguments() {
    // Test all string types as node arguments

    // Identifier strings
    let doc = kdl! {
        node identifier_string
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_string(), Some("identifier_string"));
    } else {
        panic!("Expected string argument");
    }

    // Quoted strings
    let doc = kdl! {
        node "quoted string with spaces"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_string(), Some("quoted string with spaces"));
    } else {
        panic!("Expected string argument");
    }

    // Empty string
    let doc = kdl! {
        node ""
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_string(), Some(""));
    } else {
        panic!("Expected empty string argument");
    }

    // String with escape sequences
    let doc = kdl! {
        node "line1\nline2\ttab"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_string(), Some("line1\nline2\ttab"));
    } else {
        panic!("Expected string argument with escapes");
    }
}

#[test]
fn test_string_values_as_properties() {
    // Test strings as property values

    // Identifier string as property value
    let doc = kdl! {
        node key=identifier_value
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(prop) = node.entries()[0].name() {
        assert_eq!(prop.value(), "key");
        if let Some(value) = node.entries()[0].value() {
            assert_eq!(value.as_string(), Some("identifier_value"));
        } else {
            panic!("Expected property value");
        }
    } else {
        panic!("Expected property");
    }

    // Quoted string as property value
    let doc = kdl! {
        node name="John Doe"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(prop) = node.entries()[0].name() {
        assert_eq!(prop.value(), "name");
        if let Some(value) = node.entries()[0].value() {
            assert_eq!(value.as_string(), Some("John Doe"));
        } else {
            panic!("Expected property value");
        }
    } else {
        panic!("Expected property");
    }
}

#[test]
fn test_number_values_as_arguments() {
    // Test decimal numbers
    let doc = kdl! {
        node 42
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(42));
    } else {
        panic!("Expected number argument");
    }

    // Test negative number
    let doc = kdl! {
        node -123
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(-123));
    } else {
        panic!("Expected negative number argument");
    }

    // Test positive number with explicit sign
    let doc = kdl! {
        node +456
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(456));
    } else {
        panic!("Expected positive number argument");
    }

    // Test zero
    let doc = kdl! {
        node 0
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(0));
    } else {
        panic!("Expected zero argument");
    }

    // Test floating point number
    let doc = kdl! {
        node 3.14159
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_f64(), Some(3.14159));
    } else {
        panic!("Expected float argument");
    }

    // Test scientific notation
    let doc = kdl! {
        node 1.23e4
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_f64(), Some(12300.0));
    } else {
        panic!("Expected scientific notation argument");
    }

    // Test negative scientific notation
    let doc = kdl! {
        node -2.5e-3
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_f64(), Some(-0.0025));
    } else {
        panic!("Expected negative scientific notation argument");
    }
}

#[test]
fn test_number_radix_formats() {
    // Test hexadecimal numbers
    let doc = kdl! {
        node 0xFF
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(255));
    } else {
        panic!("Expected hex argument");
    }

    // Test octal numbers
    let doc = kdl! {
        node 0o777
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(511));
    } else {
        panic!("Expected octal argument");
    }

    // Test binary numbers
    let doc = kdl! {
        node 0b1010
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(10));
    } else {
        panic!("Expected binary argument");
    }
}

#[test]
fn test_keyword_numbers() {
    // Test positive infinity
    let doc = kdl! {
        node #inf
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert!(arg.as_f64().unwrap().is_infinite());
        assert!(arg.as_f64().unwrap().is_sign_positive());
    } else {
        panic!("Expected infinity argument");
    }

    // Test negative infinity
    let doc = kdl! {
        node #-inf
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert!(arg.as_f64().unwrap().is_infinite());
        assert!(arg.as_f64().unwrap().is_sign_negative());
    } else {
        panic!("Expected negative infinity argument");
    }

    // Test NaN
    let doc = kdl! {
        node #nan
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert!(arg.as_f64().unwrap().is_nan());
    } else {
        panic!("Expected NaN argument");
    }
}

#[test]
fn test_number_values_as_properties() {
    // Test integer as property value
    let doc = kdl! {
        node port=8080
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(prop) = node.entries()[0].name() {
        assert_eq!(prop.value(), "port");
        if let Some(value) = node.entries()[0].value() {
            assert_eq!(value.as_i64(), Some(8080));
        } else {
            panic!("Expected property value");
        }
    } else {
        panic!("Expected property");
    }

    // Test float as property value
    let doc = kdl! {
        node ratio=1.618
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(prop) = node.entries()[0].name() {
        assert_eq!(prop.value(), "ratio");
        if let Some(value) = node.entries()[0].value() {
            assert_eq!(value.as_f64(), Some(1.618));
        } else {
            panic!("Expected property value");
        }
    } else {
        panic!("Expected property");
    }
}

#[test]
fn test_boolean_values_as_arguments() {
    // Test true
    let doc = kdl! {
        node #true
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_bool(), Some(true));
    } else {
        panic!("Expected boolean argument");
    }

    // Test false
    let doc = kdl! {
        node #false
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_bool(), Some(false));
    } else {
        panic!("Expected boolean argument");
    }
}

#[test]
fn test_boolean_values_as_properties() {
    // Test true as property value
    let doc = kdl! {
        node enabled=#true
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(prop) = node.entries()[0].name() {
        assert_eq!(prop.value(), "enabled");
        if let Some(value) = node.entries()[0].value() {
            assert_eq!(value.as_bool(), Some(true));
        } else {
            panic!("Expected property value");
        }
    } else {
        panic!("Expected property");
    }

    // Test false as property value
    let doc = kdl! {
        node debug=#false
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(prop) = node.entries()[0].name() {
        assert_eq!(prop.value(), "debug");
        if let Some(value) = node.entries()[0].value() {
            assert_eq!(value.as_bool(), Some(false));
        } else {
            panic!("Expected property value");
        }
    } else {
        panic!("Expected property");
    }
}

#[test]
fn test_null_values_as_arguments() {
    // Test null
    let doc = kdl! {
        node #null
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert!(arg.as_string().is_none());
        assert!(arg.as_i64().is_none());
        assert!(arg.as_f64().is_none());
        assert!(arg.as_bool().is_none());
    } else {
        panic!("Expected null argument");
    }
}

#[test]
fn test_null_values_as_properties() {
    // Test null as property value
    let doc = kdl! {
        node optional=#null
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(prop) = node.entries()[0].name() {
        assert_eq!(prop.value(), "optional");
        if let Some(value) = node.entries()[0].value() {
            assert!(value.as_string().is_none());
            assert!(value.as_i64().is_none());
            assert!(value.as_f64().is_none());
            assert!(value.as_bool().is_none());
        } else {
            panic!("Expected property value");
        }
    } else {
        panic!("Expected property");
    }
}

#[test]
fn test_type_annotated_values_as_arguments() {
    // Test type-annotated string
    let doc = kdl! {
        node (string)"hello"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_string(), Some("hello"));
        assert_eq!(arg.type_name().unwrap().value(), "string");
    } else {
        panic!("Expected type-annotated string argument");
    }

    // Test type-annotated number
    let doc = kdl! {
        node (i32)42
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(42));
        assert_eq!(arg.type_name().unwrap().value(), "i32");
    } else {
        panic!("Expected type-annotated number argument");
    }

    // Test type-annotated boolean
    let doc = kdl! {
        node (bool)#true
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_bool(), Some(true));
        assert_eq!(arg.type_name().unwrap().value(), "bool");
    } else {
        panic!("Expected type-annotated boolean argument");
    }

    // Test type-annotated null
    let doc = kdl! {
        node (option)#null
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert!(arg.as_string().is_none());
        assert_eq!(arg.type_name().unwrap().value(), "option");
    } else {
        panic!("Expected type-annotated null argument");
    }
}

#[test]
fn test_type_annotated_values_as_properties() {
    // Test type-annotated string property
    let doc = kdl! {
        node name=(string)"Alice"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(prop) = node.entries()[0].name() {
        assert_eq!(prop.value(), "name");
        if let Some(value) = node.entries()[0].value() {
            assert_eq!(value.as_string(), Some("Alice"));
            assert_eq!(value.type_name().unwrap().value(), "string");
        } else {
            panic!("Expected property value");
        }
    } else {
        panic!("Expected property");
    }

    // Test type-annotated number property
    let doc = kdl! {
        node age=(u8)25
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(prop) = node.entries()[0].name() {
        assert_eq!(prop.value(), "age");
        if let Some(value) = node.entries()[0].value() {
            assert_eq!(value.as_i64(), Some(25));
            assert_eq!(value.type_name().unwrap().value(), "u8");
        } else {
            panic!("Expected property value");
        }
    } else {
        panic!("Expected property");
    }
}

#[test]
fn test_reserved_type_annotations() {
    // Test numeric type annotations from section 3.8
    seq!(N in 8, 16, 32, 64, 128 {
        // Test signed integers
        let doc = kdl! {
            node (i~N)42
        };
        assert_eq!(doc.nodes().len(), 1);
        let node = &doc.nodes()[0];
        assert_eq!(node.entries().len(), 1);
        if let Some(arg) = node.entries()[0].value() {
            assert_eq!(arg.as_i64(), Some(42));
            assert_eq!(arg.type_name().unwrap().value(), stringify!(i~N));
        } else {
            panic!("Expected i{} argument", N);
        }

        // Test unsigned integers
        let doc = kdl! {
            node (u~N)42
        };
        assert_eq!(doc.nodes().len(), 1);
        let node = &doc.nodes()[0];
        assert_eq!(node.entries().len(), 1);
        if let Some(arg) = node.entries()[0].value() {
            assert_eq!(arg.as_i64(), Some(42));
            assert_eq!(arg.type_name().unwrap().value(), stringify!(u~N));
        } else {
            panic!("Expected u{} argument", N);
        }
    });

    // Test floating point type annotations
    let doc = kdl! {
        node (f32)3.14
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_f64(), Some(3.14));
        assert_eq!(arg.type_name().unwrap().value(), "f32");
    } else {
        panic!("Expected f32 argument");
    }

    let doc = kdl! {
        node (f64)3.14159
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_f64(), Some(3.14159));
        assert_eq!(arg.type_name().unwrap().value(), "f64");
    } else {
        panic!("Expected f64 argument");
    }

    // Test platform-dependent types
    let doc = kdl! {
        node (isize)42
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(42));
        assert_eq!(arg.type_name().unwrap().value(), "isize");
    } else {
        panic!("Expected isize argument");
    }

    let doc = kdl! {
        node (usize)42
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(42));
        assert_eq!(arg.type_name().unwrap().value(), "usize");
    } else {
        panic!("Expected usize argument");
    }
}

#[test]
fn test_string_type_annotations() {
    // Test some string-specific type annotations from section 3.8.3
    let doc = kdl! {
        node (date-time)"2023-01-01T12:00:00Z"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_string(), Some("2023-01-01T12:00:00Z"));
        assert_eq!(arg.type_name().unwrap().value(), "date-time");
    } else {
        panic!("Expected date-time argument");
    }

    let doc = kdl! {
        node (email)"user@example.com"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_string(), Some("user@example.com"));
        assert_eq!(arg.type_name().unwrap().value(), "email");
    } else {
        panic!("Expected email argument");
    }

    let doc = kdl! {
        node (url)"https://example.com"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_string(), Some("https://example.com"));
        assert_eq!(arg.type_name().unwrap().value(), "url");
    } else {
        panic!("Expected url argument");
    }
}

#[test]
fn test_mixed_value_types() {
    // Test multiple values of different types as arguments
    let doc = kdl! {
        node "string" 42 #true #null 3.14
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5);

    if let Some(arg1) = node.entries()[0].value() {
        assert_eq!(arg1.as_string(), Some("string"));
    } else {
        panic!("Expected string argument");
    }

    if let Some(arg2) = node.entries()[1].value() {
        assert_eq!(arg2.as_i64(), Some(42));
    } else {
        panic!("Expected number argument");
    }

    if let Some(arg3) = node.entries()[2].value() {
        assert_eq!(arg3.as_bool(), Some(true));
    } else {
        panic!("Expected boolean argument");
    }

    if let Some(arg4) = node.entries()[3].value() {
        assert!(arg4.as_string().is_none());
        assert!(arg4.as_i64().is_none());
        assert!(arg4.as_bool().is_none());
    } else {
        panic!("Expected null argument");
    }

    if let Some(arg5) = node.entries()[4].value() {
        assert_eq!(arg5.as_f64(), Some(3.14));
    } else {
        panic!("Expected float argument");
    }

    // Test mixed value types as properties
    let doc = kdl! {
        node name="Alice" age=30 active=#true data=#null ratio=1.5
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5);

    // Verify each property
    for entry in node.entries() {
        if let Some(prop_name) = entry.name() {
            match prop_name.value() {
                "name" => {
                    if let Some(value) = entry.value() {
                        assert_eq!(value.as_string(), Some("Alice"));
                    } else {
                        panic!("Expected name value");
                    }
                }
                "age" => {
                    if let Some(value) = entry.value() {
                        assert_eq!(value.as_i64(), Some(30));
                    } else {
                        panic!("Expected age value");
                    }
                }
                "active" => {
                    if let Some(value) = entry.value() {
                        assert_eq!(value.as_bool(), Some(true));
                    } else {
                        panic!("Expected active value");
                    }
                }
                "data" => {
                    if let Some(value) = entry.value() {
                        assert!(value.as_string().is_none());
                        assert!(value.as_i64().is_none());
                        assert!(value.as_bool().is_none());
                    } else {
                        panic!("Expected data value");
                    }
                }
                "ratio" => {
                    if let Some(value) = entry.value() {
                        assert_eq!(value.as_f64(), Some(1.5));
                    } else {
                        panic!("Expected ratio value");
                    }
                }
                _ => panic!("Unexpected property name: {}", prop_name.value()),
            }
        } else {
            panic!("Expected property entry");
        }
    }
}

#[test]
fn test_string_values_as_node_names() {
    // Test identifier string as node name
    let doc = kdl! {
        identifier_node
    };
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(doc.nodes()[0].name().value(), "identifier_node");

    // Test quoted string as node name
    let doc = kdl! {
        "quoted node name"
    };
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(doc.nodes()[0].name().value(), "quoted node name");

    // Test empty string as node name
    let doc = kdl! {
        ""
    };
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(doc.nodes()[0].name().value(), "");
}

#[test]
fn test_string_values_as_property_keys() {
    // Test identifier string as property key
    let doc = kdl! {
        node identifier_key="value"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(prop) = node.entries()[0].name() {
        assert_eq!(prop.value(), "identifier_key");
    } else {
        panic!("Expected property key");
    }

    // Test quoted string as property key
    let doc = kdl! {
        node "quoted key"="value"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(prop) = node.entries()[0].name() {
        assert_eq!(prop.value(), "quoted key");
    } else {
        panic!("Expected property key");
    }
}

#[test]
fn test_value_edge_cases() {
    // Test large numbers
    let doc = kdl! {
        node 9223372036854775807
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(9223372036854775807));
    } else {
        panic!("Expected large number argument");
    }

    // Test very small decimal number
    let doc = kdl! {
        node 0.000001
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_f64(), Some(0.000001));
    } else {
        panic!("Expected small decimal argument");
    }

    // Test number with underscores
    let doc = kdl! {
        node 1_000_000
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(1000000));
    } else {
        panic!("Expected number with underscores argument");
    }

    // Test hex with underscores
    let doc = kdl! {
        node 0xFF_FF_FF
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    if let Some(arg) = node.entries()[0].value() {
        assert_eq!(arg.as_i64(), Some(16777215));
    } else {
        panic!("Expected hex with underscores argument");
    }
}

// Error testing - These should fail compilation
#[test]
fn test_invalid_value_syntax_compile_errors() {
    // Note: These are tested by attempting compilation and expecting errors
    // The actual implementation would use compile_fail doctests or similar

    // Test that non-string values cannot be used as node names
    let invalid_number_name = quote! {
        kdl! {
            42 "value"
        }
    };
    let result = kdl_impl2(invalid_number_name);
    assert!(result.is_err(), "Number should not be valid as node name");

    let invalid_bool_name = quote! {
        kdl! {
            #true "value"
        }
    };
    let result = kdl_impl2(invalid_bool_name);
    assert!(result.is_err(), "Boolean should not be valid as node name");

    let invalid_null_name = quote! {
        kdl! {
            #null "value"
        }
    };
    let result = kdl_impl2(invalid_null_name);
    assert!(result.is_err(), "Null should not be valid as node name");

    // Test that non-string values cannot be used as property keys
    let invalid_number_key = quote! {
        kdl! {
            node 42="value"
        }
    };
    let result = kdl_impl2(invalid_number_key);
    assert!(result.is_err(), "Number should not be valid as property key");

    let invalid_bool_key = quote! {
        kdl! {
            node #true="value"
        }
    };
    let result = kdl_impl2(invalid_bool_key);
    assert!(result.is_err(), "Boolean should not be valid as property key");

    let invalid_null_key = quote! {
        kdl! {
            node #null="value"
        }
    };
    let result = kdl_impl2(invalid_null_key);
    assert!(result.is_err(), "Null should not be valid as property key");
}

#[test]
fn test_numeric_literal_edge_cases() {
    // Test that leading decimal point is invalid (per spec)
    let invalid_leading_decimal = quote! {
        kdl! {
            node .123
        }
    };
    let result = kdl_impl2(invalid_leading_decimal);
    assert!(result.is_err(), "Leading decimal point should be invalid");

    // Test that bare identifiers inf, -inf, nan are invalid
    let invalid_inf_identifier = quote! {
        kdl! {
            node inf
        }
    };
    let result = kdl_impl2(invalid_inf_identifier);
    assert!(result.is_err(), "Bare 'inf' identifier should be invalid");

    let invalid_neg_inf_identifier = quote! {
        kdl! {
            node -inf
        }
    };
    let result = kdl_impl2(invalid_neg_inf_identifier);
    assert!(result.is_err(), "Bare '-inf' identifier should be invalid");

    let invalid_nan_identifier = quote! {
        kdl! {
            node nan
        }
    };
    let result = kdl_impl2(invalid_nan_identifier);
    assert!(result.is_err(), "Bare 'nan' identifier should be invalid");
}

#[test]
fn test_multiple_type_annotations_error() {
    // Test that multiple type annotations are invalid
    let invalid_multiple_annotations = quote! {
        kdl! {
            node (i32)(u32)42
        }
    };
    let result = kdl_impl2(invalid_multiple_annotations);
    assert!(result.is_err(), "Multiple type annotations should be invalid");
}

#[test]
fn test_empty_type_annotation_error() {
    // Test that empty type annotations are invalid
    let invalid_empty_annotation = quote! {
        kdl! {
            node ()42
        }
    };
    let result = kdl_impl2(invalid_empty_annotation);
    assert!(result.is_err(), "Empty type annotation should be invalid");
}