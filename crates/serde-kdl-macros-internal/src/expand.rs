//! Code generation for KDL documents
//!
//! This module handles the generation of Rust code from parsed KDL AST structures.

use crate::{
    ast::{KDL_NODE, KdlDocument, KdlNode, KdlValue, SERDE_KDL_KDL_EXPORT},
    parse::{type_annotation::IntoSetTypeAnnotation as _, value::KdlLit},
};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Result, spanned::Spanned as _};

pub fn generate_kdl_code(document: &KdlDocument) -> Result<TokenStream2> {
    let nodes = document.nodes.iter().map(expand_node_creation);
    // Generate LSP hints for IDE support
    let hints = ide_hints::write_hints(document)?;

    Ok(quote! { {
        { #hints }
        {
            let mut document = #SERDE_KDL_KDL_EXPORT::KdlDocument::new();
            document.nodes_mut().extend([#(#nodes),*]);
            document
        }
    } })
}

fn expand_node_creation(node: &KdlNode) -> TokenStream2 {
    let name = node.name.value();

    // Generate code for entries (arguments and properties)
    let entries = node.entries.iter().map(|prop| {
        let key = prop.name_str();
        let value = &prop.value;
        let constructor = if let Some(key) = &key {
            quote! { #SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value) }
        } else {
            quote! { #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value) }
        };
        let entry = quote! { entry };
        let set_ty = value.as_set_ty(&entry);

        quote! { {
            let mut #entry = #constructor;
            #set_ty
            #entry
        } }
    });

    // Generate children codes
    let children_code = node.children.as_ref().map(|children| {
        let child_codes = children.nodes.iter().map(generate_child_node_code);
        quote! {
            node.set_children({
                let mut doc = #SERDE_KDL_KDL_EXPORT::KdlDocument::new();
                doc.nodes_mut().extend([#(#child_codes),*]);
                doc
            });
        }
    });

    let node_ident = quote! { node };
    let node_typing = node.as_set_ty(&node_ident);

    quote! {
        {
            let mut #node_ident = #KDL_NODE::new(#name);
            #node_typing
            #node_ident.entries_mut().extend([#(#entries),*]);
            #children_code
            #node_ident
        }
    }
}

fn generate_child_node_code(node: &KdlNode) -> TokenStream2 {
    let name = node.name.value();

    // Generate code for properties and arguments
    let entries = node.entries.iter().map(|entry| {
        let key = entry.name_str();
        let value = &entry.value;
        let constructor = if let Some(key) = &key {
            quote! { #SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value) }
        } else {
            quote! { #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value) }
        };
        let entry_ident = quote! { entry };
        let set_ty = value.as_set_ty(&entry_ident);

        quote! { {
            let mut #entry_ident = #constructor;
            #set_ty
            #entry_ident
        } }
    });

    // Generate nested children codes
    let nested_children_code = node.children.as_ref().map(|children| {
        let nested_child_codes = children.nodes.iter().map(generate_child_node_code);
        quote! {
            child_node.set_children({
                let mut doc = #SERDE_KDL_KDL_EXPORT::KdlDocument::new();
                doc.nodes_mut().extend([#(#nested_child_codes),*]);
                doc
            });
        }
    });

    let child_node_ident = quote! { child_node };
    let child_node_typing = node.as_set_ty(&child_node_ident);

    quote! { {
        let mut #child_node_ident = #KDL_NODE::new(#name);
        #child_node_typing
        #child_node_ident.entries_mut().extend([#(#entries),*]);
        #nested_children_code
        #child_node_ident
    } }
}

#[cfg(not(feature = "ide-hints"))]
mod ide_hints {
    pub fn write_hints(_document: &KdlDocument) -> TokenStream2 {
        quote! {}
    }
}

#[cfg(feature = "ide-hints")]
mod ide_hints {
    use super::*;
    use crate::ast::KdlIdentifier;
    use proc_macro2::Span;

    /// Generate phantom const bindings for LSP/IDE support.
    /// This creates bindings that use the same identifiers as the KDL input,
    /// allowing IDEs to provide syntax highlighting and other features.
    pub fn write_hints(document: &KdlDocument) -> syn::Result<TokenStream2> {
        let mut hints = quote! {};
        write_hints_from_nodes(&document.nodes, &mut hints)?;
        Ok(hints)
    }

    fn write_hints_from_nodes(nodes: &[KdlNode], hints: &mut TokenStream2) -> syn::Result<()> {
        for node in nodes {
            // Generate hint for node names that are identifiers
            let node_ident = match &node.name {
                KdlIdentifier::Identifier { ident } => ident.clone(),
                KdlIdentifier::Quoted { value, span } => {
                    format_ident!("{}", sanitize_ident(value)?, span = *span)
                }
            };
            hints.extend(quote! {
                #[allow(non_snake_case, non_camel_case_types, dead_code, unused)]
                { struct #node_ident; }
            });

            // Generate hint for property keys that are identifiers
            for entry in &node.entries {
                // Generate hint for property values
                hints.extend(value_hint(&entry.value));

                let ident = match &entry.name {
                    Some(KdlIdentifier::Identifier { ident }) => ident.clone(),
                    Some(KdlIdentifier::Quoted { value, span }) => {
                        format_ident!("{}", sanitize_ident(value)?, span = *span)
                    }
                    None => continue,
                };
                let enum_ident =
                    format_ident!("{node_ident}_Prop_{}", ident, span = Span::call_site());
                hints.extend(quote! {
                    #[allow(non_snake_case, non_camel_case_types, dead_code, unused)]
                    { enum #enum_ident { #ident(#SERDE_KDL_KDL_EXPORT::KdlValue) } }
                });
            }

            // Recurse into children
            if let Some(children) = &node.children {
                write_hints_from_nodes(&children.nodes, hints)?;
            }
        }

        Ok(())
    }

    fn value_hint(value: &KdlValue) -> TokenStream2 {
        match value {
            KdlValue::Lit(KdlLit::Null(null)) => {
                quote! {
                    #[allow(non_snake_case, non_upper_case_globals, unused)]
                    { const #null: #SERDE_KDL_KDL_EXPORT::KdlValue = #SERDE_KDL_KDL_EXPORT::KdlValue::Null; }
                }
            }
            KdlValue::Lit(KdlLit::Nan(nan)) => {
                quote! {
                    #[allow(non_snake_case, non_upper_case_globals, unused)]
                    { const #nan: #SERDE_KDL_KDL_EXPORT::KdlValue = #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::NAN); }
                }
            }
            KdlValue::Lit(KdlLit::Infinity(inf)) => {
                quote! {
                    #[allow(non_snake_case, non_upper_case_globals, unused)]
                    { const #inf: #SERDE_KDL_KDL_EXPORT::KdlValue = #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::INFINITY); }
                }
            }
            KdlValue::Lit(KdlLit::NegInfinity(inf)) => {
                let ident = format_ident!("neg_inf", span = inf.span());
                quote! {
                    #[allow(non_snake_case, non_upper_case_globals, unused)]
                    { const #ident: #SERDE_KDL_KDL_EXPORT::KdlValue = #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::NEG_INFINITY); }
                }
            }
            _ => quote! {},
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
        use super::ide_hints::sanitize_ident;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[rstest]
        #[case("123", Some("_123"))]
        #[case("foo", Some("foo"))]
        #[case("true", Some("r#true"))]
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
            let expected = expected.map(|s| s.to_string());

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_parse_true() {
        let parsed: syn::Ident = syn::parse_str("true").expect("Failed to parse");
        assert_eq!(parsed, "true");
    }
}
