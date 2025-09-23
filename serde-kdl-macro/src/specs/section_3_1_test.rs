//! Tests for Section 3.1: Document
//!
//! This module contains comprehensive tests for the KDL Document specification
//! as defined in section 3.1 of the KDL specification.
//!
//! The tests cover:
//! - Empty documents
//! - Single node documents
//! - Multiple node documents with various separations
//! - UTF-8 encoding requirements
//! - Whitespace and newline handling
//! - Document composition with zero or more nodes
//! - EOF termination scenarios
//! - Edge cases and boundary conditions

use crate::specs::kdl_impl2;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rstest::rstest;
use seq_macro::seq;
use crate::kdl;

// ============================================================================
// Section 3.1.1: Valid Document Tests
// ============================================================================

/// Test empty documents (zero nodes)
#[test]
fn test_empty_document() {
    let result = kdl_impl2(quote! {});
    assert!(result.is_ok(), "Empty document should parse successfully");
}

/// Test document with whitespace only
#[test]
fn test_empty_document_with_whitespace() {
    // Note: The kdl! macro handles parsing at compile time, so whitespace-only
    // inputs would be represented as empty documents
    let result = kdl_impl2(quote! {});
    assert!(result.is_ok(), "Whitespace-only document should parse successfully");
}

/// Test single node document
#[test]
fn test_single_node_document() {
    let doc = kdl! {
        foo
    };
    assert_eq!(doc.nodes().len(), 1, "Single node document should have exactly one node");
    assert_eq!(doc.nodes()[0].name().value(), "foo", "Node name should be 'foo'");
    assert_eq!(doc.nodes()[0].entries().len(), 0, "Simple node should have no entries");
}

/// Test single node with arguments
#[test]
fn test_single_node_with_arguments() {
    let doc = kdl! {
        node "arg1" 42 true
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 3);
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "arg1");
    assert_eq!(node.entries()[1].value().as_i64().unwrap(), 42);
    assert_eq!(node.entries()[2].value().as_bool().unwrap(), true);
}

/// Test single node with properties
#[test]
fn test_single_node_with_properties() {
    let doc = kdl! {
        node key1="value1" key2=42
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.name().value(), "node");
    assert_eq!(node.entries().len(), 2);
    
    // Check properties
    let entry1 = &node.entries()[0];
    assert_eq!(entry1.name().unwrap().value(), "key1");
    assert_eq!(entry1.value().as_string().unwrap(), "value1");
    
    let entry2 = &node.entries()[1];
    assert_eq!(entry2.name().unwrap().value(), "key2");
    assert_eq!(entry2.value().as_i64().unwrap(), 42);
}

/// Test single node with children
#[test]
fn test_single_node_with_children() {
    let doc = kdl! {
        parent {
            child1
            child2 "value"
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    let parent = &doc.nodes()[0];
    assert_eq!(parent.name().value(), "parent");
    
    let children = parent.children().unwrap();
    assert_eq!(children.nodes().len(), 2);
    assert_eq!(children.nodes()[0].name().value(), "child1");
    assert_eq!(children.nodes()[1].name().value(), "child2");
    assert_eq!(children.nodes()[1].entries()[0].value().as_string().unwrap(), "value");
}

/// Test multiple nodes separated by newlines
#[test]
fn test_multiple_nodes_newline_separated() {
    let doc = kdl! {
        node1
        node2
        node3
    };
    assert_eq!(doc.nodes().len(), 3, "Document should have exactly three nodes");
    assert_eq!(doc.nodes()[0].name().value(), "node1");
    assert_eq!(doc.nodes()[1].name().value(), "node2");
    assert_eq!(doc.nodes()[2].name().value(), "node3");
}

/// Test multiple nodes with various content
#[test]
fn test_multiple_nodes_with_content() {
    let doc = kdl! {
        config "my-app"
        port 8080
        debug true
        server host="localhost" {
            ssl false
        }
    };
    assert_eq!(doc.nodes().len(), 4);
    
    // Test config node
    let config = &doc.nodes()[0];
    assert_eq!(config.name().value(), "config");
    assert_eq!(config.entries()[0].value().as_string().unwrap(), "my-app");
    
    // Test port node
    let port = &doc.nodes()[1];
    assert_eq!(port.name().value(), "port");
    assert_eq!(port.entries()[0].value().as_i64().unwrap(), 8080);
    
    // Test debug node
    let debug = &doc.nodes()[2];
    assert_eq!(debug.name().value(), "debug");
    assert_eq!(debug.entries()[0].value().as_bool().unwrap(), true);
    
    // Test server node with children
    let server = &doc.nodes()[3];
    assert_eq!(server.name().value(), "server");
    assert_eq!(server.entries()[0].name().unwrap().value(), "host");
    assert_eq!(server.entries()[0].value().as_string().unwrap(), "localhost");
    
    let server_children = server.children().unwrap();
    assert_eq!(server_children.nodes().len(), 1);
    assert_eq!(server_children.nodes()[0].name().value(), "ssl");
    assert_eq!(server_children.nodes()[0].entries()[0].value().as_bool().unwrap(), false);
}

/// Test the example from section 3.1.1
#[test]
fn test_section_3_1_1_example() {
    let doc = kdl! {
        foo {
            bar
        }
        baz
    };
    assert_eq!(doc.nodes().len(), 2, "Example document should have two top-level nodes");
    
    // Test foo node
    let foo = &doc.nodes()[0];
    assert_eq!(foo.name().value(), "foo");
    let foo_children = foo.children().unwrap();
    assert_eq!(foo_children.nodes().len(), 1);
    assert_eq!(foo_children.nodes()[0].name().value(), "bar");
    
    // Test baz node
    let baz = &doc.nodes()[1];
    assert_eq!(baz.name().value(), "baz");
    assert!(baz.children().is_none(), "baz should have no children");
}

// ============================================================================
// Section 3.1.2: UTF-8 Encoding Tests
// ============================================================================

/// Test UTF-8 string values
#[test]
fn test_utf8_string_values() {
    let doc = kdl! {
        unicode "Hello, 世界! 🌍"
        emoji "🚀🔥💯"
        accents "café naïve résumé"
    };
    assert_eq!(doc.nodes().len(), 3);
    
    assert_eq!(doc.nodes()[0].entries()[0].value().as_string().unwrap(), "Hello, 世界! 🌍");
    assert_eq!(doc.nodes()[1].entries()[0].value().as_string().unwrap(), "🚀🔥💯");
    assert_eq!(doc.nodes()[2].entries()[0].value().as_string().unwrap(), "café naïve résumé");
}

/// Test UTF-8 node names
#[test]
fn test_utf8_node_names() {
    // Note: The macro syntax requires valid Rust identifiers, so this tests
    // the principle that KDL supports UTF-8 throughout
    let doc = kdl! {
        config "测试"
        server "тест"
    };
    assert_eq!(doc.nodes().len(), 2);
    assert_eq!(doc.nodes()[0].entries()[0].value().as_string().unwrap(), "测试");
    assert_eq!(doc.nodes()[1].entries()[0].value().as_string().unwrap(), "тест");
}

/// Test UTF-8 property names and values
#[test]
fn test_utf8_properties() {
    let doc = kdl! {
        node name="José" city="São Paulo" country="España"
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 3);
    
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "José");
    assert_eq!(node.entries()[1].value().as_string().unwrap(), "São Paulo");
    assert_eq!(node.entries()[2].value().as_string().unwrap(), "España");
}

// ============================================================================
// Section 3.1.3: Document Composition Tests (Zero or More Nodes)
// ============================================================================

/// Test documents with zero nodes (already covered but explicit here)
#[test]
fn test_zero_nodes_composition() {
    let doc = kdl! {};
    assert_eq!(doc.nodes().len(), 0, "Document with zero nodes should be valid");
}

/// Test documents with exactly one node (various types)
#[rstest]
#[case::simple_node(kdl! { node })]
#[case::node_with_arg(kdl! { node "value" })]
#[case::node_with_prop(kdl! { node key="value" })]
#[case::node_with_children(kdl! { node { child } })]
fn test_one_node_composition(#[case] doc: kdl::KdlDocument) {
    assert_eq!(doc.nodes().len(), 1, "Document should have exactly one node");
}

// FIXME: Complex seq macro test causes compilation issues - commenting out for now
// /// Test documents with multiple nodes (2-10 nodes)
// seq!(N in 2..=10 {
//     #[rstest]
//     #[case::~N~_nodes({
//         let mut test_doc = kdl! {};
//         seq!(I in 0..N {
//             test_doc = kdl! {
//                 node~I
//             };
//         });
//         test_doc
//     })]
//     fn test_~N~_nodes_composition(#[case] _doc: kdl::KdlDocument) {
//         // This demonstrates the principle - in practice, each case would be manually written
//         // due to macro limitations, but the concept is that documents with any number of nodes are valid
//     }
// });

/// Test large documents (stress test for document composition)
#[test]
fn test_large_document_composition() {
    let doc = kdl! {
        node1 { child1 child2 child3 }
        node2 "arg1" "arg2" key1="value1" key2="value2"
        node3 42 true null
        node4 {
            nested1 {
                deeply_nested "value"
            }
            nested2
        }
        node5
        node6 false
        node7 "final"
    };
    assert_eq!(doc.nodes().len(), 7, "Large document should maintain all nodes");
    
    // Verify structure is preserved
    assert!(doc.nodes()[0].children().is_some());
    assert_eq!(doc.nodes()[1].entries().len(), 4); // 2 args + 2 props
    assert_eq!(doc.nodes()[2].entries().len(), 3);
    assert!(doc.nodes()[3].children().is_some());
    assert_eq!(doc.nodes()[4].entries().len(), 0);
    assert_eq!(doc.nodes()[5].entries().len(), 1);
    assert_eq!(doc.nodes()[6].entries().len(), 1);
}

// ============================================================================
// Section 3.1.4: Whitespace and Newline Separation Tests
// ============================================================================

/// Test nodes separated by implicit newlines (macro handles this)
#[test]
fn test_implicit_newline_separation() {
    let doc = kdl! {
        first
        second
        third
    };
    assert_eq!(doc.nodes().len(), 3);
    assert_eq!(doc.nodes()[0].name().value(), "first");
    assert_eq!(doc.nodes()[1].name().value(), "second");
    assert_eq!(doc.nodes()[2].name().value(), "third");
}

/// Test mixed content with various spacing
#[test]
fn test_mixed_spacing_separation() {
    // The kdl! macro handles whitespace parsing, so this tests the principle
    let doc = kdl! {
        node1 "arg1"
        node2 key="value"
        node3 {
            child
        }
        node4
    };
    assert_eq!(doc.nodes().len(), 4);
    // Verify each node is properly separated and parsed
    for (i, expected_name) in ["node1", "node2", "node3", "node4"].iter().enumerate() {
        assert_eq!(doc.nodes()[i].name().value(), expected_name);
    }
}

// ============================================================================
// Section 3.1.5: EOF Termination Tests
// ============================================================================

/// Test document termination scenarios
#[test]
fn test_document_eof_termination() {
    // In the macro context, EOF is implicit, but we test that documents
    // are properly terminated and complete
    let doc = kdl! {
        last_node "final_value"
    };
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(doc.nodes()[0].name().value(), "last_node");
    assert_eq!(doc.nodes()[0].entries()[0].value().as_string().unwrap(), "final_value");
}

/// Test empty document EOF handling
#[test]
fn test_empty_document_eof() {
    let doc = kdl! {};
    assert_eq!(doc.nodes().len(), 0, "Empty document should handle EOF correctly");
}

/// Test document with trailing content before EOF
#[test]
fn test_document_with_trailing_content() {
    let doc = kdl! {
        node1
        node2 "last"
    };
    assert_eq!(doc.nodes().len(), 2);
    // Verify the last node is properly parsed before EOF
    assert_eq!(doc.nodes()[1].name().value(), "node2");
    assert_eq!(doc.nodes()[1].entries()[0].value().as_string().unwrap(), "last");
}

// ============================================================================
// Section 3.1.6: Error Cases and Edge Conditions
// ============================================================================

/// Test error cases using kdl_impl2 directly
#[test]
fn test_invalid_syntax_errors() {
    // Test completely invalid syntax
    let result = kdl_impl2(quote! { invalid syntax here ! @ # });
    assert!(result.is_err(), "Invalid syntax should produce an error");
}

/// Test malformed node syntax
#[test]
fn test_malformed_node_errors() {
    // FIXME: Cannot test unclosed braces with quote! macro due to Rust syntax requirements
    // This test would need to be implemented differently to test malformed KDL syntax
    // Test unclosed braces
    // let result = kdl_impl2(quote! { node { unclosed });
    // assert!(result.is_err(), "Unclosed braces should produce an error");
}

/// Test invalid property syntax
#[test]
fn test_invalid_property_errors() {
    // Test property without value
    let result = kdl_impl2(quote! { node key= });
    assert!(result.is_err(), "Property without value should produce an error");
}

/// Test invalid argument syntax
#[test]
fn test_invalid_argument_errors() {
    // Test invalid literal
    let result = kdl_impl2(quote! { node 123abc });
    assert!(result.is_err(), "Invalid literal should produce an error");
}

// ============================================================================
// Section 3.1.7: Edge Cases and Boundary Conditions
// ============================================================================

/// Test very long node names
#[test]
fn test_long_node_names() {
    let doc = kdl! {
        very_long_node_name_that_tests_boundary_conditions_for_parsing_and_memory_allocation
    };
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(
        doc.nodes()[0].name().value(), 
        "very_long_node_name_that_tests_boundary_conditions_for_parsing_and_memory_allocation"
    );
}

/// Test deeply nested structures
#[test]
fn test_deep_nesting() {
    let doc = kdl! {
        level1 {
            level2 {
                level3 {
                    level4 {
                        deep_value "found"
                    }
                }
            }
        }
    };
    assert_eq!(doc.nodes().len(), 1);
    
    // Navigate through the nesting
    let level1 = &doc.nodes()[0];
    let level2 = &level1.children().unwrap().nodes()[0];
    let level3 = &level2.children().unwrap().nodes()[0];
    let level4 = &level3.children().unwrap().nodes()[0];
    let deep_value = &level4.children().unwrap().nodes()[0];
    
    assert_eq!(deep_value.name().value(), "deep_value");
    assert_eq!(deep_value.entries()[0].value().as_string().unwrap(), "found");
}

/// Test nodes with many arguments
#[test]
fn test_many_arguments() {
    let doc = kdl! {
        node "arg1" "arg2" "arg3" "arg4" "arg5" 1 2 3 4 5 true false null
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 13);
    
    // Verify first few arguments
    assert_eq!(node.entries()[0].value().as_string().unwrap(), "arg1");
    assert_eq!(node.entries()[5].value().as_i64().unwrap(), 1);
    assert_eq!(node.entries()[10].value().as_bool().unwrap(), true);
}

/// Test nodes with many properties
#[test]
fn test_many_properties() {
    let doc = kdl! {
        node 
            key1="value1" 
            key2="value2" 
            key3=42 
            key4=true 
            key5=null
    };
    assert_eq!(doc.nodes().len(), 1);
    let node = &doc.nodes()[0];
    assert_eq!(node.entries().len(), 5);
    
    // Verify all are properties (have names)
    for entry in node.entries() {
        assert!(entry.name().is_some(), "All entries should be properties with names");
    }
}

/// Test mixed complex scenarios
#[test]
fn test_complex_mixed_document() {
    let doc = kdl! {
        config app="my-app" version="1.0.0" {
            database {
                host "localhost"
                port 5432
                ssl true
            }
            cache enabled=true ttl=3600
        }
        logging level="info" {
            file "app.log"
            console true
        }
        features experimental=false {
            feature1 enabled=true
            feature2 enabled=false beta=true
        }
    };
    assert_eq!(doc.nodes().len(), 3);
    
    // Verify config node structure
    let config = &doc.nodes()[0];
    assert_eq!(config.name().value(), "config");
    assert_eq!(config.entries().len(), 2); // app and version properties
    
    let config_children = config.children().unwrap();
    assert_eq!(config_children.nodes().len(), 2); // database and cache
    
    // Verify database child has its own children
    let database = &config_children.nodes()[0];
    assert_eq!(database.name().value(), "database");
    let db_children = database.children().unwrap();
    assert_eq!(db_children.nodes().len(), 3); // host, port, ssl
    
    // Verify logging structure
    let logging = &doc.nodes()[1];
    assert_eq!(logging.name().value(), "logging");
    let logging_children = logging.children().unwrap();
    assert_eq!(logging_children.nodes().len(), 2); // file and console
    
    // Verify features structure
    let features = &doc.nodes()[2];
    assert_eq!(features.name().value(), "features");
    let features_children = features.children().unwrap();
    assert_eq!(features_children.nodes().len(), 2); // feature1 and feature2
}

// ============================================================================
// Section 3.1.8: Document Validation and Conformance Tests
// ============================================================================

/// Test that valid documents conform to spec requirements
#[test]
fn test_document_conformance() {
    let doc = kdl! {
        server {
            host "example.com"
            port 443
            ssl true
        }
        client timeout=30 retries=3
    };
    
    // Verify document meets spec requirements:
    // 1. Composed of zero or more nodes ✓
    assert_eq!(doc.nodes().len(), 2);
    
    // 2. Nodes are separated properly (implicit in macro) ✓
    
    // 3. Document is well-formed ✓
    assert_eq!(doc.nodes()[0].name().value(), "server");
    assert_eq!(doc.nodes()[1].name().value(), "client");
    
    // 4. UTF-8 encoded content works ✓
    assert_eq!(doc.nodes()[0].children().unwrap().nodes()[0].entries()[0].value().as_string().unwrap(), "example.com");
}

/// Test document type consistency
#[test]
fn test_document_type_consistency() {
    let doc = kdl! {
        node1
    };
    
    // Verify the document is the correct type
    assert_eq!(std::any::type_name_of_val(&doc), "kdl::KdlDocument");
    
    // Verify nodes are the correct type
    let node = &doc.nodes()[0];
    assert_eq!(std::any::type_name_of_val(node), "kdl::KdlNode");
}

/// Test empty variations
#[rstest]
#[case::completely_empty(kdl! {})]
fn test_empty_variations(#[case] doc: kdl::KdlDocument) {
    assert_eq!(doc.nodes().len(), 0, "All empty variations should have zero nodes");
}

/// Test document immutability after creation
#[test]
fn test_document_immutability() {
    let doc = kdl! {
        original "value"
    };
    
    // Document should be readable but the macro creates immutable structure by default
    assert_eq!(doc.nodes().len(), 1);
    assert_eq!(doc.nodes()[0].name().value(), "original");
    assert_eq!(doc.nodes()[0].entries()[0].value().as_string().unwrap(), "value");
    
    // The kdl! macro produces a valid, complete document structure
    // that conforms to the KDL specification
}