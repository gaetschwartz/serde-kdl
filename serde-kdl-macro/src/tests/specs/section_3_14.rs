//! Tests for KDL Section 3.14: Number
//!
//! This module contains comprehensive tests for all KDL number formats:
//! - Keywords: #inf, #-inf, #nan
//! - Decimal: 123, 123.45, 1.23e4, 1.23E-5, +123, -456
//! - Hexadecimal: 0x1F, 0XFF, +0x123, -0xABC, 0x12_34
//! - Octal: 0o17, 0O77, +0o123, -0o456, 0o12_34
//! - Binary: 0b101, 0B111, +0b101, -0b110, 0b10_11

use crate::tests::specs::kdl_impl2;
use crate::assert_eq_tk;

#[test]
fn test_keyword_numbers() {
    // Test positive infinity
    let result = kdl_impl2(quote::quote! {
        node #inf
    });
    assert!(result.is_ok());

    // Test negative infinity
    let result = kdl_impl2(quote::quote! {
        node #-inf
    });
    assert!(result.is_ok());

    // Test NaN
    let result = kdl_impl2(quote::quote! {
        node #nan
    });
    assert!(result.is_ok());
}

#[test]
fn test_decimal_integers() {
    // Test positive integers
    let result = kdl_impl2(quote::quote! {
        node 123 456 789
    });
    assert!(result.is_ok());

    // Test negative integers
    let result = kdl_impl2(quote::quote! {
        node -123 -456 -789
    });
    assert!(result.is_ok());

    // Test explicitly positive integers
    let result = kdl_impl2(quote::quote! {
        node +123 +456 +789
    });
    assert!(result.is_ok());

    // Test zero
    let result = kdl_impl2(quote::quote! {
        node 0 -0 +0
    });
    assert!(result.is_ok());
}

#[test]
fn test_decimal_floats() {
    // Test basic floats
    let result = kdl_impl2(quote::quote! {
        node 123.45 -456.78 +789.01
    });
    assert!(result.is_ok());

    // Test floats starting with decimal
    let result = kdl_impl2(quote::quote! {
        node 0.1 0.5 0.999
    });
    assert!(result.is_ok());

    // Test floats ending in zero
    let result = kdl_impl2(quote::quote! {
        node 123.0 -456.0 +789.0
    });
    assert!(result.is_ok());
}

#[test]
fn test_scientific_notation() {
    // Test basic scientific notation
    let result = kdl_impl2(quote::quote! {
        node 1e5 1E5 1e-5 1E-5
    });
    assert!(result.is_ok());

    // Test with explicit positive exponent
    let result = kdl_impl2(quote::quote! {
        node 1e+5 1E+5
    });
    assert!(result.is_ok());

    // Test with decimal point and exponent
    let result = kdl_impl2(quote::quote! {
        node 1.23e4 1.23E-5 -2.5e10 +3.14E-2
    });
    assert!(result.is_ok());
}

#[test]
fn test_underscores_in_decimals() {
    // Test underscores in integers
    let result = kdl_impl2(quote::quote! {
        node 1_000_000 -2_500 +1_2_3
    });
    assert!(result.is_ok());

    // Test underscores in floats
    let result = kdl_impl2(quote::quote! {
        node 1_234.567_89 -9_876.543_21
    });
    assert!(result.is_ok());

    // Test underscores in scientific notation
    let result = kdl_impl2(quote::quote! {
        node 1_000e1_5 2.5_0E-1_0
    });
    assert!(result.is_ok());
}

#[test]
fn test_hexadecimal_numbers() {
    // Test basic hex (lowercase x)
    let result = kdl_impl2(quote::quote! {
        node 0xff 0x123 0xabc
    });
    assert!(result.is_ok());

    // Test basic hex (uppercase X)
    let result = kdl_impl2(quote::quote! {
        node 0XFF 0X123 0XABC
    });
    assert!(result.is_ok());

    // Test negative hex
    let result = kdl_impl2(quote::quote! {
        node -0xff -0XFF
    });
    assert!(result.is_ok());

    // Test positive hex
    let result = kdl_impl2(quote::quote! {
        node +0xff +0XFF
    });
    assert!(result.is_ok());

    // Test hex with underscores
    let result = kdl_impl2(quote::quote! {
        node 0x12_34_AB 0XFF_EE_DD
    });
    assert!(result.is_ok());

    // Test mixed case hex
    let result = kdl_impl2(quote::quote! {
        node 0xaBcDeF 0XaBcDeF
    });
    assert!(result.is_ok());
}

#[test]
fn test_octal_numbers() {
    // Test basic octal (lowercase o)
    let result = kdl_impl2(quote::quote! {
        node 0o77 0o123 0o456
    });
    assert!(result.is_ok());

    // Test basic octal (uppercase O)
    let result = kdl_impl2(quote::quote! {
        node 0O77 0O123 0O456
    });
    assert!(result.is_ok());

    // Test negative octal
    let result = kdl_impl2(quote::quote! {
        node -0o77 -0O77
    });
    assert!(result.is_ok());

    // Test positive octal
    let result = kdl_impl2(quote::quote! {
        node +0o77 +0O77
    });
    assert!(result.is_ok());

    // Test octal with underscores
    let result = kdl_impl2(quote::quote! {
        node 0o12_34 0O56_70
    });
    assert!(result.is_ok());
}

#[test]
fn test_binary_numbers() {
    // Test basic binary (lowercase b)
    let result = kdl_impl2(quote::quote! {
        node 0b101 0b110 0b111
    });
    assert!(result.is_ok());

    // Test basic binary (uppercase B)
    let result = kdl_impl2(quote::quote! {
        node 0B101 0B110 0B111
    });
    assert!(result.is_ok());

    // Test negative binary
    let result = kdl_impl2(quote::quote! {
        node -0b101 -0B101
    });
    assert!(result.is_ok());

    // Test positive binary
    let result = kdl_impl2(quote::quote! {
        node +0b101 +0B101
    });
    assert!(result.is_ok());

    // Test binary with underscores
    let result = kdl_impl2(quote::quote! {
        node 0b10_11_01 0B11_00_10
    });
    assert!(result.is_ok());
}

#[test]
fn test_mixed_number_formats() {
    // Test all number formats in one node
    let result = kdl_impl2(quote::quote! {
        node 123 0xff 0o77 0b101 #inf 1.23e4
    });
    assert!(result.is_ok());

    // Test with signs
    let result = kdl_impl2(quote::quote! {
        node -123 +0xff -0o77 +0b101 #-inf -1.23e4
    });
    assert!(result.is_ok());

    // Test with underscores
    let result = kdl_impl2(quote::quote! {
        node 1_000 0x12_34 0o56_70 0b10_11 1_000.5_0
    });
    assert!(result.is_ok());
}

#[test]
fn test_number_edge_cases() {
    // Test large numbers
    let result = kdl_impl2(quote::quote! {
        node 999999999999999999 1.7976931348623157e308
    });
    assert!(result.is_ok());

    // Test small numbers
    let result = kdl_impl2(quote::quote! {
        node 0.000000000000001 1e-300
    });
    assert!(result.is_ok());

    // Test minimum values
    let result = kdl_impl2(quote::quote! {
        node 0x0 0o0 0b0
    });
    assert!(result.is_ok());

    // Test maximum single-digit values
    let result = kdl_impl2(quote::quote! {
        node 0xF 0o7 0b1
    });
    assert!(result.is_ok());
}

#[test]
fn test_invalid_decimal_formats() {
    // Test invalid: numbers starting with decimal point
    let result = kdl_impl2(quote::quote! {
        node .1
    });
    assert!(result.is_err());

    // Test invalid: numbers ending with decimal point
    let result = kdl_impl2(quote::quote! {
        node 1.
    });
    assert!(result.is_err());

    // Test invalid: multiple decimal points
    let result = kdl_impl2(quote::quote! {
        node 1.2.3
    });
    assert!(result.is_err());

    // Test invalid: exponent without digits
    let result = kdl_impl2(quote::quote! {
        node 1e
    });
    assert!(result.is_err());
}

#[test]
fn test_invalid_hex_formats() {
    // Test invalid: hex without digits
    let result = kdl_impl2(quote::quote! {
        node 0x
    });
    assert!(result.is_err());

    // Test invalid: hex with invalid digits
    let result = kdl_impl2(quote::quote! {
        node 0xG
    });
    assert!(result.is_err());

    // Test invalid: hex with decimal point
    let result = kdl_impl2(quote::quote! {
        node 0x12.34
    });
    assert!(result.is_err());
}

#[test]
fn test_invalid_octal_formats() {
    // Test invalid: octal without digits
    let result = kdl_impl2(quote::quote! {
        node 0o
    });
    assert!(result.is_err());

    // Test invalid: octal with invalid digits
    let result = kdl_impl2(quote::quote! {
        node 0o8
    });
    assert!(result.is_err());

    // Test invalid: octal with decimal point
    let result = kdl_impl2(quote::quote! {
        node 0o12.34
    });
    assert!(result.is_err());
}

#[test]
fn test_invalid_binary_formats() {
    // Test invalid: binary without digits
    let result = kdl_impl2(quote::quote! {
        node 0b
    });
    assert!(result.is_err());

    // Test invalid: binary with invalid digits
    let result = kdl_impl2(quote::quote! {
        node 0b2
    });
    assert!(result.is_err());

    // Test invalid: binary with decimal point
    let result = kdl_impl2(quote::quote! {
        node 0b10.11
    });
    assert!(result.is_err());
}

#[test]
fn test_invalid_keyword_numbers() {
    // Test invalid keyword numbers
    let result = kdl_impl2(quote::quote! {
        node #inf2
    });
    assert!(result.is_err());

    let result = kdl_impl2(quote::quote! {
        node #infinity
    });
    assert!(result.is_err());

    let result = kdl_impl2(quote::quote! {
        node #NaN
    });
    assert!(result.is_err()); // Case sensitive

    let result = kdl_impl2(quote::quote! {
        node #--inf
    });
    assert!(result.is_err()); // Double negative
}

#[test]
fn test_underscore_placement() {
    // Test invalid: underscores at start
    let result = kdl_impl2(quote::quote! {
        node _123
    });
    assert!(result.is_err());

    // Test invalid: underscores at end
    let result = kdl_impl2(quote::quote! {
        node 123_
    });
    assert!(result.is_err());

    // Test invalid: consecutive underscores
    let result = kdl_impl2(quote::quote! {
        node 12__34
    });
    assert!(result.is_err());

    // Test invalid: underscores around decimal point
    let result = kdl_impl2(quote::quote! {
        node 12_.34
    });
    assert!(result.is_err());

    let result = kdl_impl2(quote::quote! {
        node 12._34
    });
    assert!(result.is_err());
}

#[test]
fn test_numbers_as_node_values() {
    // Test numbers as node arguments
    let result = kdl_impl2(quote::quote! {
        server 123 "localhost" port=8080
    });
    assert!(result.is_ok());

    // Test numbers as property values
    let result = kdl_impl2(quote::quote! {
        config timeout=30 max_connections=1000 rate=0.5
    });
    assert!(result.is_ok());

    // Test hex/octal/binary as values
    let result = kdl_impl2(quote::quote! {
        flags mask=0xFF permissions=0o755 bits=0b11001100
    });
    assert!(result.is_ok());
}

#[test]
fn test_numbers_with_type_annotations() {
    // Test type-annotated numbers
    let result = kdl_impl2(quote::quote! {
        node (i32)123 (f64)3.14 (u8)0xFF
    });
    assert!(result.is_ok());

    // Test keyword numbers with type annotations
    let result = kdl_impl2(quote::quote! {
        node (f64)#inf (f32)#nan
    });
    assert!(result.is_ok());
}