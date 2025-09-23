//! A KDL macro that allows writing KDL syntax directly in Rust code.
//!
//! This crate provides a procedural macro for embedding KDL (KDL Document Language)
//! syntax directly in Rust source code. The macro parses the KDL at compile time
//! and generates the corresponding kdl crate data structures.

use proc_macro::TokenStream;

// Module declarations
mod ast;
mod validation;
mod utils;
mod parser;
mod codegen;
mod parse;

#[cfg(test)]
mod specs;

/// A KDL macro that allows writing KDL syntax directly in Rust code.
///
/// # Examples
///
/// ```rust
/// use serde_kdl_macro::kdl;
///
/// // Simple node with value
/// let doc = kdl! { node 42 };
///
/// // Node with properties
/// let doc = kdl! { node key="value" 42 };
///
/// // Nested nodes
/// let doc = kdl! {
///     parent {
///         child "value"
///     }
/// };
/// ```
///
/// # Line Continuation Support
///
/// This macro supports KDL line continuations (Section 3.3 of the KDL specification).
/// Line continuations allow nodes to be spread across multiple lines using a backslash
/// followed by optional whitespace/comments and a newline.
///
/// Note: Due to Rust's lexer limitations, line continuations in the macro syntax
/// are processed through string preprocessing before parsing.
#[proc_macro]
pub fn kdl(input: TokenStream) -> TokenStream {
    match parser::kdl_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}






























#[cfg(test)]
mod tests {
    use syn::parse_quote;
    use crate::ast::{KdlDocument, KdlValue};

    #[test]
    fn test_simple_node_parsing() {
        let input: KdlDocument = parse_quote! { node 42 };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].name, "node");
        assert_eq!(input.nodes[0].arguments.len(), 1);
        assert!(matches!(input.nodes[0].arguments[0], KdlValue::Integer(42)));
    }

    #[test]
    fn test_node_with_properties() {
        let input: KdlDocument = parse_quote! { node key="value" 42 };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].properties.len(), 1);
        assert_eq!(input.nodes[0].properties[0].key, "key");
        assert_eq!(input.nodes[0].arguments.len(), 1);
    }

    #[test]
    fn test_nested_nodes() {
        let input: KdlDocument = parse_quote! {
            parent {
                child "value"
            }
        };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].children.len(), 1);
        assert_eq!(input.nodes[0].children[0].name, "child");
    }

    #[test]
    fn test_multiple_root_nodes() {
        let input: KdlDocument = parse_quote! {
            node1 "value1"
            node2 "value2"
        };
        assert_eq!(input.nodes.len(), 2);
        assert_eq!(input.nodes[0].name, "node1");
        assert_eq!(input.nodes[1].name, "node2");
    }

    #[test]
    fn test_hyphenated_identifiers() {
        let input: KdlDocument = parse_quote! { runs-on "ubuntu-latest" };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].name, "runs-on");
        assert_eq!(input.nodes[0].arguments.len(), 1);
    }

    #[test]
    fn test_boolean_values() {
        let input: KdlDocument = parse_quote! { node true false #true #false };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].arguments.len(), 4);
        assert!(matches!(
            input.nodes[0].arguments[0],
            KdlValue::Boolean(true)
        ));
        assert!(matches!(
            input.nodes[0].arguments[1],
            KdlValue::Boolean(false)
        ));
        assert!(matches!(
            input.nodes[0].arguments[2],
            KdlValue::Boolean(true)
        ));
        assert!(matches!(
            input.nodes[0].arguments[3],
            KdlValue::Boolean(false)
        ));
    }

    #[test]
    fn test_mixed_arguments_and_properties() {
        let input: KdlDocument = parse_quote! { node "arg1" key="value" "arg2" };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].arguments.len(), 2);
        assert_eq!(input.nodes[0].properties.len(), 1);
        assert_eq!(input.nodes[0].properties[0].key, "key");
    }

    #[test]
    fn test_keywords_as_identifiers() {
        // If you need an identifier that contains special characters like #,
        // it must be quoted in KDL
        let input: KdlDocument = parse_quote! { "r#override" #true };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].name, "r#override");
        assert_eq!(input.nodes[0].arguments.len(), 1);
        assert!(matches!(
            input.nodes[0].arguments[0],
            KdlValue::Boolean(true)
        ));
    }

    #[test]
    fn test_string_values() {
        let input: KdlDocument = parse_quote! { node "string" "other-value" };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].arguments.len(), 2);
        assert!(matches!(input.nodes[0].arguments[0], KdlValue::String(_)));
        assert!(matches!(input.nodes[0].arguments[1], KdlValue::String(_)));
    }
}
