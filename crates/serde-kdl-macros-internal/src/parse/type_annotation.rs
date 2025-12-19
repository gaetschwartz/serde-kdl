//! Type annotation parsing
//!
//! This module handles parsing of KDL type annotations according to Section 3.8
//! of the KDL specification.

use crate::ast::KdlIdentifier;
use quote::ToTokens;
use std::ops::Deref;
use syn::{parse::ParseStream, spanned::Spanned, token::Paren, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct MaybeAnnotated<T> {
    pub type_annotation: Option<KdlIdentifier>,
    pub item: T,
}

#[allow(dead_code)]
impl<T> MaybeAnnotated<T> {
    pub fn type_annotation(&self) -> Option<&KdlIdentifier> {
        self.type_annotation.as_ref()
    }

    pub fn new(item: T, type_annotation: Option<KdlIdentifier>) -> Self {
        Self {
            type_annotation,
            item,
        }
    }
}

impl<T: ToTokens> ToTokens for MaybeAnnotated<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.item.to_tokens(tokens);
    }
}

impl<T> Deref for MaybeAnnotated<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T: syn::parse::Parse + Spanned + std::fmt::Debug> syn::parse::Parse for MaybeAnnotated<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let type_annotation = if input.peek(Paren) {
            let content;
            syn::parenthesized!(content in input);
            let ident = content.parse::<KdlIdentifier>()?;
            // eprintln!(
            //     "[type_annotation] Parsed type annotation: {} at {}",
            //     ident,
            //     SpanDisplay(ident.span()),
            // );
            Some(ident)
        } else {
            None
        };
        let item: T = input.parse()?;
        // eprintln!(
        //     "[type_annotation] Parsed item {:?} at {}",
        //     item,
        //     SpanDisplay(item.span()),
        // );

        Ok(MaybeAnnotated {
            type_annotation,
            item,
        })
    }
}

impl<T: std::fmt::Display> std::fmt::Display for MaybeAnnotated<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(type_ann) = &self.type_annotation {
            write!(f, "({}){}", type_ann.value(), self.item)
        } else {
            write!(f, "{}", self.item)
        }
    }
}
