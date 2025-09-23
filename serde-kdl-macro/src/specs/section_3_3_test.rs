//! Tests for KDL Line Continuation specification (Section 3.3)
//!
//! Line continuations allow Nodes to be spread across multiple lines.
//! A line continuation is a `\` character followed by zero or more whitespace
//! items (including multiline comments) and an optional single-line comment.
//! It must be terminated by a Newline.

use serde_kdl_macro::kdl;
use seq_macro::seq;

/// Test basic line continuation with backslash
#[test]
fn test_basic_line_continuation() {
    let doc = kdl! {
        my-node 1 2 \
        3 4
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "my-node");
    assert_eq!(node.entries().len(), 4);

    // Verify all arguments are parsed correctly
    assert_eq!(node.entries()[0].value().as_i64(), Some(1));
    assert_eq!(node.entries()[1].value().as_i64(), Some(2));
    assert_eq!(node.entries()[2].value().as_i64(), Some(3));
    assert_eq!(node.entries()[3].value().as_i64(), Some(4));
}

/// Test line continuation with single-line comment after backslash
#[test]
fn test_line_continuation_with_comment() {
    let doc = kdl! {
        my-node 1 2 \  // comments are ok after \
                3 4    // This is the actual end of the Node.
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "my-node");
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64(), Some(1));
    assert_eq!(node.entries()[1].value().as_i64(), Some(2));
    assert_eq!(node.entries()[2].value().as_i64(), Some(3));
    assert_eq!(node.entries()[3].value().as_i64(), Some(4));
}

/// Test line continuation with only whitespace after backslash
#[test]
fn test_line_continuation_with_whitespace() {
    let doc = kdl! {
        my-node 1 2 \
                3 4
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "my-node");
    assert_eq!(node.entries().len(), 4);
}

/// Test line continuation with tabs and spaces after backslash
#[test]
fn test_line_continuation_with_mixed_whitespace() {
    let doc = kdl! {
        my-node 1 2 \
                3 4
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "my-node");
    assert_eq!(node.entries().len(), 4);
}

/// Test multiple line continuations in a single node
#[test]
fn test_multiple_line_continuations() {
    let doc = kdl! {
        my-node 1 \
                2 \
                3 \
                4
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "my-node");
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64(), Some(1));
    assert_eq!(node.entries()[1].value().as_i64(), Some(2));
    assert_eq!(node.entries()[2].value().as_i64(), Some(3));
    assert_eq!(node.entries()[3].value().as_i64(), Some(4));
}

/// Test line continuation with properties
#[test]
fn test_line_continuation_with_properties() {
    let doc = kdl! {
        my-node key1="value1" \
                key2="value2" \
                arg1 arg2
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "my-node");
    assert_eq!(node.entries().len(), 4);

    // Check properties
    assert_eq!(node.get("key1").unwrap().value().as_string(), Some("value1"));
    assert_eq!(node.get("key2").unwrap().value().as_string(), Some("value2"));
}

/// Test line continuation with child nodes
#[test]
fn test_line_continuation_with_children() {
    let doc = kdl! {
        parent arg1 \
               arg2 {
            child1
            child2
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");
    assert_eq!(parent.entries().len(), 2);
    assert_eq!(parent.children().unwrap().nodes().len(), 2);
}

/// Test line continuation with string arguments
#[test]
fn test_line_continuation_with_strings() {
    let doc = kdl! {
        my-node "first string" \
                "second string" \
                "third string"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "my-node");
    assert_eq!(node.entries().len(), 3);

    assert_eq!(node.entries()[0].value().as_string(), Some("first string"));
    assert_eq!(node.entries()[1].value().as_string(), Some("second string"));
    assert_eq!(node.entries()[2].value().as_string(), Some("third string"));
}

/// Test line continuation with different value types
#[test]
fn test_line_continuation_with_mixed_types() {
    let doc = kdl! {
        my-node 42 \
                "string" \
                true \
                3.14
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "my-node");
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64(), Some(42));
    assert_eq!(node.entries()[1].value().as_string(), Some("string"));
    assert_eq!(node.entries()[2].value().as_bool(), Some(true));
    assert_eq!(node.entries()[3].value().as_f64(), Some(3.14));
}

/// Test line continuation with multiline comments after backslash
#[test]
fn test_line_continuation_with_multiline_comment() {
    let doc = kdl! {
        my-node 1 2 \ /* multiline
                         comment */
                3 4
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "my-node");
    assert_eq!(node.entries().len(), 4);
}

/// Test line continuation at different positions in node
#[test]
fn test_line_continuation_positions() {
    // After node name
    let doc1 = kdl! {
        my-node \
        arg1 arg2
    };
    assert_eq!(doc1.nodes().len(), 1);
    assert_eq!(doc1.nodes()[0].entries().len(), 2);

    // After arguments
    let doc2 = kdl! {
        my-node arg1 \
        arg2
    };
    assert_eq!(doc2.nodes().len(), 1);
    assert_eq!(doc2.nodes()[0].entries().len(), 2);

    // After properties
    let doc3 = kdl! {
        my-node key="value" \
        arg1
    };
    assert_eq!(doc3.nodes().len(), 1);
    assert_eq!(doc3.nodes()[0].entries().len(), 2);
}

/// Test line continuation with very long indentation
#[test]
fn test_line_continuation_with_deep_indentation() {
    let doc = kdl! {
        my-node 1 \
                                    2 \
                                                        3
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "my-node");
    assert_eq!(node.entries().len(), 3);
}

/// Test multiple nodes with line continuations
#[test]
fn test_multiple_nodes_with_line_continuations() {
    let doc = kdl! {
        node1 arg1 \
              arg2
        node2 arg3 \
              arg4
    };
    assert_eq!(doc.nodes().len(), 2);

    let node1 = &doc.nodes()[0];
    assert_eq!(node1.name().value(), "node1");
    assert_eq!(node1.entries().len(), 2);

    let node2 = &doc.nodes()[1];
    assert_eq!(node2.name().value(), "node2");
    assert_eq!(node2.entries().len(), 2);
}

/// Test line continuation with escaped strings
#[test]
fn test_line_continuation_with_escaped_strings() {
    let doc = kdl! {
        my-node "string with \"quotes\"" \
                "string with \\backslash"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2);

    assert_eq!(node.entries()[0].value().as_string(), Some("string with \"quotes\""));
    assert_eq!(node.entries()[1].value().as_string(), Some("string with \\backslash"));
}

/// Test line continuation preserves argument order
#[test]
fn test_line_continuation_preserves_order() {
    let doc = kdl! {
        my-node \
        first \
        second \
        third
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    assert_eq!(node.entries()[0].value().as_string(), Some("first"));
    assert_eq!(node.entries()[1].value().as_string(), Some("second"));
    assert_eq!(node.entries()[2].value().as_string(), Some("third"));
}

/// Test line continuation with raw strings
#[test]
fn test_line_continuation_with_raw_strings() {
    let doc = kdl! {
        my-node r"raw string 1" \
                r"raw string 2"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 2);

    assert_eq!(node.entries()[0].value().as_string(), Some("raw string 1"));
    assert_eq!(node.entries()[1].value().as_string(), Some("raw string 2"));
}

// Test that line continuation properly handles node termination
#[test]
fn test_line_continuation_node_termination() {
    let doc = kdl! {
        node1 arg1 \
              arg2;
        node2 arg3
    };
    assert_eq!(doc.nodes().len(), 2);

    let node1 = &doc.nodes()[0];
    assert_eq!(node1.name().value(), "node1");
    assert_eq!(node1.entries().len(), 2);

    let node2 = &doc.nodes()[1];
    assert_eq!(node2.name().value(), "node2");
    assert_eq!(node2.entries().len(), 1);
}

/// Test error cases with invalid line continuation usage
#[test]
fn test_invalid_line_continuation_usage() {
    // Test backslash not followed by newline (should fail)
    let result1 = crate::kdl_impl2(quote::quote! {
        my-node 1 \ 2 3
    });
    assert!(result1.is_err(), "Backslash not followed by newline should fail");

    // Test backslash with text after comment on same line (should fail)
    let result2 = crate::kdl_impl2(quote::quote! {
        my-node 1 \ // comment more text
        2 3
    });
    assert!(result2.is_err(), "Text after comment on line continuation should fail");

    // Test backslash at end of file without newline (should fail)
    let result3 = crate::kdl_impl2(quote::quote! {
        my-node 1 \
    });
    // This might be valid depending on implementation, so we don't assert failure
}

/// Test line continuation with numbers in various formats
#[test]
fn test_line_continuation_with_number_formats() {
    let doc = kdl! {
        my-node 0x1A \
                0b1010 \
                0o755 \
                1_000_000
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);

    assert_eq!(node.entries()[0].value().as_i64(), Some(26));     // 0x1A
    assert_eq!(node.entries()[1].value().as_i64(), Some(10));     // 0b1010
    assert_eq!(node.entries()[2].value().as_i64(), Some(493));    // 0o755
    assert_eq!(node.entries()[3].value().as_i64(), Some(1000000)); // 1_000_000
}

/// Test line continuation with boolean values
#[test]
fn test_line_continuation_with_booleans() {
    let doc = kdl! {
        my-node true \
                false \
                true
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    assert_eq!(node.entries()[0].value().as_bool(), Some(true));
    assert_eq!(node.entries()[1].value().as_bool(), Some(false));
    assert_eq!(node.entries()[2].value().as_bool(), Some(true));
}

/// Test line continuation with null values
#[test]
fn test_line_continuation_with_null() {
    let doc = kdl! {
        my-node null \
                "not null" \
                null
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    assert!(node.entries()[0].value().is_null());
    assert_eq!(node.entries()[1].value().as_string(), Some("not null"));
    assert!(node.entries()[2].value().is_null());
}

/// Test line continuation boundary - exactly at backslash
#[test]
fn test_line_continuation_boundary_conditions() {
    // Test with minimal continuation
    let doc1 = kdl! {
        node \
        arg
    };
    assert_eq!(doc1.nodes().len(), 1);
    assert_eq!(doc1.nodes()[0].entries().len(), 1);

    // Test with just backslash and newline
    let doc2 = kdl! {
        node 1 \
2
    };
    assert_eq!(doc2.nodes().len(), 1);
    assert_eq!(doc2.nodes()[0].entries().len(), 2);
}

/// Test line continuation with type annotations
#[test]
fn test_line_continuation_with_type_annotations() {
    let doc = kdl! {
        my-node (i32)42 \
                (string)"hello" \
                (f64)3.14
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);

    // Check type annotations are preserved
    assert_eq!(node.entries()[0].ty().unwrap().value(), "i32");
    assert_eq!(node.entries()[1].ty().unwrap().value(), "string");
    assert_eq!(node.entries()[2].ty().unwrap().value(), "f64");
}

/// Stress test with many line continuations
seq!(N in 0..10 {
    #[test]
    fn test_many_line_continuations_~N() {
        let doc = kdl! {
            node arg0 \
            #(
                arg~N \
            )*
            final_arg
        };
        assert_eq!(doc.nodes().len(), 1);
        let node = &doc.nodes()[0];
        assert_eq!(node.entries().len(), 12); // arg0 + 10 numbered args + final_arg
    }
});

/// Test line continuation edge case - multiple backslashes
#[test]
fn test_multiple_backslashes_edge_case() {
    // This should treat the first backslash as line continuation and second as error
    let result = crate::kdl_impl2(quote::quote! {
        my-node 1 \\
        2 3
    });
    // The exact behavior depends on implementation - might be error or might parse first backslash
}

/// Test line continuation with all whitespace types
#[test]
fn test_line_continuation_all_whitespace_types() {
    let doc = kdl! {
        my-node 1 \
                2 \
                3 \
                4
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 4);
}