//! Test implementation of KDL Specification Section 3.10 - Identifier String
//!
//! Tests for identifier string validation according to the KDL specification.
//! This covers both Section 3.10.1 (Non-initial characters) and Section 3.10.2 (Non-identifier characters).

use crate::specs::kdl_impl2;
use quote::quote;

/// Tests for Section 3.10.1 - Non-initial characters
///
/// Tests that certain characters cannot be the first character in an identifier string.
#[cfg(test)]
mod section_3_10_1_non_initial_characters {
    use super::*;

    #[test]
    fn test_valid_initial_characters() {
        // Valid: regular identifiers
        let result = kdl_impl2(quote! {
            foo "arg"
        });
        assert!(result.is_ok(), "Should accept identifier 'foo'");

        let result = kdl_impl2(quote! {
            bar-baz "arg"
        });
        assert!(result.is_ok(), "Should accept identifier 'bar-baz'");

        // Valid: starting with --
        let result = kdl_impl2(quote! {
            --this "arg"
        });
        assert!(result.is_ok(), "Should accept identifier '--this'");

        // Valid: starting with . (not followed by digit)
        let result = kdl_impl2(quote! {
            .md "arg"
        });
        assert!(result.is_ok(), "Should accept identifier '.md'");

        // Valid: starting with + (not followed by digit)
        let result = kdl_impl2(quote! {
            +abc "arg"
        });
        assert!(result.is_ok(), "Should accept identifier '+abc'");

        // Valid: starting with - (not followed by digit)
        let result = kdl_impl2(quote! {
            -xyz "arg"
        });
        assert!(result.is_ok(), "Should accept identifier '-xyz'");

        // Valid: starting with . followed by non-digit
        let result = kdl_impl2(quote! {
            .foo "arg"
        });
        assert!(result.is_ok(), "Should accept identifier '.foo'");
    }

    #[test]
    fn test_invalid_initial_digits() {
        // Invalid: starting with digit
        let result = kdl_impl2(quote! {
            1abc "arg"
        });
        assert!(result.is_err(), "Should reject identifier starting with digit '1abc'");

        let result = kdl_impl2(quote! {
            9xyz "arg"
        });
        assert!(result.is_err(), "Should reject identifier starting with digit '9xyz'");
    }

    #[test]
    fn test_invalid_plus_followed_by_digit() {
        // For these edge cases where Rust's tokenizer might interfere,
        // test the validation function directly
        use crate::validation::validate_identifier_string;

        let result = validate_identifier_string("+1", proc_macro2::Span::call_site());
        assert!(result.is_err(), "Should reject identifier '+1' (+ followed by digit)");

        let result = validate_identifier_string("+123abc", proc_macro2::Span::call_site());
        assert!(result.is_err(), "Should reject identifier '+123abc' (+ followed by digit)");
    }

    #[test]
    fn test_invalid_minus_followed_by_digit() {
        // For these edge cases where Rust's tokenizer might interfere,
        // test the validation function directly
        use crate::validation::validate_identifier_string;

        let result = validate_identifier_string("-1", proc_macro2::Span::call_site());
        assert!(result.is_err(), "Should reject identifier '-1' (- followed by digit)");

        let result = validate_identifier_string("-42abc", proc_macro2::Span::call_site());
        assert!(result.is_err(), "Should reject identifier '-42abc' (- followed by digit)");
    }

    #[test]
    fn test_invalid_dot_followed_by_digit() {
        // For these edge cases where Rust's tokenizer might interfere,
        // test the validation function directly
        use crate::validation::validate_identifier_string;

        let result = validate_identifier_string(".1", proc_macro2::Span::call_site());
        assert!(result.is_err(), "Should reject identifier '.1' (. followed by digit)");

        let result = validate_identifier_string(".123", proc_macro2::Span::call_site());
        assert!(result.is_err(), "Should reject identifier '.123' (. followed by digit)");
    }

    #[test]
    fn test_invalid_sign_dot_digit_pattern() {
        // For these edge cases where Rust's tokenizer might interfere,
        // test the validation function directly
        use crate::validation::validate_identifier_string;

        let result = validate_identifier_string("+.1", proc_macro2::Span::call_site());
        assert!(result.is_err(), "Should reject identifier '+.1' (+. followed by digit)");

        let result = validate_identifier_string("-.1", proc_macro2::Span::call_site());
        assert!(result.is_err(), "Should reject identifier '-.1' (-. followed by digit)");
    }

    #[test]
    fn test_valid_sign_dot_non_digit_pattern() {
        // Valid: +. followed by non-digit
        let result = kdl_impl2(quote! {
            +.foo "arg"
        });
        assert!(result.is_ok(), "Should accept identifier '+.foo' (+. followed by non-digit)");

        // Valid: -. followed by non-digit
        let result = kdl_impl2(quote! {
            -.bar "arg"
        });
        assert!(result.is_ok(), "Should accept identifier '-.bar' (-. followed by non-digit)");
    }
}

/// Tests for Section 3.10.2 - Non-identifier characters
///
/// Tests that certain characters cannot be used anywhere in an identifier string.
#[cfg(test)]
mod section_3_10_2_non_identifier_characters {

    #[test]
    fn test_invalid_punctuation_characters() {
        // Test each disallowed punctuation character
        let invalid_chars = ['(', ')', '{', '}', '[', ']', '/', '\\', '"', '#', ';', '='];

        for ch in invalid_chars {
            let identifier = format!("foo{}bar", ch);

            // Test the validation function directly since we can't use dynamic identifiers in quote!
            use crate::validation::validate_identifier_string;
            let result = validate_identifier_string(&identifier, proc_macro2::Span::call_site());
            assert!(result.is_err(), "Should reject identifier '{}' containing '{}'", identifier, ch);
        }
    }

    #[test]
    fn test_invalid_whitespace_characters() {
        // Test various whitespace characters from Section 3.17
        let whitespace_chars = [
            '\u{0009}', // Character Tabulation
            '\u{0020}', // Space
            '\u{00A0}', // No-Break Space
            '\u{1680}', // Ogham Space Mark
            '\u{2000}', // En Quad
            '\u{2001}', // Em Quad
            '\u{2002}', // En Space
            '\u{2003}', // Em Space
            '\u{2004}', // Three-Per-Em Space
            '\u{2005}', // Four-Per-Em Space
            '\u{2006}', // Six-Per-Em Space
            '\u{2007}', // Figure Space
            '\u{2008}', // Punctuation Space
            '\u{2009}', // Thin Space
            '\u{200A}', // Hair Space
            '\u{202F}', // Narrow No-Break Space
            '\u{205F}', // Medium Mathematical Space
            '\u{3000}', // Ideographic Space
        ];

        for ch in whitespace_chars {
            let identifier = format!("foo{}bar", ch);
            use crate::validation::validate_identifier_string;
            let result = validate_identifier_string(&identifier, proc_macro2::Span::call_site());
            assert!(result.is_err(), "Should reject identifier containing whitespace U+{:04X}", ch as u32);
        }
    }

    #[test]
    fn test_invalid_newline_characters() {
        // Test various newline characters from Section 3.18
        let newline_chars = [
            '\u{000A}', // LF - Line Feed
            '\u{000B}', // VT - Vertical Tab
            '\u{000C}', // FF - Form Feed
            '\u{000D}', // CR - Carriage Return
            '\u{0085}', // NEL - Next Line
            '\u{2028}', // LS - Line Separator
            '\u{2029}', // PS - Paragraph Separator
        ];

        for ch in newline_chars {
            let identifier = format!("foo{}bar", ch);
            use crate::validation::validate_identifier_string;
            let result = validate_identifier_string(&identifier, proc_macro2::Span::call_site());
            assert!(result.is_err(), "Should reject identifier containing newline U+{:04X}", ch as u32);
        }
    }

    #[test]
    fn test_invalid_disallowed_code_points() {
        // Test some disallowed literal code points from Section 3.19
        let disallowed_chars = [
            '\u{0001}', // Control character
            '\u{0008}', // Control character
            '\u{000E}', // Control character
            '\u{001F}', // Control character
            '\u{007F}', // Delete control character
            '\u{200E}', // Direction control
            '\u{200F}', // Direction control
            '\u{202A}', // Direction control
            '\u{202E}', // Direction control
            '\u{2066}', // Direction control
            '\u{2069}', // Direction control
            '\u{FEFF}', // BOM
        ];

        for ch in disallowed_chars {
            let identifier = format!("foo{}bar", ch);
            use crate::validation::validate_identifier_string;
            let result = validate_identifier_string(&identifier, proc_macro2::Span::call_site());
            assert!(result.is_err(), "Should reject identifier containing disallowed code point U+{:04X}", ch as u32);
        }
    }
}

/// Tests for disallowed patterns that look like numbers or keywords
#[cfg(test)]
mod disallowed_patterns {

    #[test]
    fn test_number_like_patterns() {
        // Test identifiers that look like numbers (should be rejected)
        let number_like_identifiers = [
            "1.0v2",   // Looks like number with suffix
            "-1em",    // Looks like negative number with suffix
            ".1",      // Almost a number pattern
            "+.5",     // Almost a number pattern with +
            "-.9",     // Almost a number pattern with -
            "123abc",  // Starts with number
            "+456def", // Starts with + and number
            "-789ghi", // Starts with - and number
        ];

        for identifier in number_like_identifiers {
            use crate::validation::validate_identifier_string;
            let result = validate_identifier_string(identifier, proc_macro2::Span::call_site());
            assert!(result.is_err(), "Should reject number-like identifier '{}'", identifier);
        }
    }

    #[test]
    fn test_keyword_patterns() {
        // Test reserved keywords without # prefix (should be rejected)
        let keywords = ["inf", "-inf", "nan", "true", "false", "null"];

        for keyword in keywords {
            use crate::validation::validate_identifier_string;
            let result = validate_identifier_string(keyword, proc_macro2::Span::call_site());
            assert!(result.is_err(), "Should reject keyword '{}' without # prefix", keyword);
        }
    }

    #[test]
    fn test_valid_keyword_with_hash_prefix() {
        // These should be handled by the type annotation parsing, not identifier validation
        // But identifiers with # are invalid per Section 3.10.2, so they should be rejected
        let hash_keywords = ["#inf", "#nan", "#true", "#false", "#null"];

        for keyword in hash_keywords {
            use crate::validation::validate_identifier_string;
            let result = validate_identifier_string(keyword, proc_macro2::Span::call_site());
            assert!(result.is_err(), "Should reject identifier '{}' containing #", keyword);
        }
    }

    #[test]
    fn test_valid_non_number_patterns() {
        // Test identifiers that are similar to numbers but are valid
        let valid_identifiers = [
            "v1.0",     // Starts with letter
            "em-1",     // Starts with letter
            "foo.1",    // Starts with letter
            "bar+2",    // Starts with letter
            "baz-3",    // Starts with letter
            ".foo",     // Dot not followed by digit
            "+bar",     // Plus not followed by digit
            "-baz",     // Minus not followed by digit
            "--option", // Double dash
            "..path",   // Double dot
        ];

        for identifier in valid_identifiers {
            use crate::validation::validate_identifier_string;
            let result = validate_identifier_string(identifier, proc_macro2::Span::call_site());
            assert!(result.is_ok(), "Should accept valid identifier '{}'", identifier);
        }
    }
}

/// Integration tests for identifier strings in various contexts
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_identifier_as_node_name() {
        // Valid identifier as node name
        let result = kdl_impl2(quote! {
            valid-identifier "argument"
        });
        assert!(result.is_ok(), "Should accept valid identifier as node name");

        // Valid complex identifier as node name
        let result = kdl_impl2(quote! {
            foo-bar-baz "argument"
        });
        assert!(result.is_ok(), "Should accept complex identifier as node name");
    }

    #[test]
    fn test_identifier_as_property_key() {
        // Valid identifier as property key
        let result = kdl_impl2(quote! {
            node valid-key="value"
        });
        assert!(result.is_ok(), "Should accept valid identifier as property key");

        // Multiple identifiers as property keys
        let result = kdl_impl2(quote! {
            node foo-bar="value1" baz-qux="value2"
        });
        assert!(result.is_ok(), "Should accept multiple identifier property keys");
    }

    #[test]
    fn test_quoted_strings_bypass_restrictions() {
        // Quoted strings should bypass identifier restrictions
        let result = kdl_impl2(quote! {
            "1abc" "arg"
        });
        assert!(result.is_ok(), "Should accept quoted string '1abc' as node name");

        let result = kdl_impl2(quote! {
            node "("="value"
        });
        assert!(result.is_ok(), "Should accept quoted string '(' as property key");

        let result = kdl_impl2(quote! {
            "true" "arg"
        });
        assert!(result.is_ok(), "Should accept quoted keyword 'true' as node name");

        let result = kdl_impl2(quote! {
            node ".1"="value"
        });
        assert!(result.is_ok(), "Should accept quoted string '.1' as property key");
    }

    #[test]
    fn test_mixed_identifier_and_quoted_strings() {
        // Mix of identifiers and quoted strings
        let result = kdl_impl2(quote! {
            valid-node "quoted-arg" unquoted-key="quoted-value" "quoted-key"="another-value"
        });
        assert!(result.is_ok(), "Should handle mix of identifiers and quoted strings");
    }

    #[test]
    fn test_identifier_with_children() {
        // Identifier node names with children
        let result = kdl_impl2(quote! {
            parent-node "arg" {
                child-node "child-arg"
                another-child key="value"
            }
        });
        assert!(result.is_ok(), "Should accept identifiers in nested structures");
    }
}

/// Edge case tests for corner cases in identifier validation
#[cfg(test)]
mod edge_cases {

    #[test]
    fn test_empty_identifier() {
        // Empty identifier should be rejected
        use crate::validation::validate_identifier_string;
        let result = validate_identifier_string("", proc_macro2::Span::call_site());
        assert!(result.is_err(), "Should reject empty identifier");
    }

    #[test]
    fn test_single_character_identifiers() {
        use crate::validation::validate_identifier_string;

        // Valid single characters (including +, -, . when not followed by digits)
        let valid_chars = ['a', 'z', 'A', 'Z', '_', '+', '-', '.'];
        for ch in valid_chars {
            let result = validate_identifier_string(&ch.to_string(), proc_macro2::Span::call_site());
            assert!(result.is_ok(), "Should accept valid single character '{}'", ch);
        }

        // Invalid single characters
        let invalid_chars = ['1', '(', ')', '#', ';', '='];
        for ch in invalid_chars {
            let result = validate_identifier_string(&ch.to_string(), proc_macro2::Span::call_site());
            assert!(result.is_err(), "Should reject invalid single character '{}'", ch);
        }
    }

    #[test]
    fn test_unicode_identifiers() {
        // Valid Unicode identifiers
        let unicode_identifiers = ["café", "naïve", "résumé", "東京", "москва"];
        for identifier in unicode_identifiers {
            use crate::validation::validate_identifier_string;
            let result = validate_identifier_string(identifier, proc_macro2::Span::call_site());
            assert!(result.is_ok(), "Should accept valid Unicode identifier '{}'", identifier);
        }
    }

    #[test]
    fn test_very_long_identifier() {
        // Test very long identifier
        let long_identifier = "a".repeat(1000);
        use crate::validation::validate_identifier_string;
        let result = validate_identifier_string(&long_identifier, proc_macro2::Span::call_site());
        assert!(result.is_ok(), "Should accept very long valid identifier");
    }

    #[test]
    fn test_boundary_cases_for_signs() {
        // Test edge cases with signs
        use crate::validation::validate_identifier_string;

        // Just a sign (should be valid)
        assert!(validate_identifier_string("+", proc_macro2::Span::call_site()).is_ok());
        assert!(validate_identifier_string("-", proc_macro2::Span::call_site()).is_ok());

        // Sign followed by non-digit, non-dot
        assert!(validate_identifier_string("+a", proc_macro2::Span::call_site()).is_ok());
        assert!(validate_identifier_string("-b", proc_macro2::Span::call_site()).is_ok());

        // Sign followed by dot, then non-digit
        assert!(validate_identifier_string("+.a", proc_macro2::Span::call_site()).is_ok());
        assert!(validate_identifier_string("-.b", proc_macro2::Span::call_site()).is_ok());
    }
}