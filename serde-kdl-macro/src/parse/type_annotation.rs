//! Type annotation parsing
//!
//! This module handles parsing of KDL type annotations according to Section 3.8
//! of the KDL specification.

use syn::{parse::ParseStream, Ident, Result};

/// Helper function to parse type annotations with whitespace support
/// Supports: (type), ( type ), (multi-word-type), etc.
pub(crate) fn parse_type_annotation(input: ParseStream) -> Result<String> {
    let content;
    syn::parenthesized!(content in input);

    // Parse the type annotation as a sequence of tokens to handle complex types
    let mut type_parts = Vec::new();

    // Parse all tokens in sequence, handling identifiers, keywords, numbers, and dashes
    while !content.is_empty() {
        if content.peek(Ident) {
            let ident: Ident = content.parse()?;
            type_parts.push(ident.to_string());
        } else if content.peek(syn::Token![type]) {
            // Handle the 'type' keyword specifically
            let _: syn::Token![type] = content.parse()?;
            type_parts.push("type".to_string());
        } else if content.peek(syn::token::Minus) {
            let _minus: syn::token::Minus = content.parse()?;
            type_parts.push("-".to_string());
        } else if content.peek(syn::LitInt) {
            // Handle numeric literals like "2" in "country-2"
            let lit_int: syn::LitInt = content.parse()?;
            type_parts.push(lit_int.base10_digits().to_string());
        } else {
            return Err(syn::Error::new(
                content.span(),
                "Expected identifier, 'type' keyword, number, or '-' in type annotation",
            ));
        }
    }

    if type_parts.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Type annotation cannot be empty",
        ));
    }

    // Validate that the structure is valid (identifier followed by optional dash-identifier pairs)
    // We allow any sequence of identifier-dash-identifier-dash-... or just identifier
    let result = type_parts.join("");

    // Basic validation: ensure it doesn't start or end with dash, and doesn't have consecutive dashes
    if result.starts_with('-') || result.ends_with('-') || result.contains("--") {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Invalid type annotation format",
        ));
    }

    Ok(result)
}
