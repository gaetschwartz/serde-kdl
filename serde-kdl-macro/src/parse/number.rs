//! Number parsing for KDL Section 3.14
//!
//! This module implements comprehensive number parsing for all KDL number formats:
//! - Keywords: #inf, #-inf, #nan
//! - Decimal: 123, 123.45, 1.23e4, 1.23E-5, +123, -456
//! - Hexadecimal: 0x1F, 0XFF, +0x123, -0xABC, 0x12_34
//! - Octal: 0o17, 0O77, +0o123, -0o456, 0o12_34
//! - Binary: 0b101, 0B111, +0b101, -0b110, 0b10_11

use crate::ast::KdlValue;
use proc_macro2::Span;
use syn::{
    parse::{discouraged::Speculative, ParseStream},
    Error, Result,
};

/// Parse a number from the input stream
///
/// This function handles all KDL number formats as specified in Section 3.14:
/// - Keyword numbers (#inf, #-inf, #nan)
/// - Decimal numbers with optional sign, decimal point, and exponent
/// - Hexadecimal numbers (0x/0X prefix)
/// - Octal numbers (0o/0O prefix)
/// - Binary numbers (0b/0B prefix)
pub(crate) fn parse_number(input: ParseStream) -> Result<KdlValue> {
    // Check for keyword numbers first (#inf, #-inf, #nan)
    if input.peek(syn::token::Pound) {
        return parse_keyword_number(input);
    }

    // Parse optional sign
    let is_negative = if input.peek(syn::token::Minus) {
        let _: syn::token::Minus = input.parse()?;
        true
    } else if input.peek(syn::token::Plus) {
        let _: syn::token::Plus = input.parse()?;
        false
    } else {
        false
    };

    // Try to parse with syn's built-in literal parsing first
    if input.peek(syn::Lit) {
        let lit: syn::Lit = input.parse()?;
        match lit {
            syn::Lit::Int(lit_int) => {
                // Check if this is a special radix number by examining the string representation
                let lit_str = lit_int.to_string();
                if lit_str.starts_with("0x") || lit_str.starts_with("0X") {
                    parse_hexadecimal(&lit_str, is_negative, lit_int.span())
                } else if lit_str.starts_with("0o") || lit_str.starts_with("0O") {
                    parse_octal(&lit_str, is_negative, lit_int.span())
                } else if lit_str.starts_with("0b") || lit_str.starts_with("0B") {
                    parse_binary(&lit_str, is_negative, lit_int.span())
                } else {
                    // Regular decimal integer
                    match lit_int.base10_parse::<i128>() {
                        Ok(mut value) => {
                            if is_negative {
                                value = -value;
                            }
                            Ok(KdlValue::Integer(value))
                        }
                        Err(_) => {
                            // Try as float if too large for i64
                            match lit_int.base10_parse::<f64>() {
                                Ok(mut value) => {
                                    if is_negative {
                                        value = -value;
                                    }
                                    Ok(KdlValue::Float(value))
                                }
                                Err(e) => Err(Error::new(
                                    lit_int.span(),
                                    format!("Invalid integer: {}", e),
                                )),
                            }
                        }
                    }
                }
            }
            syn::Lit::Float(lit_float) => match lit_float.base10_parse::<f64>() {
                Ok(mut value) => {
                    if is_negative {
                        value = -value;
                    }
                    Ok(KdlValue::Float(value))
                }
                Err(e) => Err(Error::new(
                    lit_float.span(),
                    format!("Invalid float: {}", e),
                )),
            },
            _ => Err(Error::new(lit.span(), "Expected number literal")),
        }
    } else {
        Err(Error::new(input.span(), "Expected number"))
    }
}

/// Parse keyword numbers: #inf, #-inf, #nan
fn parse_keyword_number(input: ParseStream) -> Result<KdlValue> {
    let _: syn::token::Pound = input.parse()?;

    // Check for negative infinity
    if input.peek(syn::token::Minus) {
        let _: syn::token::Minus = input.parse()?;
        let ident: syn::Ident = input.parse()?;
        if ident == "inf" {
            Ok(KdlValue::Float(f64::NEG_INFINITY))
        } else {
            Err(Error::new(ident.span(), "Expected 'inf' after '#-'"))
        }
    } else {
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
            "inf" => Ok(KdlValue::Float(f64::INFINITY)),
            "nan" => Ok(KdlValue::Float(f64::NAN)),
            _ => Err(Error::new(
                ident.span(),
                format!(
                    "Invalid keyword number: #{}. Only #inf, #-inf, and #nan are supported",
                    ident
                ),
            )),
        }
    }
}

/// Parse hexadecimal numbers (0x/0X prefix)
fn parse_hexadecimal(number_str: &str, is_negative: bool, span: Span) -> Result<KdlValue> {
    let hex_part = &number_str[2..]; // Remove 0x/0X prefix
    let clean_hex = hex_part.replace('_', "");

    // Note: Rust's literal parsing already validates hex digits
    match i128::from_str_radix(&clean_hex, 16) {
        Ok(mut value) => {
            if is_negative {
                value = -value;
            }
            Ok(KdlValue::Integer(value))
        }
        Err(_) => {
            // Try as float if too large for i128
            match u128::from_str_radix(&clean_hex, 16) {
                Ok(value) => {
                    let mut float_value = value as f64;
                    if is_negative {
                        float_value = -float_value;
                    }
                    Ok(KdlValue::Float(float_value))
                }
                Err(e) => Err(Error::new(
                    span,
                    format!("Invalid hexadecimal number: {}", e),
                )),
            }
        }
    }
}

/// Parse octal numbers (0o/0O prefix)
fn parse_octal(number_str: &str, is_negative: bool, span: Span) -> Result<KdlValue> {
    let octal_part = &number_str[2..]; // Remove 0o/0O prefix
    let clean_octal = octal_part.replace('_', "");

    // Note: Rust's literal parsing already validates octal digits
    match i128::from_str_radix(&clean_octal, 8) {
        Ok(mut value) => {
            if is_negative {
                value = -value;
            }
            Ok(KdlValue::Integer(value))
        }
        Err(_) => {
            // Try as float if too large for i128
            match u128::from_str_radix(&clean_octal, 8) {
                Ok(value) => {
                    let mut float_value = value as f64;
                    if is_negative {
                        float_value = -float_value;
                    }
                    Ok(KdlValue::Float(float_value))
                }
                Err(e) => Err(Error::new(
                    span,
                    format!("Invalid octal number: {}", e),
                )),
            }
        }
    }
}

/// Parse binary numbers (0b/0B prefix)
fn parse_binary(number_str: &str, is_negative: bool, span: Span) -> Result<KdlValue> {
    let binary_part = &number_str[2..]; // Remove 0b/0B prefix
    let clean_binary = binary_part.replace('_', "");

    // Note: Rust's literal parsing already validates binary digits
    match i128::from_str_radix(&clean_binary, 2) {
        Ok(mut value) => {
            if is_negative {
                value = -value;
            }
            Ok(KdlValue::Integer(value))
        }
        Err(_) => {
            // Try as float if too large for i128
            match u128::from_str_radix(&clean_binary, 2) {
                Ok(value) => {
                    let mut float_value = value as f64;
                    if is_negative {
                        float_value = -float_value;
                    }
                    Ok(KdlValue::Float(float_value))
                }
                Err(e) => Err(Error::new(
                    span,
                    format!("Invalid binary number: {}", e),
                )),
            }
        }
    }
}

/// Try to parse a number-like token from the input
/// This is a helper function that can be used by the value parser
pub(crate) fn try_parse_number(input: ParseStream) -> Result<Option<KdlValue>> {
    // Save position to backtrack if this isn't a number
    let fork = input.fork();

    match parse_number(&fork) {
        Ok(value) => {
            // Advance the main stream
            input.advance_to(&fork);
            Ok(Some(value))
        }
        Err(_) => Ok(None), // Not a number, let other parsers handle it
    }
}
