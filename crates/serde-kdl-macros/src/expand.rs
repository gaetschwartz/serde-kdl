//! Code generation for KDL documents
//!
//! This module handles the generation of Rust code from parsed KDL AST structures.

use crate::{
    ast::{
        extract_type_annotation, KdlDocument, KdlNode, KdlValue, KDL_NODE, SERDE_KDL_KDL_EXPORT,
    },
    parse::value::KdlLit,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{spanned::Spanned as _, Result};

pub(crate) fn generate_kdl_code(document: &KdlDocument) -> Result<TokenStream2> {
    let nodes = document.nodes.iter().map(expand_node_creation);
    // Generate LSP hints for IDE support
    let hints = ide_hints::write_hints(document);

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
    // Generate argument codes with type annotation handling
    let args = node.arguments.iter().map(|value| {
        if let Some(type_str) = extract_type_annotation(value) {
            quote! { {
                let mut entry = #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value);
                entry.set_ty(#type_str);
                entry
            } }
        } else {
            quote! { #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value) }
        }
    });

    // Generate property codes with type annotation handling
    let props = node.properties.iter().map(|prop| {
        let key = prop.key.value();
        let value = &prop.value;
        if let Some(type_str) = extract_type_annotation(&prop.value) {
            quote! { {
                let mut entry = #SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value);
                entry.set_ty(#type_str);
                entry
            } }
        } else {
            quote! { #SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value) }
        }
    });

    // Generate children codes
    let children_code = if node.has_children_block {
        let child_codes = node.children.iter().map(generate_child_node_code);
        quote! {
            node.set_children({
                let mut doc = #SERDE_KDL_KDL_EXPORT::KdlDocument::new();
                doc.nodes_mut().extend([#(#child_codes),*]);
                doc
            });
        }
    } else {
        quote! {}
    };

    let node_typing = node.type_annotation.as_deref().map(|ty| {
        quote! { node.set_ty(#ty); }
    });

    quote! {
        {
            let mut node = #KDL_NODE::new(#name);
            #node_typing
            node.entries_mut().extend([#(#props,)* #(#args,)*]);
            #children_code
            node
        }
    }
}

fn generate_child_node_code(node: &KdlNode) -> TokenStream2 {
    let name = node.name.value();
    // Generate argument codes with type annotation handling
    let args = node.arguments.iter().map(|value| {
        if let Some(type_str) = extract_type_annotation(value) {
            quote! { {
                let mut entry = #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value);
                entry.set_ty(#type_str);
                entry
            } }
        } else {
            quote! { #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value) }
        }
    });

    // Generate property codes with type annotation handling
    let props = node.properties.iter().map(|prop| {
        let key = prop.key.value();
        let value = &prop.value;
        if let Some(type_str) = extract_type_annotation(&prop.value) {
            quote! { {
                let mut entry = #SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value);
                entry.set_ty(#type_str);
                entry
            } }
        } else {
            quote! { #SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value) }
        }
    });

    // Generate nested children codes
    let nested_children_code = if node.has_children_block {
        let nested_child_codes = node.children.iter().map(generate_child_node_code);
        quote! {
            child_node.set_children({
                let mut doc = #SERDE_KDL_KDL_EXPORT::KdlDocument::new();
                doc.nodes_mut().extend([#(#nested_child_codes),*]);
                doc
            });
        }
    } else {
        quote! {}
    };

    let child_node_typing = node.type_annotation.as_ref().map(|ty| {
        quote! { child_node.set_ty(#ty); }
    });

    quote! { {
        let mut child_node = #KDL_NODE::new(#name);
        #child_node_typing
        child_node.entries_mut().extend([#(#props,)* #(#args,)*]);
        #nested_children_code
        child_node
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
    use proc_macro2::Span;

    use crate::ast::KdlString;

    use super::*;
    /// Generate phantom const bindings for LSP/IDE support.
    /// This creates bindings that use the same identifiers as the KDL input,
    /// allowing IDEs to provide syntax highlighting and other features.
    pub fn write_hints(document: &KdlDocument) -> TokenStream2 {
        let mut hints = quote! {};
        write_hints_from_nodes(&document.nodes, &mut hints);
        hints
    }

    fn write_hints_from_nodes(nodes: &[KdlNode], hints: &mut TokenStream2) {
        for node in nodes {
            // Generate hint for node names that are identifiers
            let node_ident = match &node.name {
                KdlString::Identifier { ident } => ident.clone(),
                KdlString::Quoted { value, span } => {
                    format_ident!("{}", sanitize_ident(value), span = *span)
                }
            };
            hints.extend(quote! {
                #[allow(non_snake_case, non_camel_case_types, dead_code, unused)]
                { struct #node_ident; }
            });

            // Generate hint for property keys that are identifiers
            for prop in &node.properties {
                let ident = match &prop.key {
                    KdlString::Identifier { ident } => ident.clone(),
                    KdlString::Quoted { value, span } => {
                        format_ident!("{}", sanitize_ident(value), span = *span)
                    }
                };
                let enum_ident =
                    format_ident!("{node_ident}_Prop_{}", ident, span = Span::call_site());
                hints.extend(quote! {
                    #[allow(non_snake_case, non_camel_case_types, dead_code, unused)]
                    { enum #enum_ident { #ident(#SERDE_KDL_KDL_EXPORT::KdlValue) } }
                });

                // Generate hint for property values
                hints.extend(value_hint(&prop.value));
            }

            // Generate hint for argument values
            for arg in &node.arguments {
                hints.extend(value_hint(arg));
            }

            // Recurse into children
            write_hints_from_nodes(&node.children, hints);
        }
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

    fn sanitize_ident(input: &str) -> syn::Ident {
        let mut output = String::with_capacity(input.len());
        if let Some(first_char) = input.chars().next() {
            if first_char.is_numeric() {
                output.push('_');
            }
        }
        for c in input.chars() {
            if c.is_alphanumeric() || c == '_' {
                output.push(c);
            } else {
                output.push('_');
            }
        }
        for _ in 0..3 {
            match syn::parse_str::<syn::Ident>(&output) {
                Ok(ident) => return ident,
                Err(_) => output.push('_'),
            }
        }
        panic!("Failed to sanitize identifier: {}", input);
    }
}
