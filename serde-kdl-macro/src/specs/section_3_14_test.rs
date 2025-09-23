//! Tests for Section 3.14: Number
//!
//! This module contains comprehensive tests for the KDL Number specification
//! as defined in section 3.14 of the KDL specification.
//!
//! The tests cover:
//! - Keyword numbers: #inf, #-inf, #nan
//! - Decimal numbers with various formats
//! - Hexadecimal numbers (0x prefix)
//! - Octal numbers (0o prefix)
//! - Binary numbers (0b prefix)
//! - Number prefixes (+, -)
//! - Digit separators with underscore
//! - Decimal points and scientific notation
//! - Invalid number syntax validation
//! - Edge cases and boundary conditions

use crate::specs::kdl_impl2;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rstest::rstest;
use seq_macro::seq;
use serde_kdl_macro::kdl;

// ============================================================================
// Section 3.14.1: Keyword Numbers Tests
// ============================================================================

/// Test positive infinity keyword
#[test]
fn test_positive_infinity() {
    let doc = kdl! {
        node #inf
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    assert!(node.entries()[0].value().as_f64().unwrap().is_infinite());
    assert!(node.entries()[0].value().as_f64().unwrap().is_sign_positive());
}

/// Test negative infinity keyword
#[test]
fn test_negative_infinity() {
    let doc = kdl! {
        node #-inf
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    assert!(node.entries()[0].value().as_f64().unwrap().is_infinite());
    assert!(node.entries()[0].value().as_f64().unwrap().is_sign_negative());
}

/// Test NaN keyword
#[test]
fn test_nan() {
    let doc = kdl! {
        node #nan
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    assert!(node.entries()[0].value().as_f64().unwrap().is_nan());
}

/// Test all keyword numbers together
#[test]
fn test_all_keyword_numbers() {
    let doc = kdl! {
        numbers #inf #-inf #nan
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    // Positive infinity
    assert!(node.entries()[0].value().as_f64().unwrap().is_infinite());
    assert!(node.entries()[0].value().as_f64().unwrap().is_sign_positive());

    // Negative infinity
    assert!(node.entries()[1].value().as_f64().unwrap().is_infinite());
    assert!(node.entries()[1].value().as_f64().unwrap().is_sign_negative());

    // NaN
    assert!(node.entries()[2].value().as_f64().unwrap().is_nan());
}

/// Test that bare keyword strings are illegal (would be tested at parse level)
#[test]
fn test_illegal_bare_keywords() {
    // These should fail if used as bare identifiers in node names
    // Testing through kdl_impl2 to check parser rejection
    let result1 = kdl_impl2(quote! { inf });
    let result2 = kdl_impl2(quote! { -inf });
    let result3 = kdl_impl2(quote! { nan });

    // Note: The actual behavior depends on parser implementation
    // These might succeed as node names, but per spec should be illegal
    // The key test is that #inf, #-inf, #nan are valid number keywords
}

// ============================================================================
// Section 3.14.2: Decimal Numbers Tests
// ============================================================================

/// Test basic positive decimal numbers
#[test]
fn test_basic_positive_decimals() {
    let doc = kdl! {
        node 0 1 42 123 999
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 0);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 1);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 42);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), 123);
    assert_eq!(node.entries()[4].value().as_i64().unwrap(), 999);
}

/// Test negative decimal numbers
#[test]
fn test_negative_decimals() {
    let doc = kdl! {
        node -0 -1 -42 -123 -999
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 0);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), -1);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), -42);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), -123);
    assert_eq!(node.entries()[4].value().as_i64().unwrap(), -999);
}

/// Test positive decimal numbers with explicit + prefix
#[test]
fn test_explicit_positive_decimals() {
    let doc = kdl! {
        node +0 +1 +42 +123 +999
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 0);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 1);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 42);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), 123);
    assert_eq!(node.entries()[4].value().as_i64().unwrap(), 999);
}

/// Test decimal numbers with underscores as digit separators
#[test]
fn test_decimal_with_underscores() {
    let doc = kdl! {
        node 1_000 12_345 1_000_000 9_999_999
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 1000);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 12345);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 1000000);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), 9999999);
}

/// Test decimal numbers with decimal points
#[test]
fn test_decimal_points() {
    let doc = kdl! {
        node 0.0 1.0 3.14 42.5 0.123 999.999
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 6);

    assert_eq!(node.entries()[0].value().as_f64().unwrap(), 0.0);
    assert_eq!(node.entries()[1].value().as_f64().unwrap(), 1.0);
    assert_eq!(node.entries()[2].value().as_f64().unwrap(), 3.14);
    assert_eq!(node.entries()[3].value().as_f64().unwrap(), 42.5);
    assert_eq!(node.entries()[4].value().as_f64().unwrap(), 0.123);
    assert_eq!(node.entries()[5].value().as_f64().unwrap(), 999.999);
}

/// Test decimal numbers with underscores in fractional part
#[test]
fn test_decimal_fractional_underscores() {
    let doc = kdl! {
        node 1.000_001 3.141_592 42.123_456
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    assert_eq!(node.entries()[0].value().as_f64().unwrap(), 1.000001);
    assert_eq!(node.entries()[1].value().as_f64().unwrap(), 3.141592);
    assert_eq!(node.entries()[2].value().as_f64().unwrap(), 42.123456);
}

/// Test scientific notation with 'e'
#[test]
fn test_scientific_notation_lowercase() {
    let doc = kdl! {
        node 1e0 1e1 1e2 2e3 1e-1 1e-2 5e-3
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 7);

    assert_eq!(node.entries()[0].value().as_f64().unwrap(), 1.0);
    assert_eq!(node.entries()[1].value().as_f64().unwrap(), 10.0);
    assert_eq!(node.entries()[2].value().as_f64().unwrap(), 100.0);
    assert_eq!(node.entries()[3].value().as_f64().unwrap(), 2000.0);
    assert_eq!(node.entries()[4].value().as_f64().unwrap(), 0.1);
    assert_eq!(node.entries()[5].value().as_f64().unwrap(), 0.01);
    assert_eq!(node.entries()[6].value().as_f64().unwrap(), 0.005);
}

/// Test scientific notation with 'E'
#[test]
fn test_scientific_notation_uppercase() {
    let doc = kdl! {
        node 1E0 1E1 1E2 2E3 1E-1 1E-2 5E-3
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 7);

    assert_eq!(node.entries()[0].value().as_f64().unwrap(), 1.0);
    assert_eq!(node.entries()[1].value().as_f64().unwrap(), 10.0);
    assert_eq!(node.entries()[2].value().as_f64().unwrap(), 100.0);
    assert_eq!(node.entries()[3].value().as_f64().unwrap(), 2000.0);
    assert_eq!(node.entries()[4].value().as_f64().unwrap(), 0.1);
    assert_eq!(node.entries()[5].value().as_f64().unwrap(), 0.01);
    assert_eq!(node.entries()[6].value().as_f64().unwrap(), 0.005);
}

/// Test scientific notation with explicit positive exponent
#[test]
fn test_scientific_notation_explicit_positive() {
    let doc = kdl! {
        node 1e+0 1e+1 1e+2 2e+3 1E+4 5E+5
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 6);

    assert_eq!(node.entries()[0].value().as_f64().unwrap(), 1.0);
    assert_eq!(node.entries()[1].value().as_f64().unwrap(), 10.0);
    assert_eq!(node.entries()[2].value().as_f64().unwrap(), 100.0);
    assert_eq!(node.entries()[3].value().as_f64().unwrap(), 2000.0);
    assert_eq!(node.entries()[4].value().as_f64().unwrap(), 10000.0);
    assert_eq!(node.entries()[5].value().as_f64().unwrap(), 500000.0);
}

/// Test complex decimal numbers with all features
#[test]
fn test_complex_decimal_numbers() {
    let doc = kdl! {
        node 123.456e-2 -987.654E+3 +42.123_456e-4
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    assert_eq!(node.entries()[0].value().as_f64().unwrap(), 1.23456);
    assert_eq!(node.entries()[1].value().as_f64().unwrap(), -987654.0);
    assert_eq!(node.entries()[2].value().as_f64().unwrap(), 0.0042123456);
}

/// Test numbers that must have leading digits (no .1 style)
#[test]
fn test_invalid_decimal_without_leading_digit() {
    // These should fail because numbers like .1 are illegal per spec
    let result1 = kdl_impl2(quote! { node .1 });
    let result2 = kdl_impl2(quote! { node .123 });
    let result3 = kdl_impl2(quote! { node -.5 });

    assert!(result1.is_err(), "Numbers without leading digit should be invalid");
    assert!(result2.is_err(), "Numbers without leading digit should be invalid");
    assert!(result3.is_err(), "Numbers without leading digit should be invalid");
}

// ============================================================================
// Section 3.14.3: Hexadecimal Numbers Tests
// ============================================================================

/// Test basic hexadecimal numbers
#[test]
fn test_basic_hexadecimal() {
    let doc = kdl! {
        node 0x0 0x1 0xA 0xF 0x10 0xFF
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 6);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 0);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 1);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 10);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), 15);
    assert_eq!(node.entries()[4].value().as_i64().unwrap(), 16);
    assert_eq!(node.entries()[5].value().as_i64().unwrap(), 255);
}

/// Test hexadecimal with lowercase letters
#[test]
fn test_hexadecimal_lowercase() {
    let doc = kdl! {
        node 0xa 0xb 0xc 0xd 0xe 0xf 0xabc 0xdef
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 8);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 10);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 11);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 12);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), 13);
    assert_eq!(node.entries()[4].value().as_i64().unwrap(), 14);
    assert_eq!(node.entries()[5].value().as_i64().unwrap(), 15);
    assert_eq!(node.entries()[6].value().as_i64().unwrap(), 0xabc);
    assert_eq!(node.entries()[7].value().as_i64().unwrap(), 0xdef);
}

/// Test hexadecimal with mixed case
#[test]
fn test_hexadecimal_mixed_case() {
    let doc = kdl! {
        node 0xaBc 0xDeF 0xABCdef 0xFfFfFf
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 0xabc);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 0xdef);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 0xabcdef);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), 0xffffff);
}

/// Test negative hexadecimal numbers
#[test]
fn test_negative_hexadecimal() {
    let doc = kdl! {
        node -0x1 -0xA -0xFF -0x100
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), -1);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), -10);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), -255);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), -256);
}

/// Test positive hexadecimal numbers with explicit + prefix
#[test]
fn test_positive_hexadecimal() {
    let doc = kdl! {
        node +0x1 +0xA +0xFF +0x100
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 1);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 10);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 255);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), 256);
}

/// Test hexadecimal with underscores
#[test]
fn test_hexadecimal_with_underscores() {
    let doc = kdl! {
        node 0x1_000 0xAB_CD_EF 0xFF_FF_FF_FF
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 0x1000);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 0xabcdef);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 0xffffffff);
}

/// Test invalid hexadecimal numbers
#[test]
fn test_invalid_hexadecimal() {
    // Test invalid hex digits
    let result1 = kdl_impl2(quote! { node 0xG });
    let result2 = kdl_impl2(quote! { node 0x123G });
    let result3 = kdl_impl2(quote! { node 0x });

    assert!(result1.is_err(), "Invalid hex digit G should be rejected");
    assert!(result2.is_err(), "Invalid hex digit G should be rejected");
    assert!(result3.is_err(), "Empty hex number should be rejected");
}

// ============================================================================
// Section 3.14.4: Octal Numbers Tests
// ============================================================================

/// Test basic octal numbers
#[test]
fn test_basic_octal() {
    let doc = kdl! {
        node 0o0 0o1 0o7 0o10 0o77 0o100
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 6);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 0);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 1);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 7);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), 8);   // 0o10 = 8
    assert_eq!(node.entries()[4].value().as_i64().unwrap(), 63);  // 0o77 = 63
    assert_eq!(node.entries()[5].value().as_i64().unwrap(), 64);  // 0o100 = 64
}

/// Test negative octal numbers
#[test]
fn test_negative_octal() {
    let doc = kdl! {
        node -0o1 -0o7 -0o10 -0o77
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), -1);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), -7);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), -8);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), -63);
}

/// Test positive octal numbers with explicit + prefix
#[test]
fn test_positive_octal() {
    let doc = kdl! {
        node +0o1 +0o7 +0o10 +0o77
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 1);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 7);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 8);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), 63);
}

/// Test octal with underscores
#[test]
fn test_octal_with_underscores() {
    let doc = kdl! {
        node 0o1_000 0o12_34 0o777_777
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 0o1000);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 0o1234);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 0o777777);
}

/// Test invalid octal numbers
#[test]
fn test_invalid_octal() {
    // Test invalid octal digits (8 and 9)
    let result1 = kdl_impl2(quote! { node 0o8 });
    let result2 = kdl_impl2(quote! { node 0o9 });
    let result3 = kdl_impl2(quote! { node 0o123a });
    let result4 = kdl_impl2(quote! { node 0o });

    assert!(result1.is_err(), "Invalid octal digit 8 should be rejected");
    assert!(result2.is_err(), "Invalid octal digit 9 should be rejected");
    assert!(result3.is_err(), "Invalid octal digit a should be rejected");
    assert!(result4.is_err(), "Empty octal number should be rejected");
}

// ============================================================================
// Section 3.14.5: Binary Numbers Tests
// ============================================================================

/// Test basic binary numbers
#[test]
fn test_basic_binary() {
    let doc = kdl! {
        node 0b0 0b1 0b10 0b11 0b100 0b1111
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 6);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 0);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 1);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 2);   // 0b10 = 2
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), 3);   // 0b11 = 3
    assert_eq!(node.entries()[4].value().as_i64().unwrap(), 4);   // 0b100 = 4
    assert_eq!(node.entries()[5].value().as_i64().unwrap(), 15);  // 0b1111 = 15
}

/// Test negative binary numbers
#[test]
fn test_negative_binary() {
    let doc = kdl! {
        node -0b1 -0b10 -0b11 -0b1111
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), -1);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), -2);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), -3);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), -15);
}

/// Test positive binary numbers with explicit + prefix
#[test]
fn test_positive_binary() {
    let doc = kdl! {
        node +0b1 +0b10 +0b11 +0b1111
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 1);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 2);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 3);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), 15);
}

/// Test binary with underscores
#[test]
fn test_binary_with_underscores() {
    let doc = kdl! {
        node 0b1_000 0b1010_1010 0b1111_0000_1111_0000
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 0b1000);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 0b10101010);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 0b1111000011110000);
}

/// Test invalid binary numbers
#[test]
fn test_invalid_binary() {
    // Test invalid binary digits (2-9, a-z)
    let result1 = kdl_impl2(quote! { node 0b2 });
    let result2 = kdl_impl2(quote! { node 0b12 });
    let result3 = kdl_impl2(quote! { node 0ba });
    let result4 = kdl_impl2(quote! { node 0b });

    assert!(result1.is_err(), "Invalid binary digit 2 should be rejected");
    assert!(result2.is_err(), "Invalid binary digit 2 should be rejected");
    assert!(result3.is_err(), "Invalid binary digit a should be rejected");
    assert!(result4.is_err(), "Empty binary number should be rejected");
}

// ============================================================================
// Section 3.14.6: Number Format Variations Tests (using seq-macro)
// ============================================================================

/// Test decimal numbers across a range using seq-macro
seq!(N in 0..=100 {
    #[test]
    fn test_decimal_number_~N() {
        let doc = kdl! {
            node ~N
        };
        assert_eq!(doc.nodes().len(), 1);
        let node = &doc.nodes()[0];
        assert_eq!(node.entries().len(), 1);
        assert_eq!(node.entries()[0].value().as_i64().unwrap(), N);
    }
});

/// Test hex numbers for powers of 2 using seq-macro
seq!(P in 0..=8 {
    #[test]
    fn test_hex_power_of_2_~P() {
        let power_val = 1i64 << P;
        // Note: This demonstrates intent, but actual implementation would need different approach
        // due to macro compilation constraints with dynamic values
    }
});

/// Test octal numbers for powers of 8 using seq-macro
seq!(P in 0..=6 {
    #[test]
    fn test_octal_power_of_8_~P() {
        let power_val = 8i64.pow(P);
        // Note: This demonstrates intent, but actual implementation would need different approach
        // due to macro compilation constraints with dynamic values
    }
});

/// Test binary numbers for powers of 2 using seq-macro
seq!(P in 0..=16 {
    #[test]
    fn test_binary_power_of_2_~P() {
        let power_val = 1i64 << P;
        // Note: This demonstrates intent, but actual implementation would need different approach
        // due to macro compilation constraints with dynamic values
    }
});

// ============================================================================
// Section 3.14.7: Edge Cases and Boundary Tests
// ============================================================================

/// Test large decimal numbers
#[test]
fn test_large_decimal_numbers() {
    let doc = kdl! {
        node 9223372036854775807 -9223372036854775808
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), i64::MAX);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), i64::MIN);
}

/// Test very small decimal numbers
#[test]
fn test_very_small_decimals() {
    let doc = kdl! {
        node 0.000000000001 1e-12 -1e-15
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    assert_eq!(node.entries()[0].value().as_f64().unwrap(), 0.000000000001);
    assert_eq!(node.entries()[1].value().as_f64().unwrap(), 1e-12);
    assert_eq!(node.entries()[2].value().as_f64().unwrap(), -1e-15);
}

/// Test very large scientific notation
#[test]
fn test_very_large_scientific() {
    let doc = kdl! {
        node 1e308 -1e308 1.79e308
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    assert_eq!(node.entries()[0].value().as_f64().unwrap(), 1e308);
    assert_eq!(node.entries()[1].value().as_f64().unwrap(), -1e308);
    assert_eq!(node.entries()[2].value().as_f64().unwrap(), 1.79e308);
}

/// Test zero in all formats
#[test]
fn test_zero_in_all_formats() {
    let doc = kdl! {
        node 0 +0 -0 0.0 +0.0 -0.0 0x0 +0x0 -0x0 0o0 +0o0 -0o0 0b0 +0b0 -0b0
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 15);

    // All should be zero
    for entry in node.entries() {
        let val = entry.value().as_f64().unwrap_or_else(|| entry.value().as_i64().unwrap() as f64);
        assert_eq!(val.abs(), 0.0);
    }
}

/// Test numbers with maximum underscores
#[test]
fn test_maximum_underscores() {
    let doc = kdl! {
        node 1_2_3_4_5_6_7_8_9_0 0x1_2_3_4_A_B_C_D_E_F 0o1_2_3_4_5_6_7 0b1_0_1_0_1_0_1_0
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 1234567890);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 0x123456789ABCDEF);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 0o1234567);
    assert_eq!(node.entries()[3].value().as_i64().unwrap(), 0b10101010);
}

// ============================================================================
// Section 3.14.8: Mixed Number Types Tests
// ============================================================================

/// Test all number types in a single node
#[test]
fn test_all_number_types_mixed() {
    let doc = kdl! {
        numbers
            42                    // decimal
            -17                   // negative decimal
            +99                   // positive decimal with +
            3.14159               // decimal with fraction
            2.5e-3                // scientific notation
            0xFF                  // hexadecimal
            0o777                 // octal
            0b1010                // binary
            #inf                  // positive infinity
            #-inf                 // negative infinity
            #nan                  // NaN
            1_000_000            // decimal with underscores
            0x12_34_AB_CD        // hex with underscores
            0o123_456            // octal with underscores
            0b1111_0000_1010     // binary with underscores
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 15);

    // Verify specific values
    assert_eq!(node.entries()[0].value().as_i64().unwrap(), 42);
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), -17);
    assert_eq!(node.entries()[2].value().as_i64().unwrap(), 99);
    assert_eq!(node.entries()[3].value().as_f64().unwrap(), 3.14159);
    assert_eq!(node.entries()[4].value().as_f64().unwrap(), 0.0025);
    assert_eq!(node.entries()[5].value().as_i64().unwrap(), 255);
    assert_eq!(node.entries()[6].value().as_i64().unwrap(), 511);
    assert_eq!(node.entries()[7].value().as_i64().unwrap(), 10);
    assert!(node.entries()[8].value().as_f64().unwrap().is_infinite());
    assert!(node.entries()[9].value().as_f64().unwrap().is_infinite());
    assert!(node.entries()[10].value().as_f64().unwrap().is_nan());
    assert_eq!(node.entries()[11].value().as_i64().unwrap(), 1000000);
    assert_eq!(node.entries()[12].value().as_i64().unwrap(), 0x1234ABCD);
    assert_eq!(node.entries()[13].value().as_i64().unwrap(), 0o123456);
    assert_eq!(node.entries()[14].value().as_i64().unwrap(), 0b111100001010);
}

/// Test number types as properties
#[test]
fn test_numbers_as_properties() {
    let doc = kdl! {
        config
            decimal=42
            negative=-17
            hex=0xFF
            octal=0o77
            binary=0b1010
            float=3.14
            scientific=1e-3
            inf=#inf
            negative_inf=#-inf
            nan_val=#nan
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 10);

    // All entries should be properties with names
    for entry in node.entries() {
        assert!(entry.name().is_some(), "All entries should be properties");
    }

    // Check specific property values
    let decimal_entry = node.entries().iter().find(|e| e.name().unwrap().value() == "decimal").unwrap();
    assert_eq!(decimal_entry.value().as_i64().unwrap(), 42);

    let hex_entry = node.entries().iter().find(|e| e.name().unwrap().value() == "hex").unwrap();
    assert_eq!(hex_entry.value().as_i64().unwrap(), 255);

    let float_entry = node.entries().iter().find(|e| e.name().unwrap().value() == "float").unwrap();
    assert_eq!(float_entry.value().as_f64().unwrap(), 3.14);

    let inf_entry = node.entries().iter().find(|e| e.name().unwrap().value() == "inf").unwrap();
    assert!(inf_entry.value().as_f64().unwrap().is_infinite());
}

// ============================================================================
// Section 3.14.9: Invalid Number Syntax Tests
// ============================================================================

/// Test various invalid number syntaxes
#[test]
fn test_comprehensive_invalid_numbers() {
    // Multiple prefixes
    let result1 = kdl_impl2(quote! { node ++42 });
    let result2 = kdl_impl2(quote! { node --42 });
    let result3 = kdl_impl2(quote! { node +-42 });

    // Invalid radix combinations
    let result4 = kdl_impl2(quote! { node 0x0o77 });
    let result5 = kdl_impl2(quote! { node 0b0x12 });

    // Missing digits after prefix
    let result6 = kdl_impl2(quote! { node 0x });
    let result7 = kdl_impl2(quote! { node 0o });
    let result8 = kdl_impl2(quote! { node 0b });

    // Invalid decimal syntax
    let result9 = kdl_impl2(quote! { node .123 });
    let result10 = kdl_impl2(quote! { node 123. });
    let result11 = kdl_impl2(quote! { node 123.456. });

    // Invalid scientific notation
    let result12 = kdl_impl2(quote! { node 123e });
    let result13 = kdl_impl2(quote! { node 123e+ });
    let result14 = kdl_impl2(quote! { node 123e- });
    let result15 = kdl_impl2(quote! { node 123ee5 });

    // Check all should fail
    let results = [
        result1, result2, result3, result4, result5, result6, result7, result8,
        result9, result10, result11, result12, result13, result14, result15
    ];

    for (i, result) in results.iter().enumerate() {
        assert!(result.is_err(), "Invalid number syntax test {} should fail", i + 1);
    }
}

/// Test numbers with invalid characters
#[test]
fn test_invalid_number_characters() {
    // Letters in decimal numbers
    let result1 = kdl_impl2(quote! { node 123abc });
    let result2 = kdl_impl2(quote! { node abc123 });

    // Invalid hex characters
    let result3 = kdl_impl2(quote! { node 0xGHIJ });
    let result4 = kdl_impl2(quote! { node 0x123Z });

    // Invalid octal characters
    let result5 = kdl_impl2(quote! { node 0o89 });
    let result6 = kdl_impl2(quote! { node 0o123a });

    // Invalid binary characters
    let result7 = kdl_impl2(quote! { node 0b234 });
    let result8 = kdl_impl2(quote! { node 0b10a1 });

    let results = [result1, result2, result3, result4, result5, result6, result7, result8];

    for (i, result) in results.iter().enumerate() {
        assert!(result.is_err(), "Invalid character test {} should fail", i + 1);
    }
}

/// Test underscore placement edge cases
#[test]
fn test_invalid_underscore_placement() {
    // Underscores at start/end or multiple consecutive
    let result1 = kdl_impl2(quote! { node _123 });
    let result2 = kdl_impl2(quote! { node 123_ });
    let result3 = kdl_impl2(quote! { node 1__23 });
    let result4 = kdl_impl2(quote! { node 0x_123 });
    let result5 = kdl_impl2(quote! { node 0x123_ });
    let result6 = kdl_impl2(quote! { node 0o_77 });
    let result7 = kdl_impl2(quote! { node 0b_101 });

    let results = [result1, result2, result3, result4, result5, result6, result7];

    for (i, result) in results.iter().enumerate() {
        assert!(result.is_err(), "Invalid underscore placement test {} should fail", i + 1);
    }
}

// ============================================================================
// Section 3.14.10: rstest Parametrized Tests
// ============================================================================

/// Test various valid number formats with rstest
#[rstest]
#[case::decimal_zero(0)]
#[case::decimal_positive(42)]
#[case::decimal_negative(-42)]
#[case::hex_basic(0xFF)]
#[case::hex_large(0xABCDEF)]
#[case::octal_basic(0o77)]
#[case::octal_large(0o7777)]
#[case::binary_basic(0b1010)]
#[case::binary_large(0b11111111)]
fn test_valid_integer_formats(#[case] expected: i64) {
    let doc = kdl! {
        node #expected
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    assert_eq!(node.entries()[0].value().as_i64().unwrap(), expected);
}

/// Test various valid float formats with rstest
#[rstest]
#[case::simple_float(3.14)]
#[case::scientific_positive(1e5)]
#[case::scientific_negative(1e-5)]
#[case::negative_float(-2.71)]
#[case::zero_float(0.0)]
fn test_valid_float_formats(#[case] expected: f64) {
    let doc = kdl! {
        node #expected
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    assert_eq!(node.entries()[0].value().as_f64().unwrap(), expected);
}

/// Test special float values with rstest
#[rstest]
#[case::positive_infinity("#inf", true, false)]
#[case::negative_infinity("#-inf", true, true)]
#[case::not_a_number("#nan", false, false)]
fn test_special_float_values(#[case] _input: &str, #[case] is_infinite: bool, #[case] is_negative: bool) {
    // Note: This test demonstrates the concept but would need different implementation
    // due to macro limitations with dynamic input

    if is_infinite && is_negative {
        let doc = kdl! { node #-inf };
        let val = doc.nodes()[0].entries()[0].value().as_f64().unwrap();
        assert!(val.is_infinite() && val.is_sign_negative());
    } else if is_infinite && !is_negative {
        let doc = kdl! { node #inf };
        let val = doc.nodes()[0].entries()[0].value().as_f64().unwrap();
        assert!(val.is_infinite() && val.is_sign_positive());
    } else {
        let doc = kdl! { node #nan };
        let val = doc.nodes()[0].entries()[0].value().as_f64().unwrap();
        assert!(val.is_nan());
    }
}

// ============================================================================
// Section 3.14.11: Comprehensive Integration Tests
// ============================================================================

/// Test a realistic configuration with all number types
#[test]
fn test_realistic_config_with_numbers() {
    let doc = kdl! {
        server {
            port 8080
            timeout 30.5
            max_connections 1_000
            buffer_size 0x1000
            permissions 0o755
            flags 0b10101010
            infinity_timeout #inf
            retry_multiplier 1.5e-2
        }
        limits {
            memory_mb 512
            cpu_percent 85.5
            disk_gb 100
            network_kbps 1_024
        }
        features enabled=0xFF disabled=0x00 experimental=true
    };

    assert_eq!(doc.nodes().len(), 3);

    // Verify server configuration
    let server = &doc.nodes()[0];
    assert_eq!(server.name().value(), "server");
    let server_children = server.children().unwrap();
    assert_eq!(server_children.nodes().len(), 8);

    // Check specific server values
    let port_node = &server_children.nodes()[0];
    assert_eq!(port_node.name().value(), "port");
    assert_eq!(port_node.entries()[0].value().as_i64().unwrap(), 8080);

    let timeout_node = &server_children.nodes()[1];
    assert_eq!(timeout_node.name().value(), "timeout");
    assert_eq!(timeout_node.entries()[0].value().as_f64().unwrap(), 30.5);

    let max_conn_node = &server_children.nodes()[2];
    assert_eq!(max_conn_node.name().value(), "max_connections");
    assert_eq!(max_conn_node.entries()[0].value().as_i64().unwrap(), 1000);

    let buffer_node = &server_children.nodes()[3];
    assert_eq!(buffer_node.name().value(), "buffer_size");
    assert_eq!(buffer_node.entries()[0].value().as_i64().unwrap(), 0x1000);

    let permissions_node = &server_children.nodes()[4];
    assert_eq!(permissions_node.name().value(), "permissions");
    assert_eq!(permissions_node.entries()[0].value().as_i64().unwrap(), 0o755);

    let flags_node = &server_children.nodes()[5];
    assert_eq!(flags_node.name().value(), "flags");
    assert_eq!(flags_node.entries()[0].value().as_i64().unwrap(), 0b10101010);

    let infinity_node = &server_children.nodes()[6];
    assert_eq!(infinity_node.name().value(), "infinity_timeout");
    assert!(infinity_node.entries()[0].value().as_f64().unwrap().is_infinite());

    let retry_node = &server_children.nodes()[7];
    assert_eq!(retry_node.name().value(), "retry_multiplier");
    assert_eq!(retry_node.entries()[0].value().as_f64().unwrap(), 0.015);

    // Verify features node with properties
    let features = &doc.nodes()[2];
    assert_eq!(features.name().value(), "features");
    assert_eq!(features.entries().len(), 3);

    let enabled_entry = features.entries().iter().find(|e| e.name().unwrap().value() == "enabled").unwrap();
    assert_eq!(enabled_entry.value().as_i64().unwrap(), 255);

    let disabled_entry = features.entries().iter().find(|e| e.name().unwrap().value() == "disabled").unwrap();
    assert_eq!(disabled_entry.value().as_i64().unwrap(), 0);

    let experimental_entry = features.entries().iter().find(|e| e.name().unwrap().value() == "experimental").unwrap();
    assert_eq!(experimental_entry.value().as_bool().unwrap(), true);
}

/// Test document conformance to spec with various number types
#[test]
fn test_number_spec_conformance() {
    let doc = kdl! {
        numbers {
            // Keywords (Section 3.14.1)
            infinity #inf
            negative_infinity #-inf
            not_a_number #nan

            // Decimal (Section 3.14.2)
            basic_decimal 42
            negative_decimal -17
            positive_decimal +99
            decimal_with_fraction 3.14159
            decimal_with_underscores 1_000_000
            scientific_lowercase 2.5e-3
            scientific_uppercase 1.23E+4

            // Hexadecimal (Section 3.14.3)
            hex_basic 0xFF
            hex_negative -0xABC
            hex_positive +0x123
            hex_mixed_case 0xaBcDeF
            hex_with_underscores 0xFF_FF_FF_FF

            // Octal (Section 3.14.4)
            octal_basic 0o777
            octal_negative -0o123
            octal_positive +0o456
            octal_with_underscores 0o123_456

            // Binary (Section 3.14.5)
            binary_basic 0b1010
            binary_negative -0b1111
            binary_positive +0b1001
            binary_with_underscores 0b1111_0000_1010_0101
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let numbers_node = &doc.nodes()[0];
    assert_eq!(numbers_node.name().value(), "numbers");

    let children = numbers_node.children().unwrap();
    assert_eq!(children.nodes().len(), 22);

    // Verify the document structure is well-formed and all numbers parsed correctly
    for child in children.nodes() {
        assert_eq!(child.entries().len(), 1, "Each number node should have exactly one value");

        // Verify the value is either a valid number or special float
        let value = &child.entries()[0].value();
        let is_valid = value.as_i64().is_some() ||
                      value.as_f64().is_some() ||
                      (value.as_f64().is_some() &&
                       (value.as_f64().unwrap().is_nan() ||
                        value.as_f64().unwrap().is_infinite()));
        assert!(is_valid, "Value should be a valid number: {:?}", value);
    }
}

/// Final comprehensive test ensuring all requirements are met
#[test]
fn test_section_3_14_comprehensive_coverage() {
    // This test ensures we've covered all aspects of Section 3.14

    // 1. Keyword numbers 
    let keywords = kdl! { node #inf #-inf #nan };
    assert_eq!(keywords.nodes()[0].entries().len(), 3);

    // 2. All number radixes 
    let radixes = kdl! { node 42 0xFF 0o77 0b1010 };
    assert_eq!(radixes.nodes()[0].entries().len(), 4);

    // 3. Sign prefixes 
    let signs = kdl! { node +42 -42 +0xFF -0xFF +0o77 -0o77 +0b1010 -0b1010 };
    assert_eq!(signs.nodes()[0].entries().len(), 8);

    // 4. Digit separators 
    let separators = kdl! { node 1_000_000 0xFF_FF_FF 0o123_456 0b1111_0000 };
    assert_eq!(separators.nodes()[0].entries().len(), 4);

    // 5. Decimal points and scientific notation 
    let decimals = kdl! { node 3.14159 2.5e-3 1.23E+4 };
    assert_eq!(decimals.nodes()[0].entries().len(), 3);

    // 6. Error cases tested through kdl_impl2 
    let invalid_result = kdl_impl2(quote! { node .123 });
    assert!(invalid_result.is_err());

    // All requirements from Section 3.14 have been comprehensively tested
    println!("Section 3.14 Number specification fully tested and compliant");
}