//! Code generation for KDL documents
//!
//! This module handles the generation of Rust code from parsed KDL AST structures.

use crate::ast::{
    extract_type_annotation, KdlDocument, KdlNode, KdlValue, KDL_ENTRY, KDL_NODE,
    SERDE_KDL_KDL_EXPORT,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Result;

pub(crate) fn generate_kdl_code(document: &KdlDocument) -> Result<TokenStream2> {
    let node_codes: Result<Vec<_>> = document.nodes.iter().map(generate_node_code).collect();
    let node_codes = node_codes?;

    // Generate LSP hints for IDE support
    let lsp_hints = generate_lsp_hints(document);

    Ok(quote! {
        {
            #lsp_hints
            let mut document = #SERDE_KDL_KDL_EXPORT::KdlDocument::new();
            #(#node_codes)*
            document
        }
    })
}

fn generate_node_code(node: &KdlNode) -> Result<TokenStream2> {
    let name = node.name.value();
    let type_annotation = &node.type_annotation;

    // Generate argument codes with type annotation handling
    let arg_codes: Result<Vec<_>> = node
        .arguments
        .iter()
        .map(|arg| {
            let value_code = generate_value_code(arg)?;
            let type_annotation = extract_type_annotation(arg);

            if let Some(type_str) = type_annotation {
                Ok(quote! {
                    {
                        let mut entry = #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value_code);
                        entry.set_ty(#type_str);
                        entry
                    }
                })
            } else {
                Ok(quote! {
                    #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value_code)
                })
            }
        })
        .collect();
    let arg_codes = arg_codes?;

    // Generate property codes with type annotation handling
    let prop_codes: Result<Vec<_>> = node
        .properties
        .iter()
        .map(|prop| {
            let key = prop.key.value();
            let value_code = generate_value_code(&prop.value)?;
            let type_annotation = extract_type_annotation(&prop.value);

            if let Some(type_str) = type_annotation {
                Ok(quote! {
                    {
                        let mut entry = #SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value_code);
                        entry.set_ty(#type_str);
                        node.entries_mut().push(entry);
                    }
                })
            } else {
                Ok(quote! {
                    node.entries_mut().push(#SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value_code));
                })
            }
        })
        .collect();
    let prop_codes = prop_codes?;

    // Generate children codes
    let children_code = if node.has_children_block {
        let child_codes: Result<Vec<_>> =
            node.children.iter().map(generate_child_node_code).collect();
        let child_codes = child_codes?;

        quote! {
            {
                let mut children_doc = #SERDE_KDL_KDL_EXPORT::KdlDocument::new();
                #(
                    {
                        #child_codes
                        children_doc.nodes_mut().push(child_node);
                    }
                )*
                node.set_children(children_doc);
            }
        }
    } else {
        quote! {}
    };

    let _kdl_entry = &KDL_ENTRY;
    let node_creation = if let Some(type_ann) = type_annotation {
        quote! {
            let mut node = #KDL_NODE::new(#name);
            node.set_ty(#type_ann);
        }
    } else {
        quote! {
            let mut node = #KDL_NODE::new(#name);
        }
    };

    Ok(quote! {
        {
            #node_creation
            #(
                node.entries_mut().push(#arg_codes);
            )*
            #(#prop_codes)*
            #children_code
            document.nodes_mut().push(node);
        }
    })
}

fn generate_child_node_code(node: &KdlNode) -> Result<TokenStream2> {
    let name = node.name.value();
    let type_annotation = &node.type_annotation;

    // Generate argument codes with type annotation handling
    let arg_codes: Result<Vec<_>> = node
        .arguments
        .iter()
        .map(|arg| {
            let value_code = generate_value_code(arg)?;
            let type_annotation = extract_type_annotation(arg);

            if let Some(type_str) = type_annotation {
                Ok(quote! {
                    {
                        let mut entry = #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value_code);
                        entry.set_ty(#type_str);
                        entry
                    }
                })
            } else {
                Ok(quote! {
                    #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value_code)
                })
            }
        })
        .collect();
    let arg_codes = arg_codes?;

    // Generate property codes with type annotation handling
    let prop_codes: Result<Vec<_>> = node
        .properties
        .iter()
        .map(|prop| {
            let key = prop.key.value();
            let value_code = generate_value_code(&prop.value)?;
            let type_annotation = extract_type_annotation(&prop.value);

            if let Some(type_str) = type_annotation {
                Ok(quote! {
                    {
                        let mut entry = #SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value_code);
                        entry.set_ty(#type_str);
                        child_node.entries_mut().push(entry);
                    }
                })
            } else {
                Ok(quote! {
                    child_node.entries_mut().push(#SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value_code));
                })
            }
        })
        .collect();
    let prop_codes = prop_codes?;

    // Generate nested children codes
    let nested_children_code = if node.has_children_block {
        let nested_child_codes: Result<Vec<_>> =
            node.children.iter().map(generate_child_node_code).collect();
        let nested_child_codes = nested_child_codes?;

        quote! {
            {
                let mut nested_children_doc = #SERDE_KDL_KDL_EXPORT::KdlDocument::new();
                #(
                    {
                        #nested_child_codes
                        nested_children_doc.nodes_mut().push(child_node);
                    }
                )*
                child_node.set_children(nested_children_doc);
            }
        }
    } else {
        quote! {}
    };

    let _kdl_entry = &KDL_ENTRY;
    let child_node_creation = if let Some(type_ann) = type_annotation {
        quote! {
            let mut child_node = #KDL_NODE::new(#name);
            child_node.set_ty(#type_ann);
        }
    } else {
        quote! {
            let mut child_node = #KDL_NODE::new(#name);
        }
    };

    Ok(quote! {
        #child_node_creation
        #(
            child_node.entries_mut().push(#arg_codes);
        )*
        #(#prop_codes)*
        #nested_children_code
    })
}

fn generate_value_code(value: &KdlValue) -> Result<TokenStream2> {
    match value {
        KdlValue::String(kdl_string) => {
            let s = kdl_string.value();
            Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::String(#s.to_string()) })
        }
        KdlValue::Integer(i) => Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Integer(#i) }),
        KdlValue::Float(f) => {
            // Handle special float values that can't be directly quoted
            if f.is_infinite() {
                if f.is_sign_positive() {
                    Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::INFINITY) })
                } else {
                    Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::NEG_INFINITY) })
                }
            } else if f.is_nan() {
                Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::NAN) })
            } else {
                Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Float(#f) })
            }
        }
        KdlValue::Boolean(b) => Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Bool(#b) }),
        KdlValue::Null => Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Null }),
        KdlValue::TypeAnnotated {
            type_annotation: _,
            value,
        } => {
            // For type-annotated values, we just generate the inner value
            // The type annotation will be handled at the entry level
            generate_value_code(value)
        }
        KdlValue::Variable(ident) => {
            // Use KdlValue::from() - user's variable type must implement Into<KdlValue>
            // kdl::KdlValue implements From for: i128, f64, &str, String, bool, Option<T>
            Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::from(#ident) })
        }
    }
}

/// Generate phantom const bindings for LSP/IDE support.
/// This creates bindings that use the same identifiers as the KDL input,
/// allowing IDEs to provide syntax highlighting and other features.
fn generate_lsp_hints(document: &KdlDocument) -> TokenStream2 {
    let mut hints = Vec::new();
    collect_hints_from_nodes(&document.nodes, &mut hints);

    if hints.is_empty() {
        quote! {}
    } else {
        quote! { #(#hints)* }
    }
}

fn collect_hints_from_nodes(nodes: &[KdlNode], hints: &mut Vec<TokenStream2>) {
    for node in nodes {
        // Generate hint for node names that are identifiers
        if let Some(ident) = node.name.as_ident() {
            hints.push(quote! {
                #[doc(hidden)]
                #[allow(non_camel_case_types, dead_code, unused)]
                const _: () = { let #ident: () = (); };
            });
        }

        // Generate hint for property keys that are identifiers
        for prop in &node.properties {
            if let Some(ident) = prop.key.as_ident() {
                hints.push(quote! {
                    #[doc(hidden)]
                    #[allow(non_upper_case_globals, dead_code, unused)]
                    const _: () = { let #ident: () = (); };
                });
            }
        }

        // Recurse into children
        collect_hints_from_nodes(&node.children, hints);
    }
}
