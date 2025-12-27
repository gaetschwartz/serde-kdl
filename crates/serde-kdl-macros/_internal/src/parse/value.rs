//! Value parsing
//!
//! This module handles parsing of KDL values including strings, numbers,
//! booleans, null, and type-annotated values.

#[cfg(kdl_macros_debug)]
use crate::utils::DisplaySpan;
use crate::{
    ast::SERDE_KDL_KDL_EXPORT,
    parse::identifier::KdlIdentifier,
    trace,
    utils::{DebugSynToken, DebugToken, HasSpan},
};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use std::ops::{Deref, RangeBounds};
use syn::{
    Ident, LitBool, Result, Token,
    parse::{Parse, ParseStream},
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
            KdlValue::Lit(KdlLit::Boolean(b)) => {
                let b = &**b;
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Bool(#b) }
            }
            KdlValue::Lit(KdlLit::Null(_)) => quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Null },
            KdlValue::Lit(KdlLit::Nan(_)) => {
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::NAN) }
            }
            KdlValue::Lit(KdlLit::Infinity(v)) => {
                if v.minus().is_some() {
                    quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::NEG_INFINITY) }
                } else {
                    quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::INFINITY) }
                }
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

impl HasSpan for KdlValue {
    fn span(&self) -> proc_macro2::Span {
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

#[derive(Debug, Clone)]
pub enum KdlLit {
    Integer(i128, Span),
    Float(f64, Span),
    Boolean(PoundLiteral<LitBool>),
    Nan(PoundLiteral<bare_identifiers::nan>),
    Infinity(PoundLiteral<MaybeMinus<bare_identifiers::inf>>),
    Null(PoundLiteral<bare_identifiers::null>),
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
}

impl HasSpan for KdlLit {
    fn span(&self) -> Span {
        match self {
            KdlLit::Integer(_, span) => *span,
            KdlLit::Float(_, span) => *span,
            KdlLit::Boolean(b) => HasSpan::span(b),
            KdlLit::Nan(nan) => HasSpan::span(nan),
            KdlLit::Infinity(inf) => HasSpan::span(inf),
            KdlLit::Null(null) => HasSpan::span(null),
        }
    }
}

impl syn::parse::Parse for KdlLit {
    fn parse(input: ParseStream) -> Result<Self> {
        trace!(
            name = "kdllit",
            tt = input,
            "Start at span {}",
            input.span().display(),
        );
        if input.peek(Token![#]) {
            let pound: Token![#] = input.parse()?;
            let pound_span = pound.span();
            trace!(
                name = "kdllit",
                "Detected pound literal at span {}",
                pound_span.display()
            );
            let next_span = input.span();
            // ensure that # has no space by comparing the spans
            let pound_lit = if next_span.start() != pound_span.end() {
                Err(syn::Error::new(
                    pound_span,
                    "No whitespace allowed between `#` and the following identifier",
                ))
            } else if MaybeMinus::<bare_identifiers::inf>::peek(input) {
                let value = input.parse::<MaybeMinus<bare_identifiers::inf>>()?;
                Ok(Self::Infinity(PoundLiteral::new(pound, value)))
            } else if input.peek(bare_identifiers::nan) {
                let nan: bare_identifiers::nan = input.parse()?;
                Ok(Self::Nan(PoundLiteral::new(pound, nan)))
            } else if input.peek(bare_identifiers::null) {
                let null: bare_identifiers::null = input.parse()?;
                Ok(Self::Null(PoundLiteral::new(pound, null)))
            } else if input.peek(syn::LitBool) {
                let boolean: syn::LitBool = input.parse()?;
                Ok(Self::Boolean(PoundLiteral::new(pound, boolean)))
            } else {
                Err(syn::Error::new(
                    input.span(),
                    "Expected one of `inf`, `-inf`, `nan`, `null`, `true`, or `false` after `#`",
                ))
            }?;
            trace!(
                name = "kdllit",
                value = ?pound_lit,
                tt = input,
                "Parsed pound literal"
            );
            return Ok(pound_lit);
        }

        if input.peek(Token![+]) {
            let _plus: Token![+] = input.parse()?;
        }

        let out = Self::parse_number(input)?;
        trace!(
            name = "kdllit",
            tt = input,
            "Parsed numeric literal at span {}: {:?}",
            out.span().display(),
            out,
        );
        Ok(out)
    }
}

impl PartialEq for KdlLit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (KdlLit::Integer(a, _), KdlLit::Integer(b, _)) => a == b,
            (KdlLit::Float(a, _), KdlLit::Float(b, _)) => a == b,
            (KdlLit::Boolean(a), KdlLit::Boolean(b)) => a.value().value == b.value().value,
            (KdlLit::Nan(_), KdlLit::Nan(_)) => true,
            (KdlLit::Infinity(a), KdlLit::Infinity(b)) => {
                a.minus().is_some() == b.minus().is_some()
            }
            (KdlLit::Null(_), KdlLit::Null(_)) => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct PoundLiteral<T>(pub syn::token::Pound, pub T);

impl<T> PoundLiteral<T> {
    pub fn new(pound: syn::token::Pound, value: T) -> Self {
        Self(pound, value)
    }

    pub fn pound(&self) -> &syn::token::Pound {
        &self.0
    }

    pub fn value(&self) -> &T {
        &self.1
    }
}

impl<T: syn::parse::Parse> syn::parse::Parse for PoundLiteral<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let pound: Token![#] = input.parse()?;
        let value: T = input.parse()?;
        Ok(PoundLiteral(pound, value))
    }
}

impl<T: syn::parse::Parse> PoundLiteral<T> {
    pub fn peek(input: ParseStream<'_>) -> bool {
        if !input.peek(Token![#]) {
            return false;
        }
        let fork = input.fork();
        let _pound: Token![#] = fork.parse().unwrap();
        _ = fork.parse::<Option<Token![-]>>().unwrap();
        let parsed = fork.parse::<T>();
        parsed.is_ok()
    }
}

impl<T: HasSpan> std::fmt::Debug for PoundLiteral<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PoundLiteral")
            .field(&DebugSynToken(&self.0))
            .field(&DebugToken(&self.1))
            .finish()
    }
}

impl<T> Deref for PoundLiteral<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<T: PartialEq> PartialEq for PoundLiteral<T> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<T: HasSpan> HasSpan for PoundLiteral<T> {
    fn span(&self) -> Span {
        self.1.span()
    }
}

#[derive(Clone)]
pub struct MaybeMinus<T>(pub Option<syn::token::Minus>, pub T);

impl<T> MaybeMinus<T> {
    pub fn minus(&self) -> Option<&syn::token::Minus> {
        self.0.as_ref()
    }

    pub fn value(&self) -> &T {
        &self.1
    }

    pub fn neg(minus: Token![-], value: T) -> Self {
        Self(Some(minus), value)
    }

    pub fn pos(value: T) -> Self {
        Self(None, value)
    }
}

impl<T: syn::parse::Parse> syn::parse::Parse for MaybeMinus<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let minus = input.parse::<Option<Token![-]>>()?;
        let value = input.parse::<T>()?;
        Ok(MaybeMinus(minus, value))
    }
}

impl MaybeMinus<bare_identifiers::inf> {
    pub fn peek(input: ParseStream<'_>) -> bool {
        input.peek(Token![-]) && input.peek2(bare_identifiers::inf)
            || input.peek(bare_identifiers::inf)
    }
}

impl<T: HasSpan> HasSpan for MaybeMinus<T> {
    fn span(&self) -> Span {
        self.1.span()
    }
}

impl Deref for MaybeMinus<bare_identifiers::inf> {
    type Target = bare_identifiers::inf;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<T: PartialEq> PartialEq for MaybeMinus<T> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1 && self.0.is_some() == other.0.is_some()
    }
}

impl PartialEq<kdl::KdlValue> for KdlValue {
    fn eq(&self, other: &kdl::KdlValue) -> bool {
        match (self, other) {
            (KdlValue::String(a), kdl::KdlValue::String(b)) => &*a.value() == b,
            (KdlValue::Lit(KdlLit::Integer(a, _)), kdl::KdlValue::Integer(b)) => a == b,
            (KdlValue::Lit(KdlLit::Float(a, _)), kdl::KdlValue::Float(b)) => a == b,
            (KdlValue::Lit(KdlLit::Boolean(a)), kdl::KdlValue::Bool(b)) => a.value().value == *b,
            (KdlValue::Lit(KdlLit::Null(_)), kdl::KdlValue::Null) => true,
            (KdlValue::Lit(KdlLit::Nan(_)), kdl::KdlValue::Float(b)) => b.is_nan(),
            (KdlValue::Lit(KdlLit::Infinity(v)), kdl::KdlValue::Float(b)) => {
                b.is_infinite() && (v.minus().is_some() == b.is_sign_negative())
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("#inf", |r| matches!(r, KdlLit::Infinity(v) if v.minus().is_none() ) )]
    #[case("#-inf", |r| matches!(r, KdlLit::Infinity(v) if v.minus().is_some() ) )]
    #[case("#nan", |r| matches!(r, KdlLit::Nan(_)) )]
    #[case("#null", |r| matches!(r, KdlLit::Null(_)) )]
    #[case("#true", |r| matches!(r, KdlLit::Boolean(b) if b.value().value ) )]
    #[case("#false", |r| matches!(r, KdlLit::Boolean(b) if !b.value().value ) )]
    fn test_parse_pound_literals(#[case] input: &str, #[case] expect: impl Fn(KdlLit) -> bool) {
        let parsed: KdlLit = syn::parse_str(input).expect("Failed to parse pound literal");
        eprintln!("Parsed: {:?}", parsed);
        assert!(expect(parsed), "Parsed value did not match expectation");
    }
}
