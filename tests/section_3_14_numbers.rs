//! Tests for KDL Section 3.14: Number support
//!
//! This module tests comprehensive number parsing for all KDL number formats
//! according to the KDL specification Section 3.14.

use serde_kdl::kdl;

#[test]
fn test_keyword_numbers() {
    // Test positive infinity
    let _data = kdl! {
        node #inf
    };
    // This should compile without errors

    // Test negative infinity
    let _data = kdl! {
        node #-inf
    };

    // Test NaN
    let _data = kdl! {
        node #nan
    };
}

#[test]
fn test_basic_decimal_numbers() {
    // Test basic positive integers
    let _data = kdl! {
        node 123 456 789
    };

    // Test negative integers
    let _data = kdl! {
        node -123 -456
    };

    // Test explicitly positive integers
    let _data = kdl! {
        node +123 +456
    };

    // Test zero variations
    let _data = kdl! {
        node 0 -0 +0
    };
}

#[test]
fn test_decimal_floats() {
    // Test basic floats
    let _data = kdl! {
        node 123.45 -456.78 +789.01
    };

    // Test floats starting with zero
    let _data = kdl! {
        node 0.1 0.5 0.999
    };

    // Test floats ending in zero
    let _data = kdl! {
        node 123.0 -456.0
    };
}

#[test]
fn test_scientific_notation() {
    // Test basic exponents
    let _data = kdl! {
        node 1e5 1E5 1e-5 1E-5
    };

    // Test positive exponents
    let _data = kdl! {
        node 1e+5 1E+5
    };

    // Test decimal with exponent
    let _data = kdl! {
        node 1.23e4 -2.5e10
    };
}

#[test]
fn test_underscores_in_numbers() {
    // Test underscores in integers
    let _data = kdl! {
        node 1_000_000 -2_500
    };

    // Test underscores in floats
    let _data = kdl! {
        node 1_234.567_89
    };

    // Test underscores in scientific notation
    let _data = kdl! {
        node 1_000e1_5
    };
}

#[test]
fn test_hexadecimal_numbers() {
    // Test basic hex
    let _data = kdl! {
        node 0xff 0x123 0xabc
    };

    // Test uppercase hex
    let _data = kdl! {
        node 0XFF 0X123 0XABC
    };

    // Test signed hex
    let _data = kdl! {
        node -0xff +0XFF
    };

    // Test hex with underscores
    let _data = kdl! {
        node 0x12_34_AB 0XFF_EE_DD
    };
}

#[test]
fn test_octal_numbers() {
    // Test basic octal
    let _data = kdl! {
        node 0o77 0o123
    };

    // Test uppercase octal
    let _data = kdl! {
        node 0O77 0O123
    };

    // Test signed octal
    let _data = kdl! {
        node -0o77 +0O123
    };

    // Test octal with underscores
    let _data = kdl! {
        node 0o12_34 0O56_70
    };
}

#[test]
fn test_binary_numbers() {
    // Test basic binary
    let _data = kdl! {
        node 0b101 0b110
    };

    // Test uppercase binary
    let _data = kdl! {
        node 0B101 0B110
    };

    // Test signed binary
    let _data = kdl! {
        node -0b101 +0B110
    };

    // Test binary with underscores
    let _data = kdl! {
        node 0b10_11_01 0B11_00_10
    };
}

#[test]
fn test_mixed_number_formats() {
    // Test all formats together
    let _data = kdl! {
        node 123 0xff 0o77 0b101 #inf 1.23e4
    };

    // Test with signs
    let _data = kdl! {
        mixed -123 +0xff -0o77 +0b101 #-inf -1.23e4
    };
}

#[test]
fn test_numbers_as_property_values() {
    // Test numbers as property values
    let _data = kdl! {
        config timeout=30 max_connections=1000 rate=0.5
    };

    // Test hex/octal/binary as property values
    let _data = kdl! {
        flags mask=0xFF permissions=0o755 bits=0b11001100
    };
}

#[test]
fn test_type_annotated_numbers() {
    // Test type annotations with numbers
    let _data = kdl! {
        node (i32)123 (f64)2.5 (u8)0xFF
    };

    // Test type annotations with keyword numbers
    let _data = kdl! {
        special (f64)#inf (f32)#nan
    };
}
