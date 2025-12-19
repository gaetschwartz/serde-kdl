//! Value parsing
//!
//! This module handles parsing of KDL values including strings, numbers,
//! booleans, null, and type-annotated values.

use std::ops::RangeBounds;

use crate::ast::{KdlIdentifier, KdlValue};
use crate::parse::type_annotation::parse_type_annotation;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::Token;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Result,
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
        // Check for other literals (bool, str - numbers are handled above)
        if input.peek(syn::LitStr) {
            let lit_str: syn::LitStr = input.parse()?;
            return Ok(KdlValue::String(KdlIdentifier::from(lit_str)));
        }
        // Check for identifiers - treat as Rust variable references
        if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            return Ok(KdlValue::Variable(ident));
        }

        match input.parse::<KdlLit>() {
            Ok(lit) => Ok(KdlValue::Lit(lit)),
            Err(e) => Err(syn::Error::new(
                e.span(),
                format!("Expected a KDL value (string, number, boolean, null, or variable). ({e})"),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum KdlLit {
    Integer(i128),
    Float(f64),
    Boolean(bool),
    Nan(bare_identifiers::nan),
    Infinity(bare_identifiers::inf),
    NegInfinity(bare_identifiers::inf),
    Null(bare_identifiers::null),
}

impl KdlLit {
    fn from_radix(
        mut s: String,
        radix: u32,
        range: impl RangeBounds<usize>,
        span: Span,
    ) -> Result<Self> {
        // remove all underscores from s
        s.retain(|c| c != '_');
        let range = (range.start_bound().cloned(), range.end_bound().cloned());
        Ok(Self::Integer(
            i128::from_str_radix(&s[range], radix).map_err(|e| {
                syn::Error::new(span, format!("Failed to parse integer literal '{s}': {e}"))
            })?,
        ))
    }
    fn parse_number(input: ParseStream) -> Result<Self> {
        if input.peek(syn::LitInt) {
            let lit_int: syn::LitInt = input.parse()?;
            let repr = lit_int.to_string();
            let out = match repr.as_bytes() {
                [b'0', b'b' | b'B', ..] => Self::from_radix(repr, 2, 2.., lit_int.span()),
                [b'0', b'o' | b'O', ..] => Self::from_radix(repr, 8, 2.., lit_int.span()),
                [b'0', b'x' | b'X', ..] => Self::from_radix(repr, 16, 2.., lit_int.span()),
                _ => Ok(Self::Integer(lit_int.base10_parse()?)),
            };
            out
        } else if input.peek(syn::LitFloat) {
            let lit_float: syn::LitFloat = input.parse()?;
            Ok(Self::Float(lit_float.base10_parse()?))
        } else {
            Err(syn::Error::new(input.span(), "Expected a numeric literal"))
        }
    }
}

impl syn::parse::Parse for KdlLit {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![#]) {
            let pound: Token![#] = input.parse()?;
            let pound_span = pound.span();
            let next_span = input.span();
            // ensure that # has no space by comparing the spans
            return if next_span.start() != pound_span.end() {
                Err(syn::Error::new(
                    pound_span,
                    "No whitespace allowed between `#` and the following identifier",
                ))
            } else if input.peek(bare_identifiers::inf) {
                let inf: bare_identifiers::inf = input.parse()?;
                Ok(Self::Infinity(inf))
            } else if input.peek(bare_identifiers::nan) {
                let nan: bare_identifiers::nan = input.parse()?;
                Ok(Self::Nan(nan))
            } else if input.peek(Token![-]) && input.peek2(bare_identifiers::inf) {
                let _minus: Token![-] = input.parse()?;
                let inf: bare_identifiers::inf = input.parse()?;
                Ok(Self::NegInfinity(inf))
            } else if input.peek(bare_identifiers::null) {
                let null: bare_identifiers::null = input.parse()?;
                Ok(Self::Null(null))
            } else if input.peek(syn::LitBool) {
                // Handle #true and #false when true/false are literals, not identifiers
                let boolean: syn::LitBool = input.parse()?;
                Ok(Self::Boolean(boolean.value))
            } else {
                Err(syn::Error::new(
                    input.span(),
                    "Expected one of `inf`, `-inf`, `nan`, `null`, `true`, or `false` after `#`",
                ))
            };
        }

        if input.peek(Token![+]) {
            let _plus: Token![+] = input.parse()?;
            return Self::parse_number(input);
        }

        Self::parse_number(input)
    }
}

impl PartialEq for KdlLit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (KdlLit::Integer(a), KdlLit::Integer(b)) => a == b,
            (KdlLit::Float(a), KdlLit::Float(b)) => a == b,
            (KdlLit::Boolean(a), KdlLit::Boolean(b)) => a == b,
            (KdlLit::Nan(_), KdlLit::Nan(_)) => true,
            (KdlLit::Infinity(_), KdlLit::Infinity(_)) => true,
            (KdlLit::NegInfinity(_), KdlLit::NegInfinity(_)) => true,
            (KdlLit::Null(_), KdlLit::Null(_)) => true,
            _ => false,
        }
    }
}
