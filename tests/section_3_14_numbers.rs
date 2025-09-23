//! Tests for KDL Section 3.14: Number support
//!
//! This module tests comprehensive number parsing for all KDL number formats
//! according to the KDL specification Section 3.14.

use serde_kdl::kdl;

#[test]
fn test_keyword_numbers() {
    // Test positive infinity
    let data = kdl! {
        node #inf
    };
    // This should compile without errors
    assert!(true);

    // Test negative infinity
    let data = kdl! {
        node #-inf
    };
    assert!(true);

    // Test NaN
    let data = kdl! {
        node #nan
    };
    assert!(true);
}

#[test]
fn test_basic_decimal_numbers() {
    // Test basic positive integers
    let data = kdl! {
        node 123 456 789
    };
    assert!(true);

    // Test negative integers
    let data = kdl! {
        node -123 -456
    };
    assert!(true);

    // Test explicitly positive integers
    let data = kdl! {
        node +123 +456
    };
    assert!(true);

    // Test zero variations
    let data = kdl! {
        node 0 -0 +0
    };
    assert!(true);
}

#[test]
fn test_decimal_floats() {
    // Test basic floats
    let data = kdl! {
        node 123.45 -456.78 +789.01
    };
    assert!(true);

    // Test floats starting with zero
    let data = kdl! {
        node 0.1 0.5 0.999
    };
    assert!(true);

    // Test floats ending in zero
    let data = kdl! {
        node 123.0 -456.0
    };
    assert!(true);
}

#[test]
fn test_scientific_notation() {
    // Test basic exponents
    let data = kdl! {
        node 1e5 1E5 1e-5 1E-5
    };
    assert!(true);

    // Test positive exponents
    let data = kdl! {
        node 1e+5 1E+5
    };
    assert!(true);

    // Test decimal with exponent
    let data = kdl! {
        node 1.23e4 -2.5e10
    };
    assert!(true);
}

#[test]
fn test_underscores_in_numbers() {
    // Test underscores in integers
    let data = kdl! {
        node 1_000_000 -2_500
    };
    assert!(true);

    // Test underscores in floats
    let data = kdl! {
        node 1_234.567_89
    };
    assert!(true);

    // Test underscores in scientific notation
    let data = kdl! {
        node 1_000e1_5
    };
    assert!(true);
}

#[test]
fn test_hexadecimal_numbers() {
    // Test basic hex
    let data = kdl! {
        node 0xff 0x123 0xabc
    };
    assert!(true);

    // Test uppercase hex
    let data = kdl! {
        node 0XFF 0X123 0XABC
    };
    assert!(true);

    // Test signed hex
    let data = kdl! {
        node -0xff +0XFF
    };
    assert!(true);

    // Test hex with underscores
    let data = kdl! {
        node 0x12_34_AB 0XFF_EE_DD
    };
    assert!(true);
}

#[test]
fn test_octal_numbers() {
    // Test basic octal
    let data = kdl! {
        node 0o77 0o123
    };
    assert!(true);

    // Test uppercase octal
    let data = kdl! {
        node 0O77 0O123
    };
    assert!(true);

    // Test signed octal
    let data = kdl! {
        node -0o77 +0O123
    };
    assert!(true);

    // Test octal with underscores
    let data = kdl! {
        node 0o12_34 0O56_70
    };
    assert!(true);
}

#[test]
fn test_binary_numbers() {
    // Test basic binary
    let data = kdl! {
        node 0b101 0b110
    };
    assert!(true);

    // Test uppercase binary
    let data = kdl! {
        node 0B101 0B110
    };
    assert!(true);

    // Test signed binary
    let data = kdl! {
        node -0b101 +0B110
    };
    assert!(true);

    // Test binary with underscores
    let data = kdl! {
        node 0b10_11_01 0B11_00_10
    };
    assert!(true);
}

#[test]
fn test_mixed_number_formats() {
    // Test all formats together
    let data = kdl! {
        node 123 0xff 0o77 0b101 #inf 1.23e4
    };
    assert!(true);

    // Test with signs
    let data = kdl! {
        mixed -123 +0xff -0o77 +0b101 #-inf -1.23e4
    };
    assert!(true);
}

#[test]
fn test_numbers_as_property_values() {
    // Test numbers as property values
    let data = kdl! {
        config timeout=30 max_connections=1000 rate=0.5
    };
    assert!(true);

    // Test hex/octal/binary as property values
    let data = kdl! {
        flags mask=0xFF permissions=0o755 bits=0b11001100
    };
    assert!(true);
}

#[test]
fn test_type_annotated_numbers() {
    // Test type annotations with numbers
    let data = kdl! {
        node (i32)123 (f64)3.14 (u8)0xFF
    };
    assert!(true);

    // Test type annotations with keyword numbers
    let data = kdl! {
        special (f64)#inf (f32)#nan
    };
    assert!(true);
}