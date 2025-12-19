//! Tests for KDL Section 3.9: String
//!
//! According to the KDL specification Section 3.9:
//! - Strings in KDL represent textual UTF-8 Values (Section 3.7)
//! - A String is either an Identifier String (Section 3.10), Quoted String (Section 3.11),
//!   Multi-Line String (Section 3.12), or Raw String (Section 3.13) variant
//! - Strings MUST be represented as UTF-8 values
//! - Strings MUST NOT include disallowed literal code points (Section 3.19) directly
//! - Quoted and Multi-Line Strings may include disallowed code points as values
//!   by representing them with their corresponding \u{...} escape

use super::doc_to_string;
use insta::assert_snapshot;
use serde_kdl_macros::kdl;

#[test]
fn test_string_types_and_contexts() {
    // Test all string types in various contexts: quoted strings with special characters,
    // strings as node names, property keys, arguments, property values, and with type annotations
    let doc = kdl! {
        node1 "hello world" "with, punctuation!" ""
        "dash-separated" "identifier-value"
        "quoted-node" "arg1" "arg2" key="value" "prop-key"="prop-value"
        unicode "Hello 世界" "🌍" "Здравствуй мир" "مرحبا بالعالم"
        typed url=(url)"https://example.com" email=(email)"test@example.com"
    };

    assert_snapshot!(doc_to_string(doc), @r#"
node1 "hello world" "with, punctuation!" "" dash-separated identifier-value quoted-node arg1 arg2 key=value prop-key=prop-value
unicode "Hello 世界" 🌍 "Здравствуй мир" "مرحبا بالعالم"
typed url=(url)"https://example.com" email=(email)test@example.com
"#);
}

#[test]
fn test_unicode_escapes_and_disallowed_codepoints() {
    // Test Unicode escape sequences including basic escapes, non-ASCII characters,
    // multiple escapes, edge cases (min/max code points), and disallowed code points
    // (control characters) that must be represented via \u{...} escapes per Section 3.19
    let doc = kdl! {
        basic "\u{41}" "\u{48}\u{65}\u{6C}\u{6C}\u{6F}"
        emoji "\u{1F30D}" "\u{1F4DD}"
        cjk "\u{4E16}" "\u{754C}"
        mixed "Hello \u{1F30D} World" "Test \u{2713} \u{2717}"
        edges "\u{ABCD}" "\u{abcd}" "\u{10FFFF}"
        bidi "\u{200E}" "\u{200F}"
    };

    assert_snapshot!(doc_to_string(doc), @r#"
basic A Hello
emoji 🌍 📝
cjk 世 界
mixed "Hello 🌍 World" "Test ✓ ✗"
edges ꯍ ꯍ 􏿿
bidi "‎" "‏"
"#);
}

#[test]
fn test_strings_in_nested_structures() {
    // Test strings in complex nested node hierarchies with various string types,
    // empty strings, single characters, and strings with spaces to ensure proper
    // parsing in realistic document structures
    let doc = kdl! {
        root {
            config environment="production" app_name="My Application" version="1.0.0"

            localization {
                lang "en" greeting="Hello 👋"
                lang "zh" greeting="你好 👋"
            }

            database {
                connection host="localhost" port=5432
                tables {
                    users name="users_table" schema="public"
                    "posts-table" title="Post \u{1F4DD}" status="active"
                }
            }

            "edge-cases" "" "a" " " "   spaced   "
        }
    };

    assert_snapshot!(doc_to_string(doc), @r#"
    root {
        config environment=production app_name="My Application" version="1.0.0"
        localization {
            lang en greeting="Hello 👋"
            lang zh greeting="你好 👋"
        }
        database {
            connection host=localhost port=5432
            tables {
                users posts-table name=users_table schema=public title="Post 📝" status=active
            }
        }
        edge-cases "" a " " "   spaced   "
    }
    "#);
}
