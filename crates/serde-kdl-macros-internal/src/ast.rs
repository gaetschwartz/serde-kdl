//! Abstract Syntax Tree (AST) definitions for KDL documents
//!
//! This module contains all the type definitions for representing KDL documents,
//! nodes, values, and related structures in memory.

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};

/// Reserved type annotations for numbers without decimals (Section 3.8.1)
#[allow(dead_code)]
pub const RESERVED_INTEGER_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "isize", "usize",
];

/// Reserved type annotations for numbers with decimals (Section 3.8.2)
#[allow(dead_code)]
pub const RESERVED_FLOAT_TYPES: &[&str] = &["f32", "f64", "decimal64", "decimal128"];

/// Reserved type annotations for strings (Section 3.8.3)
#[allow(dead_code)]
pub const RESERVED_STRING_TYPES: &[&str] = &[
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

pub const SERDE_KDL_KDL_EXPORT: ConstPath = ConstPath(&["kdl"]);
pub const KDL_NODE: ConstPath = ConstPath(&["kdl", "KdlNode"]);

/// Checks if a type annotation is a reserved type
#[allow(dead_code)]
#[must_use]
pub fn is_reserved_type(type_annotation: &str) -> bool {
    RESERVED_INTEGER_TYPES.contains(&type_annotation)
        || RESERVED_FLOAT_TYPES.contains(&type_annotation)
        || RESERVED_STRING_TYPES.contains(&type_annotation)
}

pub struct ConstPath<'a>(&'a [&'static str]);

impl ToTokens for ConstPath<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for segment in self.0 {
            let ident = format_ident!("{}", segment);
            tokens.extend(quote! { :: #ident });
        }
    }
}
