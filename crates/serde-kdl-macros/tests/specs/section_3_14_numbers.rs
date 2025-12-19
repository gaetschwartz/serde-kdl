//! Tests for KDL Section 3.14: Number
//!
//! This module tests all KDL number formats:
//! - Keywords: #inf, #-inf, #nan
//! - Decimal: integers, floats, scientific notation
//! - Hexadecimal: 0x prefix
//! - Octal: 0o prefix
//! - Binary: 0b prefix

use super::doc_to_string;
use insta::assert_snapshot;
use serde_kdl_macros::kdl;

#[test]
fn test_decimal_numbers_and_keywords() {
    // Decimal integers with signs
    let doc = kdl! {
        integers 42 -123 +456 0
    };
    assert_snapshot!(doc_to_string(doc), @"integers 42 -123 456 0");

    // Decimal floats
    let doc = kdl! {
        floats 3.14 -2.5 +1.0 0.999
    };
    assert_snapshot!(doc_to_string(doc), @"floats 3.14 -2.5 1.0 0.999");

    // Scientific notation
    let doc = kdl! {
        scientific 1e5 1.23e4 -2.5e-10 +3.14E+2
    };
    assert_snapshot!(doc_to_string(doc), @r#"scientific 100000.0 12300.0 -2.5e-10 314.0"#);

    // Underscores in decimals
    let doc = kdl! {
        underscored 1_000_000 1_234.567_89 1_000e1_0
    };
    assert_snapshot!(doc_to_string(doc), @r#"underscored 1000000 1234.56789 10000000000000.0"#);

    // Special keyword numbers
    let doc = kdl! {
        keywords #inf #-inf #nan
    };
    assert_snapshot!(doc_to_string(doc), @"keywords #inf #-inf #nan");
}

#[test]
fn test_radix_numbers() {
    // Hexadecimal (lowercase and uppercase x/X, with various cases)
    let doc = kdl! {
        hex 0xff 0XFF 0xaBcDeF -0x123 +0XAB 0x12_34_AB
    };
    assert_snapshot!(doc_to_string(doc), @"hex 255 255 11259375 -291 171 1193131");

    // Octal (lowercase and uppercase o/O)
    let doc = kdl! {
        octal 0o77 0O123 -0o456 +0O70 0o12_34
    };
    assert_snapshot!(doc_to_string(doc), @"octal 63 83 -302 56 668");

    // Binary (lowercase and uppercase b/B)
    let doc = kdl! {
        binary 0b101 0B1111 -0b110 +0B11 0b10_11_01
    };
    assert_snapshot!(doc_to_string(doc), @"binary 5 15 -6 3 45");
}

#[test]
fn test_numbers_in_context() {
    // Numbers as node arguments and property values
    let doc = kdl! {
        server 8080 "localhost" {
            config timeout=30 max_connections=1_000 rate=0.5
            flags mask=0xFF permissions=0o755 bits=0b11001100
        }
    };
    assert_snapshot!(doc_to_string(doc), @r"
    server 8080 localhost {
        config timeout=30 max_connections=1000 rate=0.5
        flags mask=255 permissions=493 bits=204
    }
    ");

    // Type-annotated numbers
    let doc = kdl! {
        typed (i32)123 (f64)3.14 (u8)0xFF (f64)#inf
    };
    assert_snapshot!(doc_to_string(doc), @"typed (i32)123 (f64)3.14 (u8)255 (f64)#inf");

    // Mixed number formats in one node
    let doc = kdl! {
        mixed 123 0xff 0o77 0b101 #inf 1.23e4 -456 +0x12_AB
    };
    assert_snapshot!(doc_to_string(doc), @"mixed 123 255 63 5 #inf 12300.0 -456 4779");
}
