//! Value parsing
//!
//! This module handles parsing of KDL values including strings, numbers,
//! booleans, null, and type-annotated values.

use crate::ast::{KdlString, KdlValue};
use crate::parse::number::try_parse_number;
use crate::parse::type_annotation::parse_type_annotation;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Lit, LitStr, Result,
};

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
        else if input.peek(syn::token::Pound) {
            let _pound: syn::token::Pound = input.parse()?;
            if input.peek(Ident) {
                let ident: Ident = input.parse()?;
                let ident_str = ident.to_string();
                if ident_str == "true" {
                    Ok(KdlValue::Boolean(true))
                } else if ident_str == "false" {
                    Ok(KdlValue::Boolean(false))
                } else if ident_str == "null" {
                    Ok(KdlValue::Null)
                } else if ident_str == "inf" {
                    Ok(KdlValue::Float(f64::INFINITY))
                } else if ident_str == "nan" {
                    Ok(KdlValue::Float(f64::NAN))
                } else {
                    Err(syn::Error::new(
                        input.span(),
                        format!("Invalid # syntax: #{}. Only #true, #false, #null, #inf, #-inf, and #nan are allowed", ident_str),
                    ))
                }
            } else if input.peek(syn::token::Minus) {
                // Handle #-inf
                let _minus: syn::token::Minus = input.parse()?;
                let ident: Ident = input.parse()?;
                if ident == "inf" {
                    Ok(KdlValue::Float(f64::NEG_INFINITY))
                } else {
                    Err(syn::Error::new(ident.span(), "Expected 'inf' after '#-'"))
                }
            } else if input.peek(syn::LitBool) {
                // Handle #true and #false when true/false are literals, not identifiers
                let boolean: syn::LitBool = input.parse()?;
                Ok(KdlValue::Boolean(boolean.value))
            } else {
                Err(syn::Error::new(
                    input.span(),
                    "Expected identifier or literal after #",
                ))
            }
        }
        // Check for identifiers
        else if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            let identifier = ident.to_string();
            if identifier == "null" {
                Ok(KdlValue::Null)
            } else if identifier == "true" {
                Ok(KdlValue::Boolean(true))
            } else if identifier == "false" {
                Ok(KdlValue::Boolean(false))
            } else {
                // Check if there are more tokens that form a compound identifier
                let mut identifier_parts = vec![identifier];

                // Try to parse dash-separated identifiers like "ubuntu-latest"
                while input.peek(syn::token::Minus) && input.peek2(Ident) {
                    let _minus: syn::token::Minus = input.parse()?;
                    let next_part: Ident = input.parse()?;
                    identifier_parts.push("-".to_string());
                    identifier_parts.push(next_part.to_string());
                }

                let full_identifier = identifier_parts.join("");
                let span = input.span();
                let kdl_string = KdlString::Identifier {
                    value: full_identifier,
                    span,
                };
                // When parsing values, allow keywords since they should be treated as actual values
                kdl_string.validate_with_context(false)?;
                Ok(KdlValue::String(kdl_string))
            }
        } else {
            Err(syn::Error::new(
                input.span(),
                "Expected string, number, boolean, null, or identifier",
            ))
        }
    }
}
