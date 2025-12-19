//! Node parsing
//!
//! This module handles parsing of KDL nodes including node names,
//! properties, arguments, and children.

use crate::ast::{KdlNode, KdlProperty, KdlString, KdlValue};
use crate::parse::type_annotation::parse_type_annotation;
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    token::Brace,
    Ident, LitStr, Result, Token,
};

impl Parse for KdlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        // Get the line number where this node starts BEFORE parsing the name
        // (used to distinguish variables from new nodes)
        // FIXME: This doesn't work in rust-analyzer since it returns dummy spans (1:0)
        let node_line = input.span().start().line;

        // Parse optional type annotation for node name
        let (type_annotation, name) = parse_node_name_with_type_annotation(input)?;
        let mut properties = Vec::new();
        let mut arguments = Vec::new();
        let mut children = Vec::new();
        let mut has_children_block = false;

        // Parse arguments and properties
        while !input.is_empty() && !input.peek(Brace) && !input.peek(Token![;]) {
            // Look ahead to see if this is a property (identifier followed by =)
            // Try to parse as property first (string key followed by =)
            // We need more sophisticated lookahead for hyphenated identifiers
            if is_property_ahead(input) {
                // Parse property key (must be a String value according to Section 3.7)
                let key: KdlString = input.parse()?;
                let _eq: Token![=] = input.parse()?;
                let value: KdlValue = input.parse()?;

                properties.push(KdlProperty { key, value });
            } else {
                // Check if this looks like the start of a new node
                // Type annotation (type)name might be a new node, but we need to check carefully
                if input.peek(syn::token::Paren) {
                    // Simple heuristic: if it's (type)identifier, it's likely a new node
                    let checkpoint = input.fork();

                    // Try to parse the type annotation
                    if let Ok(_type_annotation) = parse_type_annotation(&checkpoint) {
                        // If the type annotation is followed by an identifier, check if it's a keyword
                        if checkpoint.peek(Ident) {
                            // This looks like a legitimate new node
                            break;
                        }
                        // Continue parsing as normal
                    }
                } else if input.peek(Ident) {
                    // Use line numbers to distinguish variables from new nodes:
                    // - Same line as node: variable reference
                    // - Different line: new node
                    let ident: Ident = input.fork().parse().unwrap();
                    let ident_line = ident.span().start().line;

                    if ident_line != node_line {
                        // Different line = new node
                        break;
                    }
                    // Same line = variable reference, continue to parse as value
                } else if input.peek(LitStr) {
                    // Use line numbers to distinguish variables from new nodes:
                    // - Same line as node: variable reference
                    // - Different line: new node
                    let ident: LitStr = input.fork().parse().unwrap();
                    let ident_line = ident.span().start().line;

                    if ident_line != node_line {
                        // Different line = new node
                        break;
                    }
                    // Same line = variable reference, continue to parse as value
                }

                // Try to parse as argument (literal value or identifier)
                let fork = input.fork();
                match fork.parse::<KdlValue>() {
                    Ok(value) => {
                        // Successfully parsed as value, commit the parse
                        input.advance_to(&fork);
                        arguments.push(value);
                    }
                    Err(e) => {
                        // If we can't parse as a value and there are more tokens, there's likely a syntax error
                        if !input.is_empty() && !input.peek(Brace) && !input.peek(Token![;]) {
                            return Err(e);
                        }
                        break; // No more tokens to parse
                    }
                }
            }
        }

        // Parse children if present
        if input.peek(Brace) {
            has_children_block = true;
            let content;
            syn::braced!(content in input);

            while !content.is_empty() {
                children.push(content.parse::<KdlNode>()?);
            }
        }

        let _: Option<Token![;]> = input.parse()?;

        let kdl_node = KdlNode {
            name,
            type_annotation,
            properties,
            arguments,
            children,
            has_children_block,
        };

        Ok(kdl_node)
    }
}

// Helper function to parse node names with optional type annotations
// Supports: (type)name or name
fn parse_node_name_with_type_annotation(input: ParseStream) -> Result<(Option<String>, KdlString)> {
    // Check for type annotation: (type)name
    if input.peek(syn::token::Paren) {
        let type_annotation = parse_type_annotation(input)?;

        // Allow optional whitespace between type annotation and name
        // (This is handled automatically by syn's parsing)

        let name = input.parse()?;
        Ok((Some(type_annotation), name))
    } else {
        let name: KdlString = input.parse()?;
        Ok((None, name))
    }
}

// Helper function to determine if the input stream starts with a property
fn is_property_ahead(input: &syn::parse::ParseBuffer) -> bool {
    if (input.peek(LitStr) || input.peek(Ident)) && input.peek2(Token![=]) {
        return true;
    }

    false
}
