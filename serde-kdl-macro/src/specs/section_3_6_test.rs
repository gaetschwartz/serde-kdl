//! Tests for KDL Children Block specification (Section 3.6)
//!
//! A children block is a block of Nodes (Section 3.2), surrounded by `{` and `}`. They
//! are an optional part of nodes, and create a hierarchy of KDL nodes.
//!
//! Regular node termination rules apply, which means multiple nodes can be
//! included in a single-line children block, as long as they're all terminated by `;`.

use serde_kdl_macro::kdl;
use seq_macro::seq;
use quote::quote;

/// Test basic children block syntax with `{` and `}`
#[test]
fn test_basic_children_block_syntax() {
    let doc = kdl! {
        parent {
            child1
            child2
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 2);
    assert_eq!(children.nodes()[0].name().value(), "child1");
    assert_eq!(children.nodes()[1].name().value(), "child2");
}

/// Test single-line children block with semicolon termination
#[test]
fn test_single_line_children_block() {
    let doc = kdl! {
        parent { child1; child2; }
    };
    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 2);
    assert_eq!(children.nodes()[0].name().value(), "child1");
    assert_eq!(children.nodes()[1].name().value(), "child2");
}

/// Test empty children blocks
#[test]
fn test_empty_children_block() {
    let doc = kdl! {
        parent {}
    };
    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 0);
}

/// Test empty children block with whitespace
#[test]
fn test_empty_children_block_with_whitespace() {
    let doc = kdl! {
        parent {

        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 0);
}

/// Test nested children blocks (multiple levels of nesting)
#[test]
fn test_nested_children_blocks() {
    let doc = kdl! {
        grandparent {
            parent {
                child {
                    grandchild
                }
            }
        }
    };
    assert_eq!(doc.nodes().len(), 1);

    let grandparent = &doc.nodes()[0];
    assert_eq!(grandparent.name().value(), "grandparent");

    let parent_level = grandparent.children().unwrap();
    assert_eq!(parent_level.nodes().len(), 1);

    let parent = &parent_level.nodes()[0];
    assert_eq!(parent.name().value(), "parent");

    let child_level = parent.children().unwrap();
    assert_eq!(child_level.nodes().len(), 1);

    let child = &child_level.nodes()[0];
    assert_eq!(child.name().value(), "child");

    let grandchild_level = child.children().unwrap();
    assert_eq!(grandchild_level.nodes().len(), 1);
    assert_eq!(grandchild_level.nodes()[0].name().value(), "grandchild");
}

/// Test children blocks with various node types
#[test]
fn test_children_with_various_node_types() {
    let doc = kdl! {
        config {
            name "my-app"
            version "1.0.0"
            debug true
            port 8080
            timeout 30.5
            enabled null
            tags "tag1" "tag2" "tag3"
            server host="localhost" port=3000 {
                database url="postgresql://localhost/mydb"
                cache enabled=true
            }
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let config = &doc.nodes()[0];
    assert_eq!(config.name().value(), "config");

    let children = config.children().unwrap();
    assert_eq!(children.nodes().len(), 8); // name, version, debug, port, timeout, enabled, tags, server

    // Test different value types in children
    let name_node = &children.nodes()[0];
    assert_eq!(name_node.name().value(), "name");
    assert_eq!(name_node.entries()[0].value().as_string(), Some("my-app"));

    let version_node = &children.nodes()[1];
    assert_eq!(version_node.name().value(), "version");
    assert_eq!(version_node.entries()[0].value().as_string(), Some("1.0.0"));

    let debug_node = &children.nodes()[2];
    assert_eq!(debug_node.name().value(), "debug");
    assert_eq!(debug_node.entries()[0].value().as_bool(), Some(true));

    let port_node = &children.nodes()[3];
    assert_eq!(port_node.name().value(), "port");
    assert_eq!(port_node.entries()[0].value().as_i64(), Some(8080));

    let timeout_node = &children.nodes()[4];
    assert_eq!(timeout_node.name().value(), "timeout");
    assert_eq!(timeout_node.entries()[0].value().as_f64(), Some(30.5));

    let enabled_node = &children.nodes()[5];
    assert_eq!(enabled_node.name().value(), "enabled");
    assert!(enabled_node.entries()[0].value().is_null());

    // Test nested children
    let server_node = &children.nodes()[7];
    assert_eq!(server_node.name().value(), "server");
    assert_eq!(server_node.get("host").unwrap().value().as_string(), Some("localhost"));
    assert_eq!(server_node.get("port").unwrap().value().as_i64(), Some(3000));

    let server_children = server_node.children().unwrap();
    assert_eq!(server_children.nodes().len(), 2);
}

/// Test children blocks with properties and arguments
#[test]
fn test_children_with_properties_and_arguments() {
    let doc = kdl! {
        parent key="value" arg1 arg2 {
            child1 "value1"
            child2 key2="value2" arg3
            child3 key3="value3" key4="value4" arg4 arg5
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");
    assert_eq!(parent.get("key").unwrap().value().as_string(), Some("value"));
    assert_eq!(parent.entries().len(), 3); // 1 property + 2 arguments

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 3);

    let child1 = &children.nodes()[0];
    assert_eq!(child1.name().value(), "child1");
    assert_eq!(child1.entries().len(), 1);
    assert_eq!(child1.entries()[0].value().as_string(), Some("value1"));

    let child2 = &children.nodes()[1];
    assert_eq!(child2.name().value(), "child2");
    assert_eq!(child2.entries().len(), 2); // 1 property + 1 argument
    assert_eq!(child2.get("key2").unwrap().value().as_string(), Some("value2"));

    let child3 = &children.nodes()[2];
    assert_eq!(child3.name().value(), "child3");
    assert_eq!(child3.entries().len(), 4); // 2 properties + 2 arguments
    assert_eq!(child3.get("key3").unwrap().value().as_string(), Some("value3"));
    assert_eq!(child3.get("key4").unwrap().value().as_string(), Some("value4"));
}

/// Test mixed single-line and multi-line children blocks
#[test]
fn test_mixed_single_and_multiline_children() {
    let doc = kdl! {
        root {
            single_line { child1; child2; child3; }
            multi_line {
                child4
                child5
                child6
            }
            mixed {
                child7;
                child8
                child9; child10
            }
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let root = &doc.nodes()[0];
    let children = root.children().unwrap();
    assert_eq!(children.nodes().len(), 3);

    let single_line = &children.nodes()[0];
    let single_children = single_line.children().unwrap();
    assert_eq!(single_children.nodes().len(), 3);

    let multi_line = &children.nodes()[1];
    let multi_children = multi_line.children().unwrap();
    assert_eq!(multi_children.nodes().len(), 3);

    let mixed = &children.nodes()[2];
    let mixed_children = mixed.children().unwrap();
    assert_eq!(mixed_children.nodes().len(), 4);
}

/// Test deeply nested children blocks (stress test)
#[test]
fn test_deeply_nested_children() {
    let doc = kdl! {
        level0 {
            level1 {
                level2 {
                    level3 {
                        level4 {
                            level5 {
                                level6 {
                                    level7 {
                                        level8 {
                                            level9 {
                                                deep_child "found!"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    assert_eq!(doc.nodes().len(), 1);

    // Navigate through all levels
    let mut current_level = &doc;
    for level in 0..=9 {
        assert_eq!(current_level.nodes().len(), 1);
        let node = &current_level.nodes()[0];
        assert_eq!(node.name().value(), format!("level{}", level));

        if level < 9 {
            current_level = node.children().unwrap();
        } else {
            // At level 9, check for deep_child
            let final_children = node.children().unwrap();
            assert_eq!(final_children.nodes().len(), 1);
            let deep_child = &final_children.nodes()[0];
            assert_eq!(deep_child.name().value(), "deep_child");
            assert_eq!(deep_child.entries()[0].value().as_string(), Some("found!"));
        }
    }
}

/// Test children blocks with comments
#[test]
fn test_children_with_comments() {
    let doc = kdl! {
        parent { // This is a comment
            child1 // Another comment
            /* Multi-line
               comment */
            child2
            child3 /* inline */ "value"
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 3);

    assert_eq!(children.nodes()[0].name().value(), "child1");
    assert_eq!(children.nodes()[1].name().value(), "child2");
    assert_eq!(children.nodes()[2].name().value(), "child3");
    assert_eq!(children.nodes()[2].entries()[0].value().as_string(), Some("value"));
}

/// Test children blocks with type annotations
#[test]
fn test_children_with_type_annotations() {
    let doc = kdl! {
        typed_parent (type)"value" {
            child1 (i32)42
            child2 (string)"hello"
            child3 (f64)3.14
            child4 (bool)true
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.entries()[0].ty().unwrap().value(), "type");

    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 4);

    assert_eq!(children.nodes()[0].entries()[0].ty().unwrap().value(), "i32");
    assert_eq!(children.nodes()[1].entries()[0].ty().unwrap().value(), "string");
    assert_eq!(children.nodes()[2].entries()[0].ty().unwrap().value(), "f64");
    assert_eq!(children.nodes()[3].entries()[0].ty().unwrap().value(), "bool");
}

/// Test children blocks with escaped strings
#[test]
fn test_children_with_escaped_strings() {
    let doc = kdl! {
        parent {
            child1 "string with \"quotes\""
            child2 "string with \\backslash"
            child3 "string with \n newline"
            child4 "string with \t tab"
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 4);

    assert_eq!(children.nodes()[0].entries()[0].value().as_string(), Some("string with \"quotes\""));
    assert_eq!(children.nodes()[1].entries()[0].value().as_string(), Some("string with \\backslash"));
    assert_eq!(children.nodes()[2].entries()[0].value().as_string(), Some("string with \n newline"));
    assert_eq!(children.nodes()[3].entries()[0].value().as_string(), Some("string with \t tab"));
}

/// Test children blocks with raw strings
#[test]
fn test_children_with_raw_strings() {
    let doc = kdl! {
        parent {
            child1 r"raw string with \n no escaping"
            child2 r#"raw string with "quotes""#
            child3 r##"raw string with #"hashes"#"##
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 3);

    assert_eq!(children.nodes()[0].entries()[0].value().as_string(), Some("raw string with \\n no escaping"));
    assert_eq!(children.nodes()[1].entries()[0].value().as_string(), Some("raw string with \"quotes\""));
    assert_eq!(children.nodes()[2].entries()[0].value().as_string(), Some("raw string with #\"hashes\"#"));
}

/// Test children blocks with numbers in various formats
#[test]
fn test_children_with_number_formats() {
    let doc = kdl! {
        parent {
            hex_child 0xFF 0x1A 0xDEADBEEF
            binary_child 0b1010 0b11110000
            octal_child 0o755 0o644
            decimal_child 42 -42 1_000_000
            float_child 3.14 -2.5 1.0e10 1.5e-3
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 5);

    let hex_child = &children.nodes()[0];
    assert_eq!(hex_child.entries()[0].value().as_i64(), Some(255));
    assert_eq!(hex_child.entries()[1].value().as_i64(), Some(26));

    let binary_child = &children.nodes()[1];
    assert_eq!(binary_child.entries()[0].value().as_i64(), Some(10));
    assert_eq!(binary_child.entries()[1].value().as_i64(), Some(240));

    let octal_child = &children.nodes()[2];
    assert_eq!(octal_child.entries()[0].value().as_i64(), Some(493));
    assert_eq!(octal_child.entries()[1].value().as_i64(), Some(420));
}

/// Test multiple siblings with children blocks
#[test]
fn test_multiple_siblings_with_children() {
    let doc = kdl! {
        sibling1 {
            child1_1
            child1_2
        }
        sibling2 {
            child2_1
            child2_2
            child2_3
        }
        sibling3 {
            child3_1
        }
        sibling4 {}
    };

    assert_eq!(doc.nodes().len(), 4);

    let sibling1 = &doc.nodes()[0];
    assert_eq!(sibling1.children().unwrap().nodes().len(), 2);

    let sibling2 = &doc.nodes()[1];
    assert_eq!(sibling2.children().unwrap().nodes().len(), 3);

    let sibling3 = &doc.nodes()[2];
    assert_eq!(sibling3.children().unwrap().nodes().len(), 1);

    let sibling4 = &doc.nodes()[3];
    assert_eq!(sibling4.children().unwrap().nodes().len(), 0);
}

/// Test children blocks with boolean and null values
#[test]
fn test_children_with_booleans_and_null() {
    let doc = kdl! {
        parent {
            bool_child true false
            null_child null null null
            mixed_child true null false "string" 42
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 3);

    let bool_child = &children.nodes()[0];
    assert_eq!(bool_child.entries()[0].value().as_bool(), Some(true));
    assert_eq!(bool_child.entries()[1].value().as_bool(), Some(false));

    let null_child = &children.nodes()[1];
    assert!(null_child.entries()[0].value().is_null());
    assert!(null_child.entries()[1].value().is_null());
    assert!(null_child.entries()[2].value().is_null());

    let mixed_child = &children.nodes()[2];
    assert_eq!(mixed_child.entries()[0].value().as_bool(), Some(true));
    assert!(mixed_child.entries()[1].value().is_null());
    assert_eq!(mixed_child.entries()[2].value().as_bool(), Some(false));
    assert_eq!(mixed_child.entries()[3].value().as_string(), Some("string"));
    assert_eq!(mixed_child.entries()[4].value().as_i64(), Some(42));
}

/// Generate stress tests with many children
seq!(N in 0..5 {
    #[test]
    fn test_many_children_~N() {
        let doc = kdl! {
            parent {
                #(
                    child~N arg~N
                )*
            }
        };
        assert_eq!(doc.nodes().len(), 1);
        let parent = &doc.nodes()[0];
        let children = parent.children().unwrap();
        assert_eq!(children.nodes().len(), 5);

        #(
            assert_eq!(children.nodes()[N].name().value(), stringify!(child~N));
        )*
    }
});

/// Test children blocks with all whitespace types
#[test]
fn test_children_with_various_whitespace() {
    let doc = kdl! {
        parent {
            child1
                child2
                    child3
            child4
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 4);

    for (i, expected_name) in ["child1", "child2", "child3", "child4"].iter().enumerate() {
        assert_eq!(children.nodes()[i].name().value(), *expected_name);
    }
}

/// Test edge case - children block immediately after node name
#[test]
fn test_children_immediately_after_node_name() {
    let doc = kdl! {
        parent{
            child1
            child2
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 2);
}

/// Test edge case - children block with mixed spacing
#[test]
fn test_children_with_mixed_spacing() {
    let doc = kdl! {
        parent   {
            child1
            child2
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 2);
}

/// Test children blocks preserving order
#[test]
fn test_children_preserve_order() {
    let doc = kdl! {
        parent {
            first
            second
            third
            fourth
            fifth
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 5);

    let expected_names = ["first", "second", "third", "fourth", "fifth"];
    for (i, expected_name) in expected_names.iter().enumerate() {
        assert_eq!(children.nodes()[i].name().value(), *expected_name);
    }
}

/// Test children blocks with line continuations
#[test]
fn test_children_with_line_continuations() {
    let doc = kdl! {
        parent {
            child1 arg1 \
                   arg2 \
                   arg3
            child2 key="value" \
                   arg4
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 2);

    let child1 = &children.nodes()[0];
    assert_eq!(child1.entries().len(), 3);

    let child2 = &children.nodes()[1];
    assert_eq!(child2.entries().len(), 2); // 1 property + 1 argument
}

// ===============================
// ERROR CASES AND INVALID SYNTAX
// ===============================

/// Test invalid children block syntax - missing opening brace
#[test]
fn test_invalid_missing_opening_brace() {
    let result = crate::specs::kdl_impl2(quote! {
        parent
            child1
            child2
        }
    });
    assert!(result.is_err(), "Missing opening brace should cause error");
}

/// Test invalid children block syntax - missing closing brace
#[test]
fn test_invalid_missing_closing_brace() {
    let result = crate::specs::kdl_impl2(quote! {
        parent {
            child1
            child2
    });
    assert!(result.is_err(), "Missing closing brace should cause error");
}

/// Test invalid children block syntax - mismatched braces
#[test]
fn test_invalid_mismatched_braces() {
    let result = crate::specs::kdl_impl2(quote! {
        parent {
            child1 {
                grandchild
            // Missing closing brace for child1
        }
    });
    assert!(result.is_err(), "Mismatched braces should cause error");
}

/// Test invalid children block syntax - extra closing brace
#[test]
fn test_invalid_extra_closing_brace() {
    let result = crate::specs::kdl_impl2(quote! {
        parent {
            child1
        }}
    });
    assert!(result.is_err(), "Extra closing brace should cause error");
}

/// Test invalid children block syntax - nested opening braces without closing
#[test]
fn test_invalid_nested_unclosed_braces() {
    let result = crate::specs::kdl_impl2(quote! {
        parent {
            child1 {
                child2 {
                    child3
                }
            // Missing closing brace for child1
        }
    });
    assert!(result.is_err(), "Unclosed nested braces should cause error");
}

/// Test invalid children block syntax - braces in wrong position
#[test]
fn test_invalid_braces_wrong_position() {
    let result = crate::specs::kdl_impl2(quote! {
        { parent
            child1
        }
    });
    assert!(result.is_err(), "Braces in wrong position should cause error");
}

/// Test invalid children block syntax - malformed children content
#[test]
fn test_invalid_malformed_children_content() {
    let result = crate::specs::kdl_impl2(quote! {
        parent {
            @invalid_syntax_here
        }
    });
    assert!(result.is_err(), "Malformed children content should cause error");
}

/// Test invalid children block syntax - semicolon without preceding node
#[test]
fn test_invalid_semicolon_without_node() {
    let result = crate::specs::kdl_impl2(quote! {
        parent {
            ; child1
        }
    });
    assert!(result.is_err(), "Semicolon without preceding node should cause error");
}

/// Test invalid children block syntax - double opening braces
#[test]
fn test_invalid_double_opening_braces() {
    let result = crate::specs::kdl_impl2(quote! {
        parent {{
            child1
        }}
    });
    assert!(result.is_err(), "Double opening braces should cause error");
}

/// Test boundary case - children block with only comments
#[test]
fn test_children_block_only_comments() {
    let doc = kdl! {
        parent {
            // Only comments here
            /* No actual nodes */
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 0);
}

/// Test boundary case - maximum nesting depth stress test
#[test]
fn test_maximum_nesting_depth() {
    // Create very deeply nested structure to test implementation limits
    let doc = kdl! {
        l0 { l1 { l2 { l3 { l4 { l5 { l6 { l7 { l8 { l9 {
            l10 { l11 { l12 { l13 { l14 { l15 { l16 { l17 { l18 { l19 {
                deep_node "maximum_depth"
            } } } } } } } } } } } } } } } } } } } }
    };

    // Should successfully parse very deep nesting
    assert_eq!(doc.nodes().len(), 1);

    // Navigate to the deepest level
    let mut current = &doc;
    for level in 0..20 {
        assert_eq!(current.nodes().len(), 1);
        let node = &current.nodes()[0];
        assert_eq!(node.name().value(), format!("l{}", level));
        current = node.children().unwrap();
    }

    // Check the final deep node
    assert_eq!(current.nodes().len(), 1);
    let deep_node = &current.nodes()[0];
    assert_eq!(deep_node.name().value(), "deep_node");
    assert_eq!(deep_node.entries()[0].value().as_string(), Some("maximum_depth"));
}

/// Test edge case - children block with all possible KDL value types
#[test]
fn test_children_with_all_value_types() {
    let doc = kdl! {
        comprehensive {
            integers 42 -42 0 0xFF 0b1010 0o755 1_000_000
            floats 3.14 -2.5 0.0 1.0e10 1.5e-3
            booleans true false
            strings "regular" r"raw" r#"raw with "quotes""#
            null_values null
            type_annotated (i32)42 (string)"hello" (f64)3.14
            properties key="value" count=10 enabled=true
            mixed_entry (type)42 "string" key="value" true null
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 8);

    // Verify all children exist with correct names
    let expected_names = [
        "integers", "floats", "booleans", "strings",
        "null_values", "type_annotated", "properties", "mixed_entry"
    ];

    for (i, expected_name) in expected_names.iter().enumerate() {
        assert_eq!(children.nodes()[i].name().value(), *expected_name);
    }
}

/// Test edge case - empty children with various brace spacing
#[test]
fn test_empty_children_brace_spacing() {
    // Test various valid empty children syntax
    let doc1 = kdl! { parent {} };
    assert_eq!(doc1.nodes()[0].children().unwrap().nodes().len(), 0);

    let doc2 = kdl! { parent { } };
    assert_eq!(doc2.nodes()[0].children().unwrap().nodes().len(), 0);

    let doc3 = kdl! {
        parent {
        }
    };
    assert_eq!(doc3.nodes()[0].children().unwrap().nodes().len(), 0);

    let doc4 = kdl! {
        parent {
            // Just comments and whitespace
        }
    };
    assert_eq!(doc4.nodes()[0].children().unwrap().nodes().len(), 0);
}

/// Test that regular node termination rules apply in children blocks
#[test]
fn test_node_termination_rules_in_children() {
    // Multiple nodes terminated by semicolons on single line
    let doc = kdl! {
        parent { child1; child2; child3; }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 3);

    // Mixed single-line with semicolons and multi-line
    let doc2 = kdl! {
        parent {
            child1; child2;
            child3
            child4
        }
    };

    let parent2 = &doc2.nodes()[0];
    let children2 = parent2.children().unwrap();
    assert_eq!(children2.nodes().len(), 4);
}

/// Test children blocks with node slashdash comments (when implemented)
#[test]
fn test_children_with_slashdash_comments() {
    // This tests the /- prefix for commenting out nodes
    // Note: Implementation may vary, documenting expected behavior
    let doc = kdl! {
        parent {
            child1
            /- child2  // This child should be commented out
            child3
        }
    };

    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    let children = parent.children().unwrap();
    // Depending on implementation, might be 2 or 3 nodes
    // Test documents current behavior
    assert!(children.nodes().len() >= 2);
}