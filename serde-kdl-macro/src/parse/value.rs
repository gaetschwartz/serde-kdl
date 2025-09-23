//! Value parsing
//!
//! This module handles parsing of KDL values including strings, numbers,
//! booleans, null, and type-annotated values.

use syn::{Result, Ident, Lit, LitStr, parse::{Parse, ParseStream}};
use crate::ast::{KdlValue, KdlString};
use crate::parse::type_annotation::parse_type_annotation;

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

        // Check for negative numbers first
        if input.peek(syn::token::Minus) {
            // Look ahead to see if this is a negative number or an identifier with dash
            let checkpoint = input.fork();
            let _minus: syn::token::Minus = checkpoint.parse()?;

            if checkpoint.peek(Lit) {
                // It's a negative number
                let _minus: syn::token::Minus = input.parse()?;
                let lit: Lit = input.parse()?;
                match lit {
                    Lit::Int(lit_int) => {
                        let value = -(lit_int.base10_parse::<i64>()?);
                        Ok(KdlValue::Integer(value))
                    }
                    Lit::Float(lit_float) => {
                        let value = -(lit_float.base10_parse::<f64>()?);
                        Ok(KdlValue::Float(value))
                    }
                    _ => Err(syn::Error::new(lit.span(), "Invalid negative literal")),
                }
            } else {
                // It's likely an identifier starting with minus, parse as a string identifier
                parse_identifier_sequence(input)
            }
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
        // Check for other literals (int, float, bool, str)
        else if input.peek(Lit) {
            let lit: Lit = input.parse()?;
            match lit {
                Lit::Int(lit_int) => {
                    let value = lit_int.base10_parse::<i64>()?;
                    Ok(KdlValue::Integer(value))
                }
                Lit::Float(lit_float) => {
                    let value = lit_float.base10_parse::<f64>()?;
                    Ok(KdlValue::Float(value))
                }
                Lit::Bool(lit_bool) => Ok(KdlValue::Boolean(lit_bool.value)),
                Lit::Str(lit_str) => {
                    let kdl_string = KdlString::from_lit_str_as_quoted(lit_str)?;
                    // When parsing values, allow keywords since they should be treated as actual values
            kdl_string.validate_with_context(false)?;
                    Ok(KdlValue::String(kdl_string))
                }
                _ => Err(syn::Error::new(lit.span(), "Unsupported literal type")),
            }
        }
        // Check for # syntax (like #true, #false)
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
                } else {
                    Err(syn::Error::new(
                        input.span(),
                        format!("Invalid # syntax: #{}. Only #true, #false, and #null are allowed", ident_str),
                    ))
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
            } else if identifier == "inf" {
                Ok(KdlValue::Float(f64::INFINITY))
            } else if identifier == "nan" {
                Ok(KdlValue::Float(f64::NAN))
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
                    span
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

// Helper function to parse identifier sequences like "ubuntu-latest"
fn parse_identifier_sequence(input: ParseStream) -> Result<KdlValue> {
    let mut parts = String::new();

    // Handle first token (could be minus or identifier)
    if input.peek(syn::token::Minus) {
        let _minus: syn::token::Minus = input.parse()?;
        parts.push('-');
    }

    // Parse the rest of the sequence
    while input.peek(Ident) {
        let ident: Ident = input.parse()?;
        parts.push_str(&ident.to_string());

        // Check for more dash-identifier pairs
        if input.peek(syn::token::Minus) && input.peek2(Ident) {
            let _minus: syn::token::Minus = input.parse()?;
            parts.push('-');
        } else {
            break;
        }
    }

    if parts.is_empty() {
        return Err(syn::Error::new(input.span(), "Expected identifier"));
    }

    // Check for keywords even in identifier sequences
    if parts == "null" {
        Ok(KdlValue::Null)
    } else if parts == "true" {
        Ok(KdlValue::Boolean(true))
    } else if parts == "false" {
        Ok(KdlValue::Boolean(false))
    } else {
        let span = input.span();
        let kdl_string = KdlString::Identifier {
            value: parts,
            span
        };
        // When parsing values, allow keywords since they should be treated as actual values
            kdl_string.validate_with_context(false)?;
        Ok(KdlValue::String(kdl_string))
    }
}
