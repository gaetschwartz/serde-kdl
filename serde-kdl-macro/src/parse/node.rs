//! Node parsing
//!
//! This module handles parsing of KDL nodes including node names,
//! properties, arguments, and children.

use syn::{Result, Ident, LitStr, token::{Brace, Eq}, Token, parse::{Parse, ParseStream}};
use crate::ast::{KdlNode, KdlProperty, KdlValue, KdlString};
use crate::parse::type_annotation::parse_type_annotation;

impl Parse for KdlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse optional type annotation for node name
        let (type_annotation, name) = parse_node_name_with_type_annotation(input)?;
        let mut properties = Vec::new();
        let mut arguments = Vec::new();
        let mut children = Vec::new();
        let mut has_children_block = false;

        // Parse arguments and properties
        while !input.is_empty() && !input.peek(Brace) && !input.peek(Token![;]) {
            // Look ahead to see if this is a property (identifier followed by =)
            let checkpoint = input.fork();

            // Try to parse as property first (string key followed by =)
            // We need more sophisticated lookahead for hyphenated identifiers
            if is_property_ahead(&checkpoint) {
                // Parse property key (must be a String value according to Section 3.7)
                let key = parse_property_key(input)?;
                let _eq: Eq = input.parse()?;
                let value: KdlValue = input.parse()?;

                // Validate that the value is a valid KDL value according to Section 3.7
                if !value.is_valid_value() {
                    return Err(syn::Error::new(
                        input.span(),
                        "Property values must be String, Number, Boolean, or Null (Section 3.7)",
                    ));
                }

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
                            let ident: Ident = checkpoint.parse().unwrap();
                            let ident_str = ident.to_string();

                            // Keywords are not valid node names, so this is a type-annotated value, not a new node
                            if ident_str == "null" || ident_str == "true" || ident_str == "false" ||
                               ident_str == "inf" || ident_str == "-inf" || ident_str == "nan" {
                                // This is a type-annotated keyword value, continue parsing as an argument
                            } else {
                                // This looks like a legitimate new node
                                break;
                            }
                        }
                        // If it's followed by a literal, it's an argument type annotation
                        // Continue parsing as normal
                    }
                } else if input.peek(Ident) {
                    // Look ahead to see if this identifier is followed by something that would
                    // make it clearly an argument vs a new node
                    let checkpoint = input.fork();
                    let ident: Ident = checkpoint.parse().unwrap();
                    let ident_str = ident.to_string();

                    // Don't treat KDL keywords as new nodes - they're values
                    if ident_str == "null" || ident_str == "true" || ident_str == "false" {
                        // This is a keyword value, continue parsing as an argument
                    } else {
                        // Be more aggressive: most identifiers at top level are new nodes
                        // Only continue if it's clearly NOT a new node (e.g., a single identifier at the end)
                        if checkpoint.is_empty() {
                            // Single identifier with nothing after it might be an argument
                            // But let's be conservative and treat it as a new node
                            break;
                        } else {
                            // If there's anything after the identifier, it's probably a new node
                            break;
                        }
                    }
                }

                // Try to parse as argument (literal value or identifier)
                match input.parse::<KdlValue>() {
                    Ok(value) => {
                        // Validate that the value is a valid KDL value according to Section 3.7
                        if !value.is_valid_value() {
                            return Err(syn::Error::new(
                                input.span(),
                                "Arguments must be String, Number, Boolean, or Null (Section 3.7)",
                            ));
                        }
                        arguments.push(value);
                    },
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

        Ok(KdlNode {
            name,
            type_annotation,
            properties,
            arguments,
            children,
            has_children_block,
        })
    }
}

// Helper function to parse node names with optional type annotations
// Supports: (type)name or name
fn parse_node_name_with_type_annotation(input: ParseStream) -> Result<(Option<String>, String)> {
    // Check for type annotation: (type)name
    if input.peek(syn::token::Paren) {
        let type_annotation = parse_type_annotation(input)?;

        // Allow optional whitespace between type annotation and name
        // (This is handled automatically by syn's parsing)

        let name = parse_bare_node_name(input)?;
        Ok((Some(type_annotation), name))
    } else {
        let name = parse_bare_node_name(input)?;
        Ok((None, name))
    }
}

// Helper function to parse node names that might have dashes or be string literals
// According to Section 3.7, node names must be String values
fn parse_bare_node_name(input: ParseStream) -> Result<String> {
    // Check if it's a string literal first (quoted strings)
    if input.peek(LitStr) {
        let lit_str: LitStr = input.parse()?;
        let processed_value = crate::parse::string::process_string_escapes(&lit_str.value(), lit_str.span())?;

        // Create a KdlString for validation
        let kdl_string = KdlString::Quoted {
            value: processed_value.clone(),
            span: lit_str.span(),
        };
        kdl_string.validate()?;

        return Ok(processed_value);
    }

    // Otherwise parse as identifier sequence (bare strings)
    // Handle identifiers that might start with punctuation like -, +, .
    let mut name_parts = Vec::new();

    // Handle leading punctuation characters
    while input.peek(syn::token::Minus) || input.peek(syn::token::Plus) || input.peek(syn::token::Dot) {
        if input.peek(syn::token::Minus) {
            let _: syn::token::Minus = input.parse()?;
            name_parts.push("-".to_string());
        } else if input.peek(syn::token::Plus) {
            let _: syn::token::Plus = input.parse()?;
            name_parts.push("+".to_string());
        } else if input.peek(syn::token::Dot) {
            let _: syn::token::Dot = input.parse()?;
            name_parts.push(".".to_string());
        }
    }

    // Parse the main identifier part
    if input.peek(Ident) {
        let first_part: Ident = input.parse()?;
        name_parts.push(first_part.to_string());

        // Check for dash-separated parts like "runs-on"
        while input.peek(syn::token::Minus) && input.peek2(Ident) {
            let _minus: syn::token::Minus = input.parse()?;
            let next_part: Ident = input.parse()?;
            name_parts.push("-".to_string());
            name_parts.push(next_part.to_string());
        }
    } else if name_parts.is_empty() {
        return Err(syn::Error::new(input.span(), "Expected identifier"));
    }

    let full_name = name_parts.join("");

    // Create a KdlString for validation
    let kdl_string = KdlString::Identifier {
        value: full_name.clone(),
        span: input.span(),
    };
    kdl_string.validate()?;

    Ok(full_name)
}

// Helper function to parse property keys (must be String values according to Section 3.7)
fn parse_property_key(input: ParseStream) -> Result<String> {
    // Check if it's a string literal first (quoted strings)
    if input.peek(LitStr) {
        let lit_str: LitStr = input.parse()?;
        let processed_value = crate::parse::string::process_string_escapes(&lit_str.value(), lit_str.span())?;

        // Create a KdlString for validation
        let kdl_string = KdlString::Quoted {
            value: processed_value.clone(),
            span: lit_str.span(),
        };
        kdl_string.validate()?;

        return Ok(processed_value);
    }

    // Otherwise parse as identifier sequence (bare strings)
    // Handle identifiers that might start with punctuation like -, +, .
    let mut name_parts = Vec::new();

    // Handle leading punctuation characters
    while input.peek(syn::token::Minus) || input.peek(syn::token::Plus) || input.peek(syn::token::Dot) {
        if input.peek(syn::token::Minus) {
            let _: syn::token::Minus = input.parse()?;
            name_parts.push("-".to_string());
        } else if input.peek(syn::token::Plus) {
            let _: syn::token::Plus = input.parse()?;
            name_parts.push("+".to_string());
        } else if input.peek(syn::token::Dot) {
            let _: syn::token::Dot = input.parse()?;
            name_parts.push(".".to_string());
        }
    }

    // Parse the main identifier part
    if input.peek(Ident) {
        let first_part: Ident = input.parse()?;
        name_parts.push(first_part.to_string());

        // Check for dash-separated parts like "runs-on"
        while input.peek(syn::token::Minus) && input.peek2(Ident) {
            let _minus: syn::token::Minus = input.parse()?;
            let next_part: Ident = input.parse()?;
            name_parts.push("-".to_string());
            name_parts.push(next_part.to_string());
        }
    } else if name_parts.is_empty() {
        return Err(syn::Error::new(input.span(), "Expected identifier"));
    }

    let full_key = name_parts.join("");

    // Create a KdlString for validation
    let kdl_string = KdlString::Identifier {
        value: full_key.clone(),
        span: input.span(),
    };
    kdl_string.validate()?;

    Ok(full_key)
}

// Helper function to determine if the input stream starts with a property
fn is_property_ahead(input: &syn::parse::ParseBuffer) -> bool {
    if input.peek(LitStr) && input.peek2(Eq) {
        // String literal followed by = is definitely a property
        return true;
    }

    // Try to parse an identifier sequence (which may start with punctuation)
    let checkpoint = input.fork();

    // Handle leading punctuation characters
    while checkpoint.peek(syn::token::Minus) || checkpoint.peek(syn::token::Plus) || checkpoint.peek(syn::token::Dot) {
        if checkpoint.peek(syn::token::Minus) {
            if checkpoint.parse::<syn::token::Minus>().is_err() { break; }
        } else if checkpoint.peek(syn::token::Plus) {
            if checkpoint.parse::<syn::token::Plus>().is_err() { break; }
        } else if checkpoint.peek(syn::token::Dot)
            && checkpoint.parse::<syn::token::Dot>().is_err() { break;
        }
    }

    // Try to parse the main identifier part
    if checkpoint.peek(Ident) {
        if checkpoint.parse::<Ident>().is_ok() {
            // Parse potential dash-separated parts
            while checkpoint.peek(syn::token::Minus) && checkpoint.peek2(Ident) {
                if checkpoint.parse::<syn::token::Minus>().is_err() {
                    break;
                }
                if checkpoint.parse::<Ident>().is_err() {
                    break;
                }
            }

            // Check if followed by =
            return checkpoint.peek(Eq);
        }
    } else {
        // If we have some punctuation but no identifier following, check if = follows
        return checkpoint.peek(Eq);
    }

    false
}
