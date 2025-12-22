use crate::parse::type_annotation::MaybeAnnotated;
use crate::parse::value::KdlValue;
use crate::validation::{self, ValidationOptions};
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
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
    use crate::parse::value::{KdlLit, PoundLiteral};
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use syn::{LitBool, Token};

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
            KdlValue::Lit(KdlLit::Boolean(PoundLiteral::new(
                Token![#](Span::call_site()),
                None,
                LitBool::new(b, Span::call_site()),
            )))
        }
    }
}
