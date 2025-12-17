//! Value parsing
//!
//! This module handles parsing of KDL values including strings, numbers,
//! booleans, null, and type-annotated values.

use crate::ast::{KdlString, KdlValue};
use crate::parse::number::try_parse_number;
use crate::parse::type_annotation::parse_type_annotation;
use syn::spanned::Spanned;
use syn::Token;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Lit, LitStr, Result,
};

pub mod bare_identifiers {
    syn::custom_keyword!(inf);
    syn::custom_keyword!(nan);
    syn::custom_keyword!(null);
}

impl Parse for KdlValue {
    fn parse(input: ParseStream) -> Result<Self> {
        // Check for type annotation: (type)value
        if input.peek(syn::token::Paren) {
            let type_annotation = parse_type_annotation(input)?;

            // Allow optional whitespace between type annotation and value
            // (This is handled automatically by syn's parsing)

            let value = Box::new(input.parse::<KdlValue>()?);

            // Note: Type annotation validation is optional per KDL spec
            // We could validate here but choose to be permissive for now
            // validate_type_annotation(&type_annotation, &value)?;

            return Ok(KdlValue::TypeAnnotated {
                type_annotation,
                value,
            });
        }

        // Try to parse as a number first (handles all formats including negative numbers)
        if let Some(number_value) = try_parse_number(input)? {
            Ok(number_value)
        }
        // Check for string literals
        else if input.peek(LitStr) {
            let lit_str: LitStr = input.parse()?;
            let kdl_string = KdlString::from_lit_str_as_quoted(lit_str)?;
            // When parsing values, allow keywords since they should be treated as actual values
            kdl_string.validate_with_context(false)?;
            Ok(KdlValue::String(kdl_string))
        }
        // Check for boolean literals
        else if input.peek(syn::LitBool) {
            let boolean: syn::LitBool = input.parse()?;
            Ok(KdlValue::Boolean(boolean.value))
        }
        // Check for other literals (bool, str - numbers are handled above)
        else if input.peek(Lit) {
            let lit: Lit = input.parse()?;
            match lit {
                Lit::Bool(lit_bool) => Ok(KdlValue::Boolean(lit_bool.value)),
                Lit::Str(lit_str) => {
                    let kdl_string = KdlString::from_lit_str_as_quoted(lit_str)?;
                    // When parsing values, allow keywords since they should be treated as actual values
                    kdl_string.validate_with_context(false)?;
                    Ok(KdlValue::String(kdl_string))
                }
                Lit::Int(_) | Lit::Float(_) => {
                    // Numbers should have been handled by try_parse_number above
                    // If we reach here, it means our number parser missed something
                    Err(syn::Error::new(
                        lit.span(),
                        "Number parsing failed - this should not happen",
                    ))
                }
                _ => Err(syn::Error::new(lit.span(), "Unsupported literal type")),
            }
        }
        // Check for # syntax (like #true, #false, #inf, #-inf, #nan)
        else if input.peek(Token![#]) {
            let pound: Token![#] = input.parse()?;
            let span = pound.span();
            let next_span = input.span();
            // ensure that # has no space by comparing the spans
            if next_span.start() != span.end() {
                return Err(syn::Error::new(
                    span,
                    "No whitespace allowed between `#` and the following identifier",
                ));
            }
            if input.peek(bare_identifiers::inf) {
                let _inf: bare_identifiers::inf = input.parse()?;
                Ok(KdlValue::Float(f64::INFINITY))
            } else if input.peek(bare_identifiers::nan) {
                let _nan: bare_identifiers::nan = input.parse()?;
                Ok(KdlValue::Float(f64::NAN))
            } else if input.peek(Token![-]) && input.peek2(bare_identifiers::inf) {
                let _minus: Token![-] = input.parse()?;
                let _inf: bare_identifiers::inf = input.parse()?;
                Ok(KdlValue::Float(f64::NEG_INFINITY))
            } else if input.peek(bare_identifiers::null) {
                let _null: bare_identifiers::null = input.parse()?;
                Ok(KdlValue::Null)
            } else if input.peek(syn::LitBool) {
                // Handle #true and #false when true/false are literals, not identifiers
                let boolean: syn::LitBool = input.parse()?;
                Ok(KdlValue::Boolean(boolean.value))
            } else {
                Err(syn::Error::new(
                    input.span(),
                    "Expected one of `inf`, `-inf`, `nan`, `null`, `true`, or `false` after `#`",
                ))
            }
        }
        // Check for identifiers - treat as Rust variable references
        else if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            Ok(KdlValue::Variable(ident))
        } else {
            Err(syn::Error::new(
                input.span(),
                "Expected string, number, boolean, null, or identifier",
            ))
        }
    }
}
