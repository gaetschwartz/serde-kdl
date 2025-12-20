//! Value parsing
//!
//! This module handles parsing of KDL values including strings, numbers,
//! booleans, null, and type-annotated values.

use crate::{ast::SERDE_KDL_KDL_EXPORT, parse::identifier::KdlIdentifier};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use std::ops::RangeBounds;
use syn::{
    Ident, Result, Token,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

pub mod bare_identifiers {
    syn::custom_keyword!(inf);
    syn::custom_keyword!(nan);
    syn::custom_keyword!(null);
}

/// Represents a KDL value (string, number, boolean, etc.)
#[derive(Clone, PartialEq)]
pub enum KdlValue {
    String(KdlIdentifier),
    Variable(syn::Ident),
    Lit(KdlLit),
}

impl Parse for KdlValue {
    fn parse(input: ParseStream) -> Result<Self> {
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

impl ToTokens for KdlValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            KdlValue::String(kdl_string) => {
                let s = kdl_string.value();
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::String(#s.to_string()) }
            }
            KdlValue::Lit(KdlLit::Integer(i, _)) => {
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Integer(#i) }
            }
            KdlValue::Lit(KdlLit::Float(f, _)) => {
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Float(#f) }
            }
            KdlValue::Lit(KdlLit::Boolean(b, _)) => {
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Bool(#b) }
            }
            KdlValue::Lit(KdlLit::Null(_)) => quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Null },
            KdlValue::Lit(KdlLit::Nan(_)) => {
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::NAN) }
            }
            KdlValue::Lit(KdlLit::Infinity(_)) => {
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::INFINITY) }
            }
            KdlValue::Lit(KdlLit::NegInfinity(_)) => {
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::NEG_INFINITY) }
            }
            KdlValue::Variable(ident) => {
                // Use KdlValue::from() - user's variable type must implement Into<KdlValue>
                // kdl::KdlValue implements From for: i128, f64, &str, String, bool, Option<T>
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::from(#ident) }
            }
        }
        .to_tokens(tokens);
    }
}

impl KdlValue {
    #[must_use]
    pub fn span(&self) -> proc_macro2::Span {
        match self {
            KdlValue::String(s) => s.span(),
            KdlValue::Variable(ident) => ident.span(),
            KdlValue::Lit(lit) => lit.span(),
        }
    }
}

impl From<(&str, proc_macro2::Span)> for KdlValue {
    fn from((s, span): (&str, proc_macro2::Span)) -> Self {
        KdlValue::String(KdlIdentifier::from((s, span)))
    }
}

impl From<(String, proc_macro2::Span)> for KdlValue {
    fn from((s, span): (String, proc_macro2::Span)) -> Self {
        KdlValue::String(KdlIdentifier::from((s, span)))
    }
}

impl std::fmt::Debug for KdlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KdlValue::String(s) => write!(f, "String({})", s.value()),
            KdlValue::Variable(ident) => write!(f, "Variable({ident})"),
            KdlValue::Lit(lit) => write!(f, "Lit({lit:?})"),
        }
    }
}

#[derive(Clone)]
pub enum KdlLit {
    Integer(i128, Span),
    Float(f64, Span),
    Boolean(bool, Span),
    Nan(bare_identifiers::nan),
    Infinity(bare_identifiers::inf),
    NegInfinity(bare_identifiers::inf),
    Null(bare_identifiers::null),
}

impl std::fmt::Debug for KdlLit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KdlLit::Integer(value, _) => write!(f, "Integer({value})"),
            KdlLit::Float(value, _) => write!(f, "Float({value})"),
            KdlLit::Boolean(value, _) => write!(f, "Boolean({value})"),
            KdlLit::Nan(_) => write!(f, "Nan"),
            KdlLit::Infinity(_) => write!(f, "Infinity"),
            KdlLit::NegInfinity(_) => write!(f, "NegInfinity"),
            KdlLit::Null(_) => write!(f, "Null"),
        }
    }
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
            span,
        ))
    }
    fn parse_number(input: ParseStream) -> Result<Self> {
        if input.peek(syn::LitInt) {
            let lit_int: syn::LitInt = input.parse()?;
            let repr = lit_int.to_string();

            match repr.as_bytes() {
                [b'0', b'b' | b'B', ..] => Self::from_radix(repr, 2, 2.., lit_int.span()),
                [b'0', b'o' | b'O', ..] => Self::from_radix(repr, 8, 2.., lit_int.span()),
                [b'0', b'x' | b'X', ..] => Self::from_radix(repr, 16, 2.., lit_int.span()),
                _ => Ok(Self::Integer(lit_int.base10_parse()?, lit_int.span())),
            }
        } else if input.peek(syn::LitFloat) {
            let lit_float: syn::LitFloat = input.parse()?;
            Ok(Self::Float(lit_float.base10_parse()?, lit_float.span()))
        } else {
            Err(syn::Error::new(input.span(), "Expected a numeric literal"))
        }
    }

    #[must_use]
    pub fn span(&self) -> Span {
        match self {
            KdlLit::Integer(_, span) => *span,
            KdlLit::Float(_, span) => *span,
            KdlLit::Boolean(_, span) => *span,
            KdlLit::Nan(nan) => nan.span(),
            KdlLit::Infinity(inf) => inf.span(),
            KdlLit::NegInfinity(inf) => inf.span(),
            KdlLit::Null(null) => null.span(),
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
                Ok(Self::Boolean(boolean.value, boolean.span()))
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
            (KdlLit::Integer(a, _), KdlLit::Integer(b, _)) => a == b,
            (KdlLit::Float(a, _), KdlLit::Float(b, _)) => a == b,
            (KdlLit::Boolean(a, _), KdlLit::Boolean(b, _)) => a == b,
            (KdlLit::Nan(_), KdlLit::Nan(_)) => true,
            (KdlLit::Infinity(_), KdlLit::Infinity(_)) => true,
            (KdlLit::NegInfinity(_), KdlLit::NegInfinity(_)) => true,
            (KdlLit::Null(_), KdlLit::Null(_)) => true,
            _ => false,
        }
    }
}

impl PartialEq<kdl::KdlValue> for KdlValue {
    fn eq(&self, other: &kdl::KdlValue) -> bool {
        match (self, other) {
            (KdlValue::String(a), kdl::KdlValue::String(b)) => &*a.value() == b,
            (KdlValue::Lit(KdlLit::Integer(a, _)), kdl::KdlValue::Integer(b)) => a == b,
            (KdlValue::Lit(KdlLit::Float(a, _)), kdl::KdlValue::Float(b)) => a == b,
            (KdlValue::Lit(KdlLit::Boolean(a, _)), kdl::KdlValue::Bool(b)) => a == b,
            (KdlValue::Lit(KdlLit::Null(_)), kdl::KdlValue::Null) => true,
            (KdlValue::Lit(KdlLit::Nan(_)), kdl::KdlValue::Float(b)) => b.is_nan(),
            (KdlValue::Lit(KdlLit::Infinity(_)), kdl::KdlValue::Float(b)) => {
                b.is_infinite() && b.is_sign_positive()
            }
            (KdlValue::Lit(KdlLit::NegInfinity(_)), kdl::KdlValue::Float(b)) => {
                b.is_infinite() && b.is_sign_negative()
            }
            _ => false,
        }
    }
}
