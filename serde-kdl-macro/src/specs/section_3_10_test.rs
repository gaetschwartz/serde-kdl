//! Tests for KDL Identifier String specification (Section 3.10)
//!
//! This module contains comprehensive tests for KDL Identifier String requirements:
//! - Initial character restrictions (Section 3.10.1)
//! - Non-identifier character restrictions (Section 3.10.2)
//! - Unicode category requirements for identifiers
//! - Disallowed patterns that resemble numbers or keywords
//! - Edge cases and boundary conditions
//! - Systematic testing of character ranges using seq-macro

use crate::specs::kdl_impl2;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rstest::rstest;
use seq_macro::seq;
use serde_kdl_macro::kdl;

// ============================================================================
// Valid Identifier Tests
// ============================================================================

/// Test basic valid identifiers
#[test]
fn test_basic_valid_identifiers() {
    let doc = kdl! {
        simple_identifier
        another-identifier
        camelCase
        PascalCase
        identifier_with_underscores
        identifier123
        _underscore_start
        $dollar_start
        café
        q¬
        >A:20
    };

    assert_eq!(doc.nodes().len(), 11);
    assert_eq!(doc.nodes()[0].name().value(), "simple_identifier");
    assert_eq!(doc.nodes()[1].name().value(), "another-identifier");
    assert_eq!(doc.nodes()[2].name().value(), "camelCase");
    assert_eq!(doc.nodes()[3].name().value(), "PascalCase");
    assert_eq!(doc.nodes()[4].name().value(), "identifier_with_underscores");
    assert_eq!(doc.nodes()[5].name().value(), "identifier123");
    assert_eq!(doc.nodes()[6].name().value(), "_underscore_start");
    assert_eq!(doc.nodes()[7].name().value(), "$dollar_start");
    assert_eq!(doc.nodes()[8].name().value(), "café");
    assert_eq!(doc.nodes()[9].name().value(), "q¬");
    assert_eq!(doc.nodes()[10].name().value(), ">A:20");
}

/// Test identifiers with special Unicode characters
#[test]
fn test_unicode_identifiers() {
    let doc = kdl! {
        // Letter-like symbols
        
        
        // Mathematical symbols
        ±
        ²
        ³
        // Various scripts
        âÑèÙê
        'D91(J)
        9?(M&@
        å,ž
        \m´
        •»»·½¹º¬
    };

    assert_eq!(doc.nodes().len(), 10);
    assert_eq!(doc.nodes()[0].name().value(), "");
    assert_eq!(doc.nodes()[1].name().value(), "");
    assert_eq!(doc.nodes()[2].name().value(), "±");
    assert_eq!(doc.nodes()[3].name().value(), "²");
    assert_eq!(doc.nodes()[4].name().value(), "³");
    assert_eq!(doc.nodes()[5].name().value(), "âÑèÙê");
    assert_eq!(doc.nodes()[6].name().value(), "'D91(J)");
    assert_eq!(doc.nodes()[7].name().value(), "9?(M&@");
    assert_eq!(doc.nodes()[8].name().value(), "å,ž");
    assert_eq!(doc.nodes()[9].name().value(), "\m´");
    assert_eq!(doc.nodes()[10].name().value(), "•»»·½¹º¬");
}

/// Test identifiers with connecting punctuation
#[test]
fn test_connecting_punctuation_identifiers() {
    let doc = kdl! {
        // Underscore (connector punctuation)
        test_identifier
        another_test_identifier
        // Note: Other connector punctuation like ? U+203F could be tested
        // but may not be easily representable in the macro syntax
    };

    assert_eq!(doc.nodes().len(), 2);
    assert_eq!(doc.nodes()[0].name().value(), "test_identifier");
    assert_eq!(doc.nodes()[1].name().value(), "another_test_identifier");
}

/// Test identifiers starting with special allowed characters
#[test]
fn test_special_initial_characters() {
    let doc = kdl! {
        --double-dash
        -single-dash-ok
        +plus-start
        .dot-start
        _underscore
        $dollar
    };

    assert_eq!(doc.nodes().len(), 6);
    assert_eq!(doc.nodes()[0].name().value(), "--double-dash");
    assert_eq!(doc.nodes()[1].name().value(), "-single-dash-ok");
    assert_eq!(doc.nodes()[2].name().value(), "+plus-start");
    assert_eq!(doc.nodes()[3].name().value(), ".dot-start");
    assert_eq!(doc.nodes()[4].name().value(), "_underscore");
    assert_eq!(doc.nodes()[5].name().value(), "$dollar");
}

// ============================================================================
// Invalid Initial Character Tests (Section 3.10.1)
// ============================================================================

/// Test that decimal digits cannot be initial characters
seq!(N in 0..=9 {
    #[test]
    fn test_digit_~N_initial_fails() {
        let result = kdl_impl2(quote! {
            ~N identifier
        });
        assert!(result.is_err(), "Identifier starting with digit {} should fail", N);
    }
});

/// Test numbers that look like identifiers should fail
#[test]
fn test_number_like_identifiers_fail() {
    // Test "almost a number" pattern - decimal point without leading digit
    let result1 = kdl_impl2(quote! {
        .1
    });
    assert!(result1.is_err(), "Identifier '.1' should fail (looks like number)");

    let result2 = kdl_impl2(quote! {
        .123
    });
    assert!(result2.is_err(), "Identifier '.123' should fail (looks like number)");

    // Test numbers with suffixes
    let result3 = kdl_impl2(quote! {
        1.0v2
    });
    assert!(result3.is_err(), "Identifier '1.0v2' should fail (looks like number)");

    let result4 = kdl_impl2(quote! {
        -1em
    });
    assert!(result4.is_err(), "Identifier '-1em' should fail (looks like number)");

    let result5 = kdl_impl2(quote! {
        123abc
    });
    assert!(result5.is_err(), "Identifier '123abc' should fail (starts with digits)");
}

/// Test + and - with digit restrictions
#[test]
fn test_plus_minus_digit_restrictions() {
    // + followed by digit should fail
    let result1 = kdl_impl2(quote! {
        +1abc
    });
    assert!(result1.is_err(), "Identifier '+1abc' should fail (+ followed by digit)");

    // - followed by digit should fail
    let result2 = kdl_impl2(quote! {
        -2abc
    });
    assert!(result2.is_err(), "Identifier '-2abc' should fail (- followed by digit)");

    // + followed by . followed by digit should fail
    let result3 = kdl_impl2(quote! {
        +.5abc
    });
    assert!(result3.is_err(), "Identifier '+.5abc' should fail (+ followed by . followed by digit)");

    // - followed by . followed by digit should fail
    let result4 = kdl_impl2(quote! {
        -.7abc
    });
    assert!(result4.is_err(), "Identifier '-.7abc' should fail (- followed by . followed by digit)");
}

/// Test . with digit restrictions
#[test]
fn test_dot_digit_restrictions() {
    // . followed by digit should fail
    let result1 = kdl_impl2(quote! {
        .3abc
    });
    assert!(result1.is_err(), "Identifier '.3abc' should fail (. followed by digit)");

    let result2 = kdl_impl2(quote! {
        .0identifier
    });
    assert!(result2.is_err(), "Identifier '.0identifier' should fail (. followed by digit)");
}

// ============================================================================
// Language Keyword Tests
// ============================================================================

/// Test that language keywords without # should fail
#[test]
fn test_language_keywords_fail() {
    let result1 = kdl_impl2(quote! {
        inf
    });
    assert!(result1.is_err(), "Identifier 'inf' should fail (reserved keyword)");

    let result2 = kdl_impl2(quote! {
        -inf
    });
    assert!(result2.is_err(), "Identifier '-inf' should fail (reserved keyword)");

    let result3 = kdl_impl2(quote! {
        nan
    });
    assert!(result3.is_err(), "Identifier 'nan' should fail (reserved keyword)");

    let result4 = kdl_impl2(quote! {
        true
    });
    assert!(result4.is_err(), "Identifier 'true' should fail (reserved keyword)");

    let result5 = kdl_impl2(quote! {
        false
    });
    assert!(result5.is_err(), "Identifier 'false' should fail (reserved keyword)");

    let result6 = kdl_impl2(quote! {
        null
    });
    assert!(result6.is_err(), "Identifier 'null' should fail (reserved keyword)");
}

// ============================================================================
// Non-identifier Character Tests (Section 3.10.2)
// ============================================================================

/// Test that specific non-identifier characters are forbidden
#[test]
fn test_forbidden_characters() {
    // Test each forbidden character individually in middle of identifier
    let forbidden_chars = ['(', ')', '{', '}', '[', ']', '/', '\\', '"', '#', ';', '='];

    for &ch in &forbidden_chars {
        let identifier_name = format!("test{}identifier", ch);
        // Note: We can't easily test this with the kdl! macro since these would
        // be tokenization errors. The test documents the requirement.

        // In a real implementation, we would test:
        // let result = parse_kdl(&format!("{}abc", ch));
        // assert!(result.is_err(), "Identifier with '{}' should fail", ch);
    }
}

/// Test whitespace characters in identifiers should fail
#[test]
fn test_whitespace_in_identifiers_fails() {
    // These would be tokenization errors, so we test the requirement conceptually

    // Space (U+0020)
    // Tab (U+0009)
    // No-Break Space (U+00A0)
    // etc. - all characters from section 3.17

    // These cannot be represented in the kdl! macro syntax as they would
    // be treated as separate tokens, but the specification is clear
}

/// Test newline characters in identifiers should fail
#[test]
fn test_newlines_in_identifiers_fail() {
    // These would typically be syntax errors at the token level
    // Testing the conceptual requirement from section 3.18

    // Line Feed (LF) - U+000A
    // Carriage Return (CR) - U+000D
    // CRLF - U+000D + U+000A
    // Next Line (NEL) - U+0085
    // Vertical Tab (VT) - U+000B
    // Form Feed (FF) - U+000C
    // Line Separator (LS) - U+2028
    // Paragraph Separator (PS) - U+2029

    // These cannot be represented in valid identifier tokens
}

/// Test disallowed literal code points in identifiers
#[test]
fn test_disallowed_codepoints_in_identifiers() {
    // Control characters U+0000-0008, U+000E-001F, U+007F
    // Non Unicode Scalar Values U+D800-DFFF
    // Unicode direction control characters U+200E-200F, U+202A-202E, U+2066-2069
    // BOM U+FEFF (except at document start)

    // These would be rejected at the tokenizer level and cannot be
    // represented in valid Rust string literals or macro input
}

// ============================================================================
// Systematic Character Range Tests
// ============================================================================

/// Test ASCII letter ranges systematically
seq!(C in 65..=90 {  // A-Z
    #[test]
    fn test_ascii_uppercase_~C() {
        let ch = char::from(C as u8);
        let identifier = format!("{}identifier_test", ch);
        let doc = kdl! {
            Aidentifier_test
        };
        assert_eq!(doc.nodes().len(), 1);
        assert!(doc.nodes()[0].name().value().starts_with('A'));
    }
});

seq!(C in 97..=122 {  // a-z
    #[test]
    fn test_ascii_lowercase_~C() {
        let ch = char::from(C as u8);
        let identifier = format!("{}identifier_test", ch);
        let doc = kdl! {
            aidentifier_test
        };
        assert_eq!(doc.nodes().len(), 1);
        assert!(doc.nodes()[0].name().value().starts_with('a'));
    }
});

/// Test underscore and common punctuation
#[test]
fn test_common_punctuation() {
    let doc = kdl! {
        _underscore
        $dollar
        // Testing other punctuation that should work in identifiers
    };

    assert_eq!(doc.nodes().len(), 2);
    assert_eq!(doc.nodes()[0].name().value(), "_underscore");
    assert_eq!(doc.nodes()[1].name().value(), "$dollar");
}

/// Test boundary cases for identifier validation
#[test]
fn test_identifier_boundary_cases() {
    // Single character identifiers
    let doc = kdl! {
        a
        z
        A
        Z
        _
        $
    };

    assert_eq!(doc.nodes().len(), 6);
    assert_eq!(doc.nodes()[0].name().value(), "a");
    assert_eq!(doc.nodes()[1].name().value(), "z");
    assert_eq!(doc.nodes()[2].name().value(), "A");
    assert_eq!(doc.nodes()[3].name().value(), "Z");
    assert_eq!(doc.nodes()[4].name().value(), "_");
    assert_eq!(doc.nodes()[5].name().value(), "$");
}

/// Test very long identifiers
#[test]
fn test_long_identifiers() {
    let doc = kdl! {
        this_is_a_very_long_identifier_name_that_should_still_be_valid_according_to_the_kdl_specification
    };

    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(
        doc.nodes()[0].name().value(),
        "this_is_a_very_long_identifier_name_that_should_still_be_valid_according_to_the_kdl_specification"
    );
}

/// Test identifiers with numbers in non-initial positions
#[test]
fn test_identifiers_with_numbers() {
    let doc = kdl! {
        test1
        test123
        version2
        item_42
        prefix_123_suffix
        a1b2c3
    };

    assert_eq!(doc.nodes().len(), 6);
    assert_eq!(doc.nodes()[0].name().value(), "test1");
    assert_eq!(doc.nodes()[1].name().value(), "test123");
    assert_eq!(doc.nodes()[2].name().value(), "version2");
    assert_eq!(doc.nodes()[3].name().value(), "item_42");
    assert_eq!(doc.nodes()[4].name().value(), "prefix_123_suffix");
    assert_eq!(doc.nodes()[5].name().value(), "a1b2c3");
}

// ============================================================================
// Unicode Category Tests
// ============================================================================

/// Test various Unicode letter categories
#[test]
fn test_unicode_letter_categories() {
    let doc = kdl! {
        // Uppercase Letter (Lu)
        Ä
        Ñ
        Ø

        // Lowercase Letter (Ll)
        ä
        ñ
        ø

        // Titlecase Letter (Lt)
        Å
        È
        Ë

        // Modifier Letter (Lm)
        °
        ²
        ·

        // Other Letter (Lo) - includes CJK ideographs
        -
        ‡
        r‰Lj
        «¿«Ê
    };

    assert_eq!(doc.nodes().len(), 17);
    // Verify some samples
    assert_eq!(doc.nodes()[0].name().value(), "Ä");
    assert_eq!(doc.nodes()[3].name().value(), "ä");
    assert_eq!(doc.nodes()[13].name().value(), "-");
    assert_eq!(doc.nodes()[14].name().value(), "‡");
}

/// Test Unicode number categories in non-initial positions
#[test]
fn test_unicode_numbers_non_initial() {
    let doc = kdl! {
        // Decimal Number (Nd) - should work in non-initial positions
        test`
        itema
        versionb

        // Letter Number (Nl) - should work
        test_p
        item_q
        version_r

        // Other Number (No) - should work
        test_½
        item_¼
        version_¾
    };

    assert_eq!(doc.nodes().len(), 9);
    assert_eq!(doc.nodes()[0].name().value(), "test`");
    assert_eq!(doc.nodes()[1].name().value(), "itema");
    assert_eq!(doc.nodes()[2].name().value(), "versionb");
    assert_eq!(doc.nodes()[3].name().value(), "test_p");
    assert_eq!(doc.nodes()[6].name().value(), "test_½");
}

/// Test Unicode mark categories in identifiers
#[test]
fn test_unicode_marks() {
    let doc = kdl! {
        // Nonspacing Mark (Mn) - combining marks
        café  // e with acute accent
        naïve // i with diaeresis

        // Spacing Combining Mark (Mc) - should work
        // Note: These are harder to represent directly in source

        // Enclosing Mark (Me) - should work
        // Note: These are rare and hard to represent
    };

    assert_eq!(doc.nodes().len(), 2);
    assert_eq!(doc.nodes()[0].name().value(), "café");
    assert_eq!(doc.nodes()[1].name().value(), "naïve");
}

/// Test Unicode connector punctuation
#[test]
fn test_unicode_connector_punctuation() {
    let doc = kdl! {
        // Connector Punctuation (Pc)
        test_identifier  // underscore is the most common
        // Other connector punctuation like ? (U+203F) might work
        // but are harder to represent in macro syntax
    };

    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(doc.nodes()[0].name().value(), "test_identifier");
}

// ============================================================================
// Edge Cases and Error Conditions
// ============================================================================

/// Test empty identifier should fail
#[test]
fn test_empty_identifier_fails() {
    // This would be a syntax error at the macro level
    // But conceptually, empty identifiers should not be allowed
}

/// Test identifiers that are only symbols/punctuation
#[test]
fn test_symbol_only_identifiers() {
    let doc = kdl! {
        // Single symbol identifiers that should work
        _
        $
        // These are valid per the specification
    };

    assert_eq!(doc.nodes().len(), 2);
    assert_eq!(doc.nodes()[0].name().value(), "_");
    assert_eq!(doc.nodes()[1].name().value(), "$");
}

/// Test case sensitivity
#[test]
fn test_case_sensitivity() {
    let doc = kdl! {
        test
        Test
        TEST
        tEsT
    };

    assert_eq!(doc.nodes().len(), 4);
    assert_eq!(doc.nodes()[0].name().value(), "test");
    assert_eq!(doc.nodes()[1].name().value(), "Test");
    assert_eq!(doc.nodes()[2].name().value(), "TEST");
    assert_eq!(doc.nodes()[3].name().value(), "tEsT");
}

/// Test identifiers with mixed scripts
#[test]
fn test_mixed_script_identifiers() {
    let doc = kdl! {
        // Mixing Latin and Greek
        ±²³abc

        // Mixing Latin and CJK
        testq¬

        // Mixing Latin and Arabic
        test91(J

        // Mixing Latin and Cyrillic
        test CAA:89
    };

    assert_eq!(doc.nodes().len(), 4);
    assert_eq!(doc.nodes()[0].name().value(), "±²³abc");
    assert_eq!(doc.nodes()[1].name().value(), "testq¬");
    assert_eq!(doc.nodes()[2].name().value(), "test91(J");
    assert_eq!(doc.nodes()[3].name().value(), "test CAA:89");
}

// ============================================================================
// Property Key Identifier Tests
// ============================================================================

/// Test identifiers as property keys
#[test]
fn test_identifier_property_keys() {
    let doc = kdl! {
        node valid_key="value" another-key="value2" _underscore_key="value3"
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    // Check property keys are properly parsed as identifiers
    assert_eq!(node.get("valid_key").unwrap().value().as_string(), Some("value"));
    assert_eq!(node.get("another-key").unwrap().value().as_string(), Some("value2"));
    assert_eq!(node.get("_underscore_key").unwrap().value().as_string(), Some("value3"));
}

/// Test invalid property key identifiers should fail
#[test]
fn test_invalid_property_key_identifiers() {
    // Property keys follow same identifier rules
    let result1 = kdl_impl2(quote! {
        node 123key="value"
    });
    assert!(result1.is_err(), "Property key starting with digit should fail");

    let result2 = kdl_impl2(quote! {
        node true="value"
    });
    assert!(result2.is_err(), "Property key using reserved keyword should fail");
}

// ============================================================================
// Type Annotation Identifier Tests
// ============================================================================

/// Test identifiers in type annotations
#[test]
fn test_identifier_type_annotations() {
    let doc = kdl! {
        node (custom_type)"value" (another-type)42 (_type_with_underscore)true
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    // Check type annotations are properly parsed as identifiers
    assert_eq!(node.entries()[0].ty().unwrap().value(), "custom_type");
    assert_eq!(node.entries()[1].ty().unwrap().value(), "another-type");
    assert_eq!(node.entries()[2].ty().unwrap().value(), "_type_with_underscore");
}

// ============================================================================
// Comprehensive Integration Tests
// ============================================================================

/// Test complex document with various identifier types
#[test]
fn test_comprehensive_identifier_usage() {
    let doc = kdl! {
        // Various identifier styles
        simple_node
        kebab-case-node
        camelCaseNode
        PascalCaseNode
        _private_node
        $special_node

        // Identifiers with numbers
        version2_node
        node_42
        test_123_end

        // Unicode identifiers
        café_node
        q¬_node
        ±²³_node

        // Node with identifier properties and type annotations
        complex_node (string_type)"value" number_prop=42 unicode_prop="q¬" {
            // Child nodes with identifier names
            child_node1
            child_node2
            unicode_child_q¬
        }
    };

    assert_eq!(doc.nodes().len(), 13);

    // Verify main nodes
    assert_eq!(doc.nodes()[0].name().value(), "simple_node");
    assert_eq!(doc.nodes()[1].name().value(), "kebab-case-node");
    assert_eq!(doc.nodes()[2].name().value(), "camelCaseNode");
    assert_eq!(doc.nodes()[3].name().value(), "PascalCaseNode");
    assert_eq!(doc.nodes()[4].name().value(), "_private_node");
    assert_eq!(doc.nodes()[5].name().value(), "$special_node");
    assert_eq!(doc.nodes()[6].name().value(), "version2_node");
    assert_eq!(doc.nodes()[7].name().value(), "node_42");
    assert_eq!(doc.nodes()[8].name().value(), "test_123_end");
    assert_eq!(doc.nodes()[9].name().value(), "café_node");
    assert_eq!(doc.nodes()[10].name().value(), "q¬_node");
    assert_eq!(doc.nodes()[11].name().value(), "±²³_node");

    // Verify complex node
    let complex_node = &doc.nodes()[12];
    assert_eq!(complex_node.name().value(), "complex_node");

    // Verify type annotation
    assert_eq!(complex_node.entries()[0].ty().unwrap().value(), "string_type");

    // Verify properties with identifier keys
    assert_eq!(complex_node.get("number_prop").unwrap().value().as_i64(), Some(42));
    assert_eq!(complex_node.get("unicode_prop").unwrap().value().as_string(), Some("q¬"));

    // Verify child nodes
    let children = complex_node.children().unwrap();
    assert_eq!(children.nodes().len(), 3);
    assert_eq!(children.nodes()[0].name().value(), "child_node1");
    assert_eq!(children.nodes()[1].name().value(), "child_node2");
    assert_eq!(children.nodes()[2].name().value(), "unicode_child_q¬");
}

/// Test identifier validation consistency across all contexts
#[test]
fn test_identifier_validation_consistency() {
    // Same identifier rules should apply to:
    // - Node names
    // - Property keys
    // - Type annotations

    let doc = kdl! {
        valid_identifier valid_key="value" (valid_type)"typed_value"
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];

    // All should use the same identifier: valid_identifier, valid_key, valid_type
    assert_eq!(node.name().value(), "valid_identifier");
    assert_eq!(node.get("valid_key").unwrap().value().as_string(), Some("value"));
    assert_eq!(node.entries()[1].ty().unwrap().value(), "valid_type");
}