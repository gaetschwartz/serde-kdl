//! Abstract Syntax Tree (AST) definitions for KDL documents
//!
//! This module contains all the type definitions for representing KDL documents,
//! nodes, values, and related structures in memory.

use crate::parse::{type_annotation::MaybeAnnotated, value::KdlLit};
pub(crate) use kdl_string::KdlIdentifier;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned as _;

/// Reserved type annotations for numbers without decimals (Section 3.8.1)
#[allow(dead_code)]
pub(crate) const RESERVED_INTEGER_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "isize", "usize",
];

/// Reserved type annotations for numbers with decimals (Section 3.8.2)
#[allow(dead_code)]
pub(crate) const RESERVED_FLOAT_TYPES: &[&str] = &["f32", "f64", "decimal64", "decimal128"];

/// Reserved type annotations for strings (Section 3.8.3)
#[allow(dead_code)]
pub(crate) const RESERVED_STRING_TYPES: &[&str] = &[
    "date-time",
    "time",
    "date",
    "duration",
    "decimal",
    "currency",
    "country-2",
    "country-3",
    "country-subdivision",
    "email",
    "idn-email",
    "hostname",
    "idn-hostname",
    "ipv4",
    "ipv6",
    "url",
    "url-reference",
    "irl",
    "irl-reference",
    "url-template",
    "uuid",
    "regex",
    "base64",
];

pub(crate) const SERDE_KDL_KDL_EXPORT: ConstPath = ConstPath(&["kdl"]);
pub(crate) const KDL_NODE: ConstPath = ConstPath(&["kdl", "KdlNode"]);

/// Represents a complete KDL document containing multiple nodes
#[derive(Debug, Clone)]
pub(crate) struct KdlDocument {
    pub(crate) nodes: Vec<KdlNode>,
}

/// Represents a single KDL node with optional properties, arguments, and children
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct KdlNode {
    pub(crate) name: KdlIdentifier,
    pub(crate) type_annotation: Option<KdlIdentifier>, // Type annotation for node name
    pub(crate) properties: Vec<KdlProperty>,
    pub(crate) arguments: Vec<MaybeAnnotated<KdlValue>>,
    pub(crate) children: Vec<KdlNode>,
    pub(crate) has_children_block: bool,
    pub(crate) terminator: Terminator,
}

impl KdlNode {
    pub(crate) fn type_annotation(&self) -> Option<&KdlIdentifier> {
        self.type_annotation.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Terminator {
    Brace,
    Semicolon,
    Eol,
    Eof,
}

/// Represents a KDL property (key="value" pair)
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct KdlProperty {
    pub(crate) key: KdlIdentifier, // According to Section 3.7, property keys must be String values
    pub(crate) value: MaybeAnnotated<KdlValue>,
}

#[cfg(test)]
impl KdlProperty {
    /// Create a new `KdlProperty`
    pub(crate) fn new(key: impl Into<KdlIdentifier>, value: impl Into<KdlValue>) -> Self {
        KdlProperty {
            key: key.into(),
            value: MaybeAnnotated {
                type_annotation: None,
                item: value.into(),
            },
        }
    }
}

/// Represents a KDL value (string, number, boolean, etc.)
#[derive(Clone, PartialEq)]
pub(crate) enum KdlValue {
    String(KdlIdentifier),
    Variable(syn::Ident),
    Lit(KdlLit),
}

impl KdlValue {
    pub(crate) fn span(&self) -> proc_macro2::Span {
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

impl ToTokens for KdlValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            KdlValue::String(kdl_string) => {
                let s = kdl_string.value();
                quote_spanned! {kdl_string.span()=> #SERDE_KDL_KDL_EXPORT::KdlValue::String(#s.to_string()) }
            }
            KdlValue::Lit(KdlLit::Integer(i, span)) => {
                quote_spanned! {*span=> #SERDE_KDL_KDL_EXPORT::KdlValue::Integer(#i) }
            }
            KdlValue::Lit(KdlLit::Float(f, span)) => {
                quote_spanned! {*span=> #SERDE_KDL_KDL_EXPORT::KdlValue::Float(#f) }
            }
            KdlValue::Lit(KdlLit::Boolean(b, span)) => {
                quote_spanned! {*span=> #SERDE_KDL_KDL_EXPORT::KdlValue::Bool(#b) }
            }
            KdlValue::Lit(KdlLit::Null(n)) => quote_spanned! {n.span()=> #SERDE_KDL_KDL_EXPORT::KdlValue::Null },
            KdlValue::Lit(KdlLit::Nan(n)) => {
                quote_spanned! {n.span()=> #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::NAN) }
            }
            KdlValue::Lit(KdlLit::Infinity(n)) => {
                quote_spanned! {n.span()=> #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::INFINITY) }
            }
            KdlValue::Lit(KdlLit::NegInfinity(n)) => {
                quote_spanned! {n.span()=> #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::NEG_INFINITY) }
            }
            KdlValue::Variable(ident) => {
                // Use KdlValue::from() - user's variable type must implement Into<KdlValue>
                // kdl::KdlValue implements From for: i128, f64, &str, String, bool, Option<T>
                quote_spanned! {ident.span()=> #SERDE_KDL_KDL_EXPORT::KdlValue::from(#ident) }
            }
        }
        .to_tokens(tokens);
    }
}

mod kdl_string {
    use super::*;
    use crate::validation::{self, ValidationOptions};
    use quote::{quote_spanned, ToTokens};
    use std::borrow::Cow;

    /// Represents different types of KDL strings as per Section 3.9
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    pub enum KdlIdentifier {
        /// Identifier String (Section 3.10) - like `foo`
        Identifier { ident: syn::Ident },
        /// Quoted String (Section 3.11) - like `"foo"`
        Quoted {
            value: String,
            span: proc_macro2::Span,
        },
    }

    #[allow(dead_code)]
    impl KdlIdentifier {
        /// Get the string value regardless of the string type
        pub(crate) fn value(&self) -> Cow<'_, str> {
            match &self {
                KdlIdentifier::Identifier { ident } => ident.to_string().into(),
                KdlIdentifier::Quoted { value, .. } => value.into(),
            }
        }

        /// Get the span for error reporting
        pub(crate) fn span(&self) -> proc_macro2::Span {
            match &self {
                KdlIdentifier::Identifier { ident } => ident.span(),
                KdlIdentifier::Quoted { span, .. } => *span,
            }
        }

        /// Get the identifier if this is an Identifier variant
        pub(crate) fn as_ident(&self) -> Option<&syn::Ident> {
            match &self {
                KdlIdentifier::Identifier { ident } => Some(ident),
                KdlIdentifier::Quoted { .. } => None,
            }
        }

        /// Get the quoted string if this is a Quoted variant
        pub(crate) fn as_quoted(&self) -> Option<(&str, proc_macro2::Span)> {
            match &self {
                KdlIdentifier::Quoted { value, span } => Some((value, *span)),
                KdlIdentifier::Identifier { .. } => None,
            }
        }

        pub(crate) fn new_identifier(ident: syn::Ident) -> syn::Result<Self> {
            validation::validate_identifier(&ident, ValidationOptions::default())?;
            Ok(KdlIdentifier::Identifier { ident })
        }

        #[cfg(test)]
        pub(crate) fn ident(ident: impl AsRef<str>) -> Self {
            KdlIdentifier::Identifier {
                ident: syn::Ident::new(ident.as_ref(), proc_macro2::Span::call_site()),
            }
        }

        pub(crate) fn new_quoted(value: String, span: proc_macro2::Span) -> Self {
            KdlIdentifier::Quoted { value, span }
        }
    }

    impl PartialEq for KdlIdentifier {
        fn eq(&self, other: &Self) -> bool {
            self.value() == other.value()
        }
    }

    impl std::fmt::Display for KdlIdentifier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value())
        }
    }

    impl ToTokens for KdlIdentifier {
        fn to_tokens(&self, tokens: &mut TokenStream2) {
            let value = self.value();
            quote_spanned! {self.span()=>
                #value
            }
            .to_tokens(tokens);
        }
    }

    impl From<syn::LitStr> for KdlIdentifier {
        fn from(lit: syn::LitStr) -> Self {
            KdlIdentifier::Quoted {
                value: lit.value(),
                span: lit.span(),
            }
        }
    }
    impl From<syn::LitCStr> for KdlIdentifier {
        fn from(lit: syn::LitCStr) -> Self {
            KdlIdentifier::Quoted {
                value: lit.value().to_string_lossy().into_owned(),
                span: lit.span(),
            }
        }
    }
    impl From<(String, proc_macro2::Span)> for KdlIdentifier {
        fn from((value, span): (String, proc_macro2::Span)) -> Self {
            KdlIdentifier::Quoted { value, span }
        }
    }
    impl From<(&str, proc_macro2::Span)> for KdlIdentifier {
        fn from((value, span): (&str, proc_macro2::Span)) -> Self {
            (value.to_string(), span).into()
        }
    }

    impl TryFrom<syn::Ident> for KdlIdentifier {
        type Error = syn::Error;

        fn try_from(ident: syn::Ident) -> syn::Result<Self> {
            KdlIdentifier::new_identifier(ident)
        }
    }

    impl TryFrom<KdlValue> for KdlIdentifier {
        type Error = syn::Error;

        fn try_from(value: KdlValue) -> syn::Result<Self> {
            match value {
                KdlValue::String(kdl_string) => Ok(kdl_string),
                KdlValue::Variable(ident) => KdlIdentifier::new_identifier(ident),
                _ => Err(syn::Error::new(
                    value.span(),
                    "Expected a string literal or identifier.",
                )),
            }
        }
    }

    impl TryFrom<MaybeAnnotated<KdlValue>> for KdlIdentifier {
        type Error = syn::Error;

        fn try_from(value: MaybeAnnotated<KdlValue>) -> syn::Result<Self> {
            let item = KdlIdentifier::try_from(value.item)?;
            if let Some(type_ann) = value.type_annotation {
                return Err(syn::Error::new(
                    type_ann.span(),
                    "Type annotations are not allowed on property keys.",
                ));
            }
            Ok(item)
        }
    }

    impl syn::parse::Parse for KdlIdentifier {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let lookahead = input.lookahead1();
            // Check if it's a string literal first (quoted strings)
            if lookahead.peek(syn::LitStr) {
                let lit_str: syn::LitStr = input.parse()?;
                let kdl_string = KdlIdentifier::from(lit_str);
                Ok(kdl_string)
            } else if input.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                let kdl_string = KdlIdentifier::new_identifier(ident)?;
                Ok(kdl_string)
            } else {
                Err(lookahead.error())
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use crate::ast::KdlValue;

        impl From<String> for KdlIdentifier {
            fn from(value: String) -> Self {
                KdlIdentifier::Quoted {
                    value,
                    span: proc_macro2::Span::call_site(),
                }
            }
        }
        impl From<&str> for KdlIdentifier {
            fn from(value: &str) -> Self {
                KdlIdentifier::Quoted {
                    value: value.to_string(),
                    span: proc_macro2::Span::call_site(),
                }
            }
        }
        impl From<&str> for KdlValue {
            fn from(s: &str) -> Self {
                KdlValue::String(KdlIdentifier::from(s))
            }
        }
        impl From<String> for KdlValue {
            fn from(s: String) -> Self {
                KdlValue::String(KdlIdentifier::from(s))
            }
        }
        impl From<i128> for KdlValue {
            fn from(i: i128) -> Self {
                KdlValue::Lit(KdlLit::Integer(i, proc_macro2::Span::call_site()))
            }
        }
        impl From<f64> for KdlValue {
            fn from(f: f64) -> Self {
                KdlValue::Lit(KdlLit::Float(f, proc_macro2::Span::call_site()))
            }
        }

        impl From<bool> for KdlValue {
            fn from(b: bool) -> Self {
                KdlValue::Lit(KdlLit::Boolean(b, proc_macro2::Span::call_site()))
            }
        }
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

/// Checks if a type annotation is a reserved type
#[allow(dead_code)]
pub(crate) fn is_reserved_type(type_annotation: &str) -> bool {
    RESERVED_INTEGER_TYPES.contains(&type_annotation)
        || RESERVED_FLOAT_TYPES.contains(&type_annotation)
        || RESERVED_STRING_TYPES.contains(&type_annotation)
}

pub(crate) struct ConstPath<'a>(&'a [&'static str]);

impl ToTokens for ConstPath<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for segment in self.0 {
            let ident = format_ident!("{}", segment);
            tokens.extend(quote! { :: #ident });
        }
    }
}
