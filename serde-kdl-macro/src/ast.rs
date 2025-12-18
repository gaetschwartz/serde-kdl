//! Abstract Syntax Tree (AST) definitions for KDL documents
//!
//! This module contains all the type definitions for representing KDL documents,
//! nodes, values, and related structures in memory.

use crate::parse::value::KdlLit;
pub(crate) use kdl_string::KdlString;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};

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
pub(crate) const KDL_ENTRY: ConstPath = ConstPath(&["kdl", "KdlEntry"]);

/// Represents a complete KDL document containing multiple nodes
#[derive(Debug, Clone)]
pub(crate) struct KdlDocument {
    pub(crate) nodes: Vec<KdlNode>,
}

/// Represents a single KDL node with optional properties, arguments, and children
#[derive(Debug, Clone)]
pub(crate) struct KdlNode {
    pub(crate) name: KdlString,
    pub(crate) type_annotation: Option<String>, // Type annotation for node name
    pub(crate) properties: Vec<KdlProperty>,
    pub(crate) arguments: Vec<KdlValue>,
    pub(crate) children: Vec<KdlNode>,
    pub(crate) has_children_block: bool,
}

/// Represents a KDL property (key="value" pair)
#[derive(Debug, Clone)]
pub(crate) struct KdlProperty {
    pub(crate) key: KdlString, // According to Section 3.7, property keys must be String values
    pub(crate) value: KdlValue,
}

/// Represents a KDL value (string, number, boolean, etc.)
#[derive(Clone, PartialEq)]
pub(crate) enum KdlValue {
    String(KdlString),
    TypeAnnotated {
        type_annotation: String, // Use String instead of Ident for more flexibility
        value: Box<KdlValue>,
    },
    /// A Rust variable reference (resolved at runtime)
    Variable(syn::Ident),
    Lit(KdlLit),
}

impl ToTokens for KdlValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            KdlValue::String(kdl_string) => {
                let s = kdl_string.value();
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::String(#s.to_string()) }
            }
            KdlValue::Lit(KdlLit::Integer(i)) => {
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Integer(#i) }
            }
            KdlValue::Lit(KdlLit::Float(f)) => {
                quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Float(#f) }
            }
            KdlValue::Lit(KdlLit::Boolean(b)) => {
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
            KdlValue::TypeAnnotated {
                type_annotation: _,
                value,
            } => {
                // For type-annotated values, we just generate the inner value
                // The type annotation will be handled at the entry level
                value.to_token_stream()
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

mod kdl_string {
    use crate::validation::{self, ValidationOptions};

    /// Represents different types of KDL strings as per Section 3.9
    #[derive(Clone)]
    pub struct KdlString {
        inner: KdlStringInner,
    }
    /// Represents different types of KDL strings as per Section 3.9
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    enum KdlStringInner {
        /// Identifier String (Section 3.10) - like `foo`
        Identifier { ident: syn::Ident },
        /// Quoted String (Section 3.11) - like `"foo"`
        Quoted {
            value: String,
            span: proc_macro2::Span,
        },
    }

    impl PartialEq for KdlString {
        fn eq(&self, other: &Self) -> bool {
            self.value() == other.value()
        }
    }

    impl std::fmt::Debug for KdlString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.fmt(f)
        }
    }

    #[allow(dead_code)]
    impl KdlString {
        /// Get the string value regardless of the string type
        pub(crate) fn value(&self) -> String {
            match &self.inner {
                KdlStringInner::Identifier { ident } => ident.to_string(),
                KdlStringInner::Quoted { value, .. } => value.clone(),
            }
        }

        /// Get the span for error reporting
        pub(crate) fn span(&self) -> proc_macro2::Span {
            match &self.inner {
                KdlStringInner::Identifier { ident } => ident.span(),
                KdlStringInner::Quoted { span, .. } => *span,
            }
        }

        /// Get the identifier if this is an Identifier variant
        pub(crate) fn as_ident(&self) -> Option<&syn::Ident> {
            match &self.inner {
                KdlStringInner::Identifier { ident } => Some(ident),
                KdlStringInner::Quoted { .. } => None,
            }
        }

        pub(crate) fn new_identifier(ident: syn::Ident) -> syn::Result<Self> {
            validation::validate_identifier(&ident, ValidationOptions::default())?;
            Ok(KdlString {
                inner: KdlStringInner::Identifier { ident },
            })
        }
    }

    impl From<syn::LitStr> for KdlString {
        fn from(lit: syn::LitStr) -> Self {
            KdlString {
                inner: KdlStringInner::Quoted {
                    value: lit.value(),
                    span: lit.span(),
                },
            }
        }
    }
    impl From<syn::LitCStr> for KdlString {
        fn from(lit: syn::LitCStr) -> Self {
            KdlString {
                inner: KdlStringInner::Quoted {
                    value: lit.value().to_string_lossy().into_owned(),
                    span: lit.span(),
                },
            }
        }
    }

    impl TryFrom<syn::Ident> for KdlString {
        type Error = syn::Error;

        fn try_from(ident: syn::Ident) -> syn::Result<Self> {
            KdlString::new_identifier(ident)
        }
    }

    impl syn::parse::Parse for KdlString {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let lookahead = input.lookahead1();
            // Check if it's a string literal first (quoted strings)
            if lookahead.peek(syn::LitStr) {
                let lit_str: syn::LitStr = input.parse()?;
                let kdl_string = KdlString::from(lit_str);
                Ok(kdl_string)
            } else if input.peek(syn::Ident) {
                // Otherwise parse as identifier (bare strings)
                let ident: syn::Ident = input.parse()?;
                let kdl_string = KdlString::new_identifier(ident)?;
                Ok(kdl_string)
            } else {
                Err(lookahead.error())
            }
        }
    }
}

impl std::fmt::Debug for KdlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KdlValue::String(s) => write!(f, "String({})", s.value()),
            KdlValue::TypeAnnotated {
                type_annotation,
                value,
            } => {
                write!(f, "TypeAnnotated({}, {:?})", type_annotation, value)
            }
            KdlValue::Variable(ident) => write!(f, "Variable({})", ident),
            KdlValue::Lit(lit) => write!(f, "Lit({:?})", lit),
        }
    }
}

impl KdlValue {
    /// Checks if this value is a String value (required for node names and property keys)
    #[allow(dead_code)]
    pub(crate) fn is_string_value(&self) -> bool {
        match self {
            KdlValue::String(_) => true,
            KdlValue::TypeAnnotated { value, .. } => value.is_string_value(),
            _ => false,
        }
    }

    /// Returns the string representation if this is a string value
    #[allow(dead_code)]
    pub(crate) fn as_string(&self) -> Option<String> {
        match self {
            KdlValue::String(s) => Some(s.value().to_string()),
            KdlValue::TypeAnnotated { value, .. } => value.as_string(),
            _ => None,
        }
    }
}

/// Extract type annotation from a KdlValue if present
pub(crate) fn extract_type_annotation(value: &KdlValue) -> Option<&str> {
    match value {
        KdlValue::TypeAnnotated {
            type_annotation, ..
        } => Some(type_annotation),
        _ => None,
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
