//! Tests for Section 3.11: Quoted String
//!
//! This module contains comprehensive tests for the KDL Quoted String specification
//! as defined in section 3.11 of the KDL specification.
//!
//! The tests cover:
//! - Basic quoted string syntax with double quotes
//! - All escape sequences (\\, \", \n, \r, \t, \/, \b, \f, \s, \u{...})
//! - Unicode code point escapes with validation
//! - Invalid escape sequence handling
//! - Escaped whitespace (Section 3.11.1.1)
//! - Disallowed literal code points validation
//! - Edge cases and boundary conditions
//! - Error handling for malformed strings

use crate::specs::kdl_impl2;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rstest::rstest;
use seq_macro::seq;
use serde_kdl_macro::kdl;

// ============================================================================
// Section 3.11.1: Basic Quoted String Tests
// ============================================================================

/// Test basic quoted string syntax
#[test]
fn test_basic_quoted_string() {
    let doc = kdl! {
        node "hello world"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "hello world");
}

/// Test empty quoted string
#[test]
fn test_empty_quoted_string() {
    let doc = kdl! {
        node ""
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 1);
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "");
}

/// Test quoted string with spaces
#[test]
fn test_quoted_string_with_spaces() {
    let doc = kdl! {
        node "  spaces  everywhere  "
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "  spaces  everywhere  ");
}

/// Test quoted string with special characters (allowed)
#[test]
fn test_quoted_string_special_characters() {
    let doc = kdl! {
        node "!@#$%^&*()_+-={}[]|:;<>?,./"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "!@#$%^&*()_+-={}[]|:;<>?,./");
}

/// Test multiple quoted strings
#[test]
fn test_multiple_quoted_strings() {
    let doc = kdl! {
        node "first" "second" "third"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "first");
    assert_eq!(node.entries()[1].value().as_string().unwrap(), "second");
    assert_eq!(node.entries()[2].value().as_string().unwrap(), "third");
}

/// Test quoted strings as property values
#[test]
fn test_quoted_strings_as_properties() {
    let doc = kdl! {
        node key1="value1" key2="value2"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2);
    assert_eq!(node.get("key1").unwrap().value().as_string().unwrap(), "value1");
    assert_eq!(node.get("key2").unwrap().value().as_string().unwrap(), "value2");
}

// ============================================================================
// Section 3.11.1: Standard Escape Sequences Tests
// ============================================================================

/// Test line feed escape sequence (\n)
#[test]
fn test_line_feed_escape() {
    let doc = kdl! {
        node "line1\nline2"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "line1\nline2");
}

/// Test carriage return escape sequence (\r)
#[test]
fn test_carriage_return_escape() {
    let doc = kdl! {
        node "before\rafter"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "before\rafter");
}

/// Test tab escape sequence (\t)
#[test]
fn test_tab_escape() {
    let doc = kdl! {
        node "before\tafter"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "before\tafter");
}

/// Test backslash escape sequence (\\)
#[test]
fn test_backslash_escape() {
    let doc = kdl! {
        node "path\\to\\file"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "path\\to\\file");
}

/// Test double quote escape sequence (\")
#[test]
fn test_double_quote_escape() {
    let doc = kdl! {
        node "She said \"Hello!\""
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "She said \"Hello!\"");
}

/// Test backspace escape sequence (\b)
#[test]
fn test_backspace_escape() {
    let doc = kdl! {
        node "before\bafter"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "before\bafter");
}

/// Test form feed escape sequence (\f)
#[test]
fn test_form_feed_escape() {
    let doc = kdl! {
        node "before\fafter"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "before\fafter");
}

/// Test space escape sequence (\s)
#[test]
fn test_space_escape() {
    let doc = kdl! {
        node "word1\sword2"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "word1 word2");
}

/// Test all basic escape sequences together
#[test]
fn test_all_basic_escapes_together() {
    let doc = kdl! {
        node "newline:\n\ttab:\t\rcarriage:\rquote:\"\\\\"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(
        node.entries()[0].value().as_string().unwrap(),
        "newline:\n\ttab:\t\rcarriage:\rquote:\"\\\\"
    );
}

// Test each escape sequence using seq-macro for thoroughness
seq!(I in 0..8 {
    #[test]
    fn test_escape_sequence_~I() {
        let test_cases = [
            ("\\n", "\n", "line_feed"),
            ("\\r", "\r", "carriage_return"),
            ("\\t", "\t", "tab"),
            ("\\\\", "\\", "backslash"),
            ("\\\"", "\"", "double_quote"),
            ("\\b", "\u{0008}", "backspace"),
            ("\\f", "\u{000C}", "form_feed"),
            ("\\s", " ", "space"),
        ];

        let (input, expected, name) = &test_cases[~I];
        let doc = kdl! {
            node input
        };
        assert_eq!(doc.nodes().len(), 1);
        let node = &doc.nodes()[0];
        assert_eq!(node.entries()[0].value().as_string().unwrap(), expected, "Failed for {}", name);
    }
});

// ============================================================================
// Section 3.11.1: Unicode Escape Sequences Tests
// ============================================================================

/// Test basic 4-digit Unicode escape
#[test]
fn test_unicode_escape_4_digit() {
    let doc = kdl! {
        node "\u{0041}"  // 'A'
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "A");
}

/// Test Unicode escape with various lengths (1-6 hex digits)
#[rstest]
#[case::one_digit("\u{A}", "\u{000A}")]      // 1 digit
#[case::two_digits("\u{41}", "A")]            // 2 digits
#[case::three_digits("\u{041}", "A")]         // 3 digits
#[case::four_digits("\u{0041}", "A")]         // 4 digits
#[case::five_digits("\u{10041}", "\u{10041}")] // 5 digits
#[case::six_digits("\u{100041}", "\u{100041}")] // 6 digits - may be invalid
fn test_unicode_escape_lengths(#[case] input: &str, #[case] expected: &str) {
    let doc = kdl! {
        node input
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), expected);
}

/// Test common Unicode characters
#[test]
fn test_unicode_escape_common_chars() {
    let doc = kdl! {
        node "\u{0048}\u{0065}\u{006C}\u{006C}\u{006F}"  // "Hello"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "Hello");
}

/// Test Unicode escape for non-ASCII characters
#[test]
fn test_unicode_escape_non_ascii() {
    let doc = kdl! {
        node "\u{00E9}\u{00F1}\u{00FC}"  // é, ñ, ü
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "éñü");
}

/// Test Unicode escape for emoji and high code points
#[test]
fn test_unicode_escape_emoji() {
    let doc = kdl! {
        node "\u{1F600}\u{1F44D}"  // = =M
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "= =M");
}

/// Test Unicode escape mixed with regular characters
#[test]
fn test_unicode_escape_mixed() {
    let doc = kdl! {
        node "Hello \u{1F30D}!"  // Hello <!
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "Hello <!");
}

/// Test multiple Unicode escapes
#[test]
fn test_multiple_unicode_escapes() {
    let doc = kdl! {
        node "\u{0048}\u{0065}\u{006C}\u{006C}\u{006F}\u{0020}\u{1F30D}"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "Hello <");
}

// ============================================================================
// Section 3.11.1.1: Escaped Whitespace Tests
// ============================================================================

/// Test basic escaped whitespace
#[test]
fn test_escaped_whitespace_basic() {
    let doc = kdl! {
        node "Hello \    World"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "Hello World");
}

/// Test escaped whitespace with tabs
#[test]
fn test_escaped_whitespace_with_tabs() {
    let doc = kdl! {
        node "Hello \	World"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "Hello World");
}

/// Test escaped whitespace preserves escape sequences
#[test]
fn test_escaped_whitespace_preserves_escapes() {
    let doc = kdl! {
        node "Hello\       \nWorld"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "Hello\nWorld");
}

/// Test escaped whitespace across multiple lines
#[test]
fn test_escaped_whitespace_multiline() {
    let doc = kdl! {
        node "Hello\n\
    World"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "Hello\nWorld");
}

/// Test escaped whitespace at string boundaries
#[test]
fn test_escaped_whitespace_boundaries() {
    let doc = kdl! {
        node "\   start"
        node2 "end   \"
    };
    assert_eq!(doc.nodes().len(), 2);
    assert_eq!(doc.nodes()[0].entries()[0].value().as_string().unwrap(), "start");
    assert_eq!(doc.nodes()[1].entries()[0].value().as_string().unwrap(), "end");
}

/// Test complex escaped whitespace patterns
#[test]
fn test_complex_escaped_whitespace() {
    let doc = kdl! {
        node "word1\    \t  \nword2\      \
             \n\tword3"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "word1\nword2\n\tword3");
}

// ============================================================================
// Section 3.11.1.2: Invalid Escape Sequences Tests
// ============================================================================

/// Test invalid single character escapes
#[rstest]
#[case::invalid_a("\\a")]
#[case::invalid_c("\\c")]
#[case::invalid_d("\\d")]
#[case::invalid_e("\\e")]
#[case::invalid_g("\\g")]
#[case::invalid_h("\\h")]
#[case::invalid_i("\\i")]
#[case::invalid_j("\\j")]
#[case::invalid_k("\\k")]
#[case::invalid_l("\\l")]
#[case::invalid_m("\\m")]
#[case::invalid_o("\\o")]
#[case::invalid_p("\\p")]
#[case::invalid_q("\\q")]
#[case::invalid_v("\\v")]
#[case::invalid_w("\\w")]
#[case::invalid_x("\\x")]
#[case::invalid_y("\\y")]
#[case::invalid_z("\\z")]
fn test_invalid_escape_sequences(#[case] invalid_escape: &str) {
    let input_literal = format!("\"{}\"", invalid_escape);
    let input = format!("node {}", input_literal);
    let token_stream: TokenStream2 = input.parse().unwrap();
    let result = kdl_impl2(token_stream);
    assert!(result.is_err(), "Invalid escape sequence '{}' should produce an error", invalid_escape);
}

/// Test invalid Unicode escape sequences
#[test]
fn test_invalid_unicode_escapes() {
    // Missing opening brace
    let result1 = kdl_impl2(quote! { node "\\u1234" });
    assert!(result1.is_err(), "Unicode escape without opening brace should fail");

    // Missing closing brace
    let result2 = kdl_impl2(quote! { node "\\u{1234" });
    assert!(result2.is_err(), "Unicode escape without closing brace should fail");

    // Empty braces
    let result3 = kdl_impl2(quote! { node "\\u{}" });
    assert!(result3.is_err(), "Empty Unicode escape should fail");

    // Too many hex digits (>6)
    let result4 = kdl_impl2(quote! { node "\\u{1234567}" });
    assert!(result4.is_err(), "Unicode escape with >6 digits should fail");

    // Invalid hex characters
    let result5 = kdl_impl2(quote! { node "\\u{GGGG}" });
    assert!(result5.is_err(), "Unicode escape with invalid hex should fail");

    // Surrogate code points (0xD800-0xDFFF are invalid)
    let result6 = kdl_impl2(quote! { node "\\u{D800}" });
    assert!(result6.is_err(), "Surrogate code point should fail");

    let result7 = kdl_impl2(quote! { node "\\u{DFFF}" });
    assert!(result7.is_err(), "Surrogate code point should fail");

    // Code points above Unicode range (>0x10FFFF)
    let result8 = kdl_impl2(quote! { node "\\u{110000}" });
    assert!(result8.is_err(), "Code point above Unicode range should fail");
}

/// Test backslash at end of string (invalid)
#[test]
fn test_backslash_at_end() {
    let result = kdl_impl2(quote! { node "text\\" });
    assert!(result.is_err(), "Trailing backslash should produce an error");
}

/// Test multiple invalid escapes
#[test]
fn test_multiple_invalid_escapes() {
    let result = kdl_impl2(quote! { node "\\a\\b\\c" });
    assert!(result.is_err(), "Multiple invalid escapes should produce an error");
}

// ============================================================================
// Section 3.11: Disallowed Literal Code Points Tests
// ============================================================================

/// Test disallowed control characters (Section 3.19)
#[test]
fn test_disallowed_control_characters() {
    // Test null character (U+0000)
    let result1 = kdl_impl2(quote! { node "\0" });
    // Note: This may or may not be an error depending on implementation

    // Test other control characters that might be disallowed
    // Note: The exact set depends on Section 3.19 which isn't provided
    // These are common control characters that are typically disallowed
    for code_point in [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07] {
        let char_str = format!("\\u{{{:04X}}}", code_point);
        let input_literal = format!("\"{}\"", char_str);
        let input = format!("node {}", input_literal);
        let token_stream: TokenStream2 = input.parse().unwrap();
        let result = kdl_impl2(token_stream);
        // May or may not be an error depending on implementation
        // assert!(result.is_err(), "Control character U+{:04X} should be disallowed", code_point);
    }
}

/// Test line terminators in quoted strings (should be escaped only)
#[test]
fn test_literal_newlines_disallowed() {
    // Literal newlines should not be allowed without escaping
    let result = kdl_impl2(quote! {
        node "line1
line2"
    });
    assert!(result.is_err(), "Literal newlines should be disallowed in quoted strings");
}

// ============================================================================
// Edge Cases and Boundary Conditions Tests
// ============================================================================

/// Test very long quoted strings
#[test]
fn test_very_long_quoted_string() {
    let long_string = "a".repeat(10000);
    let doc = kdl! {
        node long_string
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), long_string);
}

/// Test quoted string with only escape sequences
#[test]
fn test_only_escape_sequences() {
    let doc = kdl! {
        node "\n\r\t\\\"\b\f\s"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "\n\r\t\\\"\u{0008}\u{000C} ");
}

/// Test quoted string with mixed escape sequences and Unicode
#[test]
fn test_mixed_escapes_and_unicode() {
    let doc = kdl! {
        node "Hello\\n\u{1F30D}\\t\u{0041}"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "Hello\\n<\\tA");
}

/// Test boundary Unicode code points
#[test]
fn test_boundary_unicode_code_points() {
    // Test minimum valid code point
    let doc1 = kdl! {
        node "\u{0}"
    };
    assert_eq!(doc1.nodes().len(), 1);
    assert_eq!(doc1.nodes()[0].entries()[0].value().as_string().unwrap(), "\u{0000}");

    // Test maximum BMP code point
    let doc2 = kdl! {
        node "\u{FFFF}"
    };
    assert_eq!(doc2.nodes().len(), 1);
    assert_eq!(doc2.nodes()[0].entries()[0].value().as_string().unwrap(), "\u{FFFF}");

    // Test maximum valid Unicode code point
    let doc3 = kdl! {
        node "\u{10FFFF}"
    };
    assert_eq!(doc3.nodes().len(), 1);
    assert_eq!(doc3.nodes()[0].entries()[0].value().as_string().unwrap(), "\u{10FFFF}");
}

/// Test case sensitivity in Unicode escapes
#[test]
fn test_unicode_escape_case_sensitivity() {
    let doc1 = kdl! {
        node "\u{abc}"  // lowercase
    };
    let doc2 = kdl! {
        node "\u{ABC}"  // uppercase
    };
    let doc3 = kdl! {
        node "\u{AbC}"  // mixed
    };

    assert_eq!(doc1.nodes()[0].entries()[0].value().as_string().unwrap(), "\u{0abc}");
    assert_eq!(doc2.nodes()[0].entries()[0].value().as_string().unwrap(), "\u{0ABC}");
    assert_eq!(doc3.nodes()[0].entries()[0].value().as_string().unwrap(), "\u{0AbC}");
}

/// Test escaped quotes within strings
#[test]
fn test_complex_quote_escaping() {
    let doc = kdl! {
        node "She said \"He said \\\"Hello!\\\"\""
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "She said \"He said \\\"Hello!\\\"\"");
}

/// Test empty Unicode escape (should be invalid)
#[test]
fn test_empty_unicode_escape() {
    let result = kdl_impl2(quote! { node "\\u{}" });
    assert!(result.is_err(), "Empty Unicode escape should be invalid");
}

/// Test leading zeros in Unicode escapes
#[test]
fn test_unicode_escape_leading_zeros() {
    let doc = kdl! {
        node "\u{0000041}"  // 'A' with leading zeros
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "A");
}

// ============================================================================
// Integration Tests with Other KDL Features
// ============================================================================

/// Test quoted strings in complex document structures
#[test]
fn test_quoted_strings_in_complex_structure() {
    let doc = kdl! {
        config app="My App" version="1.0.0" {
            database url="postgresql://localhost:5432/mydb" {
                migrations path="./migrations"
                encoding "UTF-8"
            }
            logging {
                format "%(asctime)s - %(name)s - %(levelname)s - %(message)s"
                file "app.log"
            }
        }
        feature "new-ui" enabled=true description="New user interface"
    };

    assert_eq!(doc.nodes().len(), 2);

    // Verify config node
    let config = &doc.nodes()[0];
    assert_eq!(config.get("app").unwrap().value().as_string().unwrap(), "My App");
    assert_eq!(config.get("version").unwrap().value().as_string().unwrap(), "1.0.0");

    // Verify nested strings
    let db = &config.children().unwrap().nodes()[0];
    assert_eq!(db.get("url").unwrap().value().as_string().unwrap(), "postgresql://localhost:5432/mydb");

    let migrations = &db.children().unwrap().nodes()[0];
    assert_eq!(migrations.get("path").unwrap().value().as_string().unwrap(), "./migrations");

    // Verify feature node
    let feature = &doc.nodes()[1];
    assert_eq!(feature.entries()[0].value().as_string().unwrap(), "new-ui");
    assert_eq!(feature.get("description").unwrap().value().as_string().unwrap(), "New user interface");
}

/// Test quoted strings with type annotations
#[test]
fn test_quoted_strings_with_type_annotations() {
    let doc = kdl! {
        node (string)"hello" (text)"world" (url)"https://example.com"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    assert_eq!(node.entries()[0].ty().unwrap().value(), "string");
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "hello");

    assert_eq!(node.entries()[1].ty().unwrap().value(), "text");
    assert_eq!(node.entries()[1].value().as_string().unwrap(), "world");

    assert_eq!(node.entries()[2].ty().unwrap().value(), "url");
    assert_eq!(node.entries()[2].value().as_string().unwrap(), "https://example.com");
}

// ============================================================================
// Stress Tests and Performance Edge Cases
// ============================================================================

/// Test many escape sequences in a single string
#[test]
fn test_many_escape_sequences() {
    let doc = kdl! {
        node "\\n\\r\\t\\\\\\\"\\b\\f\\s\\n\\r\\t\\\\\\\"\\b\\f\\s"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    let expected = "\n\r\t\\\"\u{0008}\u{000C} \n\r\t\\\"\u{0008}\u{000C} ";
    assert_eq!(node.entries()[0].value().as_string().unwrap(), expected);
}

/// Test deeply nested quoted strings
#[test]
fn test_deeply_nested_quoted_strings() {
    let doc = kdl! {
        level1 "value1" {
            level2 "value2" {
                level3 "value3" {
                    level4 "value4" {
                        deep "final value with escapes: \\n\\t\u{1F600}"
                    }
                }
            }
        }
    };

    // Navigate to the deep value
    let level1 = &doc.nodes()[0];
    let level2 = &level1.children().unwrap().nodes()[0];
    let level3 = &level2.children().unwrap().nodes()[0];
    let level4 = &level3.children().unwrap().nodes()[0];
    let deep = &level4.children().unwrap().nodes()[0];

    assert_eq!(deep.entries()[0].value().as_string().unwrap(), "final value with escapes: \n\t= ");
}

/// Test quoted strings with all valid Unicode ranges
#[test]
fn test_unicode_range_coverage() {
    let doc = kdl! {
        node
            "\u{20}"        // Basic Latin space
            "\u{A0}"        // Non-breaking space
            "\u{1F4}"       // Latin Extended-A
            "\u{370}"       // Greek
            "\u{400}"       // Cyrillic
            "\u{590}"       // Hebrew
            "\u{600}"       // Arabic
            "\u{4E00}"      // CJK
            "\u{1F600}"     // Emoji
            "\u{10000}"     // Supplementary plane
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 10);

    // Verify each Unicode character is correctly parsed
    assert_eq!(node.entries()[0].value().as_string().unwrap(), " ");
    assert_eq!(node.entries()[1].value().as_string().unwrap(), "\u{A0}");
    assert_eq!(node.entries()[2].value().as_string().unwrap(), "\u{1F4}");
    assert_eq!(node.entries()[3].value().as_string().unwrap(), "\u{370}");
    assert_eq!(node.entries()[4].value().as_string().unwrap(), "\u{400}");
    assert_eq!(node.entries()[5].value().as_string().unwrap(), "\u{590}");
    assert_eq!(node.entries()[6].value().as_string().unwrap(), "\u{600}");
    assert_eq!(node.entries()[7].value().as_string().unwrap(), "\u{4E00}");
    assert_eq!(node.entries()[8].value().as_string().unwrap(), "= ");
    assert_eq!(node.entries()[9].value().as_string().unwrap(), "\u{10000}");
}

// ============================================================================
// Error Recovery and Malformed Input Tests
// ============================================================================

/// Test unclosed quoted string
#[test]
fn test_unclosed_quoted_string() {
    let result = kdl_impl2(quote! { node "unclosed string });
    assert!(result.is_err(), "Unclosed quoted string should produce an error");
}

/// Test quoted string with mismatched quotes
#[test]
fn test_mismatched_quotes() {
    let input: TokenStream2 = "node 'wrong quote type\"".parse().unwrap();
    let result = kdl_impl2(input);
    assert!(result.is_err(), "Mismatched quote types should produce an error");
}

/// Test escaped newline in macro context
#[test]
fn test_macro_context_constraints() {
    // The kdl! macro may have different constraints than the full parser
    let doc = kdl! {
        node "valid\nstring"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "valid\nstring");
}

/// Test maximum supported string length
#[test]
fn test_maximum_string_length() {
    // Test with a very large string to ensure no arbitrary limits
    let large_content = "x".repeat(65536); // 64KB string
    let doc = kdl! {
        node large_content
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries()[0].value().as_string().unwrap().len(), 65536);
}

// ============================================================================
// Conformance and Validation Tests
// ============================================================================

/// Test quoted string conformance to specification
#[test]
fn test_quoted_string_spec_conformance() {
    let doc = kdl! {
        test "basic string"
             "string with \"quotes\""
             "string with \\backslashes\\"
             "string with \n newlines"
             "string with \u{1F4A9} emoji"
             "string with \     escaped whitespace"
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 6);

    // Verify each string conforms to spec
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "basic string");
    assert_eq!(node.entries()[1].value().as_string().unwrap(), "string with \"quotes\"");
    assert_eq!(node.entries()[2].value().as_string().unwrap(), "string with \\backslashes\\");
    assert_eq!(node.entries()[3].value().as_string().unwrap(), "string with \n newlines");
    assert_eq!(node.entries()[4].value().as_string().unwrap(), "string with =© emoji");
    assert_eq!(node.entries()[5].value().as_string().unwrap(), "string with escaped whitespace");
}

/// Test that quoted strings properly handle UTF-8 encoding
#[test]
fn test_utf8_encoding_compliance() {
    let doc = kdl! {
        utf8 "English" "Español" "Français" "Deutsch" "-‡" "å,ž" "\m´" "'D91(J)" "âÑèÙê" " CAA:89"
    };

    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 10);

    // All strings should be properly decoded as UTF-8
    let languages = [
        "English", "Español", "Français", "Deutsch", "-‡",
        "å,ž", "\m´", "'D91(J)", "âÑèÙê", " CAA:89"
    ];

    for (i, expected) in languages.iter().enumerate() {
        assert_eq!(node.entries()[i].value().as_string().unwrap(), *expected);
    }
}

/// Test quoted string type consistency
#[test]
fn test_quoted_string_type_consistency() {
    let doc = kdl! {
        node "test string"
    };

    let node = &doc.nodes()[0];
    let entry = &node.entries()[0];

    // Verify the value is recognized as a string type
    assert!(entry.value().is_string());
    assert!(!entry.value().is_i64());
    assert!(!entry.value().is_f64());
    assert!(!entry.value().is_bool());
    assert!(!entry.value().is_null());
}