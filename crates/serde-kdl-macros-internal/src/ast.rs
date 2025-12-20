//! Abstract Syntax Tree (AST) definitions for KDL documents
//!
//! This module contains all the type definitions for representing KDL documents,
//! nodes, values, and related structures in memory.

use std::borrow::Cow;

use crate::parse::{type_annotation::MaybeAnnotated, value::KdlLit};
pub use kdl_string::KdlIdentifier;
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

/// Represents a complete KDL document containing multiple nodes
#[derive(Debug, Clone)]
pub struct KdlDocument {
    pub nodes: Vec<KdlNode>,
}

impl KdlDocument {
    #[must_use]
    pub fn nodes(&self) -> &[KdlNode] {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut Vec<KdlNode> {
        &mut self.nodes
    }

    #[must_use]
    pub fn from_nodes(nodes: Vec<KdlNode>) -> Self {
        KdlDocument { nodes }
    }
}

/// Represents a single KDL node with optional properties, arguments, and children
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct KdlNode {
    pub name: KdlIdentifier,
    pub type_annotation: Option<KdlIdentifier>, // Type annotation for node name
    pub entries: Vec<KdlEntry>,
    pub children: Option<KdlDocument>,
    pub terminator: Terminator,
}

impl KdlNode {
    #[must_use]
    pub fn ty(&self) -> Option<&KdlIdentifier> {
        self.type_annotation.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Terminator {
    Brace,
    Semicolon,
    Eol,
    Eof,
}

/// Represents a KDL property (key="value" pair)
#[derive(Debug, Clone, PartialEq)]
pub struct KdlEntry {
    pub name: Option<KdlIdentifier>, // According to Section 3.7, property keys must be String values
    pub value: MaybeAnnotated<KdlValue>,
}

impl KdlEntry {
    pub fn new_prop(name: impl Into<KdlIdentifier>, value: impl Into<KdlValue>) -> KdlEntry {
        KdlEntry {
            name: Some(name.into()),
            value: MaybeAnnotated::new(value.into()),
        }
    }

    pub fn new(value: impl Into<KdlValue>) -> KdlEntry {
        KdlEntry {
            name: None,
            value: MaybeAnnotated::new(value.into()),
        }
    }

    pub fn new_typed_prop(
        name: impl Into<KdlIdentifier>,
        value: MaybeAnnotated<impl Into<KdlValue>>,
    ) -> KdlEntry {
        KdlEntry {
            name: Some(name.into()),
            value: MaybeAnnotated {
                item: value.item.into(),
                type_annotation: value.type_annotation,
            },
        }
    }

    pub fn new_typed_arg(value: MaybeAnnotated<impl Into<KdlValue>>) -> KdlEntry {
        KdlEntry {
            name: None,
            value: MaybeAnnotated {
                item: value.item.into(),
                type_annotation: value.type_annotation,
            },
        }
    }

    pub fn set_ty(&mut self, type_annotation: KdlIdentifier) {
        self.value.type_annotation = Some(type_annotation);
    }

    #[must_use]
    pub fn ty(&self) -> Option<&KdlIdentifier> {
        self.value.type_annotation.as_ref()
    }

    #[must_use]
    pub fn name(&self) -> Option<&KdlIdentifier> {
        self.name.as_ref()
    }

    #[must_use]
    pub fn name_str(&self) -> Option<Cow<'_, str>> {
        self.name.as_ref().map(|n| n.value())
    }
}

/// Represents a KDL value (string, number, boolean, etc.)
#[derive(Clone, PartialEq)]
pub enum KdlValue {
    String(KdlIdentifier),
    Variable(syn::Ident),
    Lit(KdlLit),
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

mod kdl_string {
    use super::*;
    use crate::validation::{self, ValidationOptions};
    use proc_macro2::Span;
    use quote::{ToTokens, quote_spanned};
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

    impl syn::parse::Parse for KdlIdentifier {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let lookahead = input.lookahead1();
            // Check if it's a string literal first (quoted strings)
            if lookahead.peek(syn::LitStr) {
                let lit_str: syn::LitStr = input.parse()?;
                let kdl_string = KdlIdentifier::from(lit_str);
                // eprintln!("[kdl_string] Parsed quoted string: {}", kdl_string.value());
                Ok(kdl_string)
            } else if input.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                let kdl_string = KdlIdentifier::new_identifier(ident)?;
                // eprintln!("[kdl_string] Parsed identifier: {}", kdl_string.value());
                Ok(kdl_string)
            } else {
                Err(lookahead.error())
            }
        }
    }

    #[allow(dead_code)]
    impl KdlIdentifier {
        /// Get the string value regardless of the string type
        #[must_use]
        pub fn value(&self) -> Cow<'_, str> {
            match &self {
                KdlIdentifier::Identifier { ident } => ident.to_string().into(),
                KdlIdentifier::Quoted { value, .. } => value.into(),
            }
        }

        /// Get the span for error reporting
        #[must_use]
        pub fn span(&self) -> proc_macro2::Span {
            match &self {
                KdlIdentifier::Identifier { ident } => ident.span(),
                KdlIdentifier::Quoted { span, .. } => *span,
            }
        }

        pub fn new_identifier(ident: syn::Ident) -> syn::Result<Self> {
            validation::validate_identifier(&ident, ValidationOptions::default())?;
            Ok(KdlIdentifier::Identifier { ident })
        }

        #[cfg(test)]
        pub fn ident_test(ident: impl AsRef<str>) -> Self {
            KdlIdentifier::Identifier {
                ident: syn::Ident::new(ident.as_ref(), proc_macro2::Span::call_site()),
            }
        }

        #[must_use]
        pub fn new_quoted(value: String, span: proc_macro2::Span) -> Self {
            KdlIdentifier::Quoted { value, span }
        }

        pub fn to_ident(&self) -> syn::Result<syn::Ident> {
            match self {
                KdlIdentifier::Identifier { ident } => Ok(ident.clone()),
                KdlIdentifier::Quoted { value, span } => {
                    let mut ident = sanitize_ident(value)?;
                    ident.set_span(*span);
                    Ok(ident)
                }
            }
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

    impl PartialEq<kdl::KdlIdentifier> for KdlIdentifier {
        fn eq(&self, other: &kdl::KdlIdentifier) -> bool {
            self.value() == other.value()
        }
    }

    const ULTRA_RESERVED: &[&str] = &["crate", "self", "super", "Self"];

    fn sanitize_ident(input: &str) -> syn::Result<syn::Ident> {
        let mut output = String::with_capacity(input.len());
        #[inline]
        fn to_valid_char(c: char) -> char {
            if unicode_ident::is_xid_continue(c) {
                c
            } else {
                '_'
            }
        }
        let mut chars = input.chars();
        if let Some(first_char) = chars.next() {
            if !unicode_ident::is_xid_start(first_char) {
                output.push('_');
            }
            output.push(to_valid_char(first_char));
        }
        for c in chars {
            output.push(to_valid_char(c));
        }
        if ULTRA_RESERVED.contains(&output.as_str()) {
            output.push('_');
        }
        if let Ok(ident) = syn::parse_str::<syn::Ident>(&output) {
            // eprintln!("[ide-hints] Sanitized identifier: {} -> {}", input, output);
            return Ok(ident);
        }
        output.push('_');
        match syn::parse_str::<syn::Ident>(&output) {
            Ok(ident) => {
                // eprintln!(
                //     "[ide-hints] Sanitized identifier with fallback: {} -> {}",
                //     input, output
                // );
                Ok(ident)
            }
            Err(e) => Err(syn::Error::new(
                Span::call_site(),
                format!("Failed to sanitize identifier '{input}' to a valid Rust identifier: {e}"),
            )),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::ast::KdlValue;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[rstest]
        #[case("123", Some("_123"))]
        #[case("foo", Some("foo"))]
        #[case("true", Some("true_"))]
        #[case("self", Some("self_"))]
        #[case("foo-bar", Some("foo_bar"))]
        #[case("foo bar", Some("foo_bar"))]
        #[case("foo@bar", Some("foo_bar"))]
        #[case("foo/bar", Some("foo_bar"))]
        // really wild cases
        #[case("!@#$%^&*()", Some("___________"))]
        // unicode cases
        #[case("变量", Some("变量"))]
        // unicode cases starting with non XID_Start but non ascii
        #[case("\u{0667}", Some("_\u{0667}"))] // Arabic-Indic Digit Seven "٧"
        #[case("٧", Some("_٧"))] // Arabic-Indic Digit Seven "٧"
        fn test_sanitize_ident(#[case] input: &str, #[case] expected: Option<&str>) {
            let result = sanitize_ident(input).ok().map(|id| id.to_string());
            let expected = expected.map(std::string::ToString::to_string);

            assert_eq!(result, expected);
        }

        #[test]
        #[should_panic]
        fn test_parse_true() {
            let _: syn::Ident = syn::parse_str("true").expect("Failed to parse");
        }

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

impl PartialEq<kdl::KdlDocument> for KdlDocument {
    fn eq(&self, other: &kdl::KdlDocument) -> bool {
        if self.nodes.len() != other.nodes().len() {
            return false;
        }

        self.nodes == other.nodes()
    }
}
impl PartialEq<kdl::KdlNode> for KdlNode {
    fn eq(&self, other: &kdl::KdlNode) -> bool {
        if &self.name != other.name() {
            return false;
        }

        match (&self.type_annotation, other.ty()) {
            (Some(a), Some(b)) if a != b => return false,
            (None, Some(_)) | (Some(_), None) => return false,
            _ => {}
        }

        let my_entries = &self.entries;
        let other_entries = other.entries();

        if my_entries != other_entries {
            return false;
        }

        match (&self.children, other.children()) {
            (Some(a), Some(b)) if a != b => return false,
            (None, Some(_)) | (Some(_), None) => return false,
            _ => {}
        }

        true
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

impl PartialEq<kdl::KdlEntry> for KdlEntry {
    fn eq(&self, other: &kdl::KdlEntry) -> bool {
        match (&self.name, other.name()) {
            (Some(a), Some(b)) if a != b => return false,
            (None, Some(_)) | (Some(_), None) => return false,
            _ => {}
        }
        if &self.value.item != other.value() {
            return false;
        }
        match (&self.value.type_annotation, other.ty()) {
            (Some(a), Some(b)) if a != b => return false,
            (None, Some(_)) | (Some(_), None) => return false,
            _ => {}
        }
        true
    }
}
