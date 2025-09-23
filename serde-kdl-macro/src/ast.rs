//! Abstract Syntax Tree (AST) definitions for KDL documents
//!
//! This module contains all the type definitions for representing KDL documents,
//! nodes, values, and related structures in memory.

use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::ToTokens;
use syn::{Path, Result, Token};

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

pub(crate) const SERDE_KDL_KDL_EXPORT: SerdeKdlPrivate = SerdeKdlPrivate(&["kdl"]);
pub(crate) const KDL_NODE: SerdeKdlPrivate = SerdeKdlPrivate(&["kdl", "KdlNode"]);
pub(crate) const KDL_ENTRY: SerdeKdlPrivate = SerdeKdlPrivate(&["kdl", "KdlEntry"]);

/// Represents a complete KDL document containing multiple nodes
#[derive(Debug, Clone)]
pub(crate) struct KdlDocument {
    pub(crate) nodes: Vec<KdlNode>,
}

/// Represents a single KDL node with optional properties, arguments, and children
#[derive(Debug, Clone)]
pub(crate) struct KdlNode {
    pub(crate) name: String,
    pub(crate) type_annotation: Option<String>, // Type annotation for node name
    pub(crate) properties: Vec<KdlProperty>,
    pub(crate) arguments: Vec<KdlValue>,
    pub(crate) children: Vec<KdlNode>,
    pub(crate) has_children_block: bool,
}

/// Represents a KDL property (key="value" pair)
#[derive(Debug, Clone)]
pub(crate) struct KdlProperty {
    pub(crate) key: String, // According to Section 3.7, property keys must be String values
    pub(crate) value: KdlValue,
}

/// Represents a KDL value (string, number, boolean, etc.)
#[derive(Clone)]
pub(crate) enum KdlValue {
    String(KdlString),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
    TypeAnnotated {
        type_annotation: String, // Use String instead of Ident for more flexibility
        value: Box<KdlValue>,
    },
}

/// Represents different types of KDL strings as per Section 3.9
#[derive(Clone, Debug)]
pub(crate) enum KdlString {
    /// Identifier String (Section 3.10) - like `foo`
    Identifier {
        value: String,
        span: proc_macro2::Span,
    },
    /// Quoted String (Section 3.11) - like `"foo"`
    Quoted {
        value: String,
        span: proc_macro2::Span,
    },
}

impl KdlString {
    /// Get the string value regardless of the string type
    pub(crate) fn value(&self) -> &str {
        match self {
            KdlString::Identifier { value, .. } | KdlString::Quoted { value, .. } => value,
        }
    }

    /// Get the span for error reporting
    pub(crate) fn span(&self) -> proc_macro2::Span {
        match self {
            KdlString::Identifier { span, .. } | KdlString::Quoted { span, .. } => *span,
        }
    }

    /// Create a quoted string from a LitStr with Unicode escape processing
    pub(crate) fn from_lit_str_as_quoted(lit_str: syn::LitStr) -> Result<Self> {
        let processed_value =
            crate::parse::string::process_string_escapes(&lit_str.value(), lit_str.span())?;
        Ok(KdlString::Quoted {
            value: processed_value,
            span: lit_str.span(),
        })
    }

    /// Create an identifier string from a string value and span
    #[allow(dead_code)]
    pub(crate) fn identifier(value: String, span: proc_macro2::Span) -> Self {
        KdlString::Identifier { value, span }
    }

    /// Validate the string according to Section 3.9 requirements
    pub(crate) fn validate(&self) -> Result<()> {
        self.validate_with_context(true)
    }

    /// Validate the string with context about whether keywords should be rejected
    pub(crate) fn validate_with_context(&self, reject_keywords: bool) -> Result<()> {
        // UTF-8 validation - Rust strings are already UTF-8, but let's be explicit
        if !self.value().is_ascii()
            && !self
                .value()
                .chars()
                .all(|c| c != char::REPLACEMENT_CHARACTER)
        {
            return Err(syn::Error::new(
                self.span(),
                "String contains invalid UTF-8 sequences",
            ));
        }

        // Apply specific validation based on string type
        match self {
            KdlString::Identifier { .. } => {
                // For identifier strings, apply Section 3.10 validation
                crate::validation::validate_identifier_string_with_context(
                    self.value(),
                    self.span(),
                    reject_keywords,
                )?;
            }
            KdlString::Quoted { .. } => {
                // Quoted strings can contain disallowed code points via escapes
                // We don't validate them here since they may have come from valid Unicode escapes
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for KdlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KdlValue::String(s) => write!(f, "String({})", s.value()),
            KdlValue::Integer(i) => write!(f, "Integer({})", i),
            KdlValue::Float(fl) => write!(f, "Float({})", fl),
            KdlValue::Boolean(b) => write!(f, "Boolean({})", b),
            KdlValue::Null => write!(f, "Null"),
            KdlValue::TypeAnnotated {
                type_annotation,
                value,
            } => {
                write!(f, "TypeAnnotated({}, {:?})", type_annotation, value)
            }
        }
    }
}

impl KdlValue {
    /// Checks if this value is a valid KDL value according to Section 3.7
    /// A value is either: String, Number (Integer/Float), Boolean, or Null
    pub(crate) fn is_valid_value(&self) -> bool {
        match self {
            KdlValue::String(_)
            | KdlValue::Integer(_)
            | KdlValue::Float(_)
            | KdlValue::Boolean(_)
            | KdlValue::Null => true,
            KdlValue::TypeAnnotated { value, .. } => value.is_valid_value(),
        }
    }

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

pub(crate) struct SerdeKdlPrivate<'a>(&'a [&'static str]);

impl ToTokens for SerdeKdlPrivate<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let path = Path {
            leading_colon: Some(Token![::](proc_macro2::Span::call_site())),
            segments: self
                .0
                .iter()
                .map(|s| syn::PathSegment {
                    ident: format_ident!("{}", s),
                    arguments: syn::PathArguments::None,
                })
                .collect(),
        };
        path.to_tokens(tokens);
    }
}
