//! Code generation for KDL documents
//!
//! This module handles the generation of Rust code from parsed KDL AST structures.

use crate::{
    ast::{
        extract_type_annotation, KdlDocument, KdlNode, KdlValue, KDL_ENTRY, KDL_NODE,
        SERDE_KDL_KDL_EXPORT,
    },
    parse::value::KdlLit,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{spanned::Spanned as _, Result};

pub(crate) fn generate_kdl_code(document: &KdlDocument) -> Result<TokenStream2> {
    let node_codes: Result<Vec<_>> = document.nodes.iter().map(generate_node_code).collect();
    let node_codes = node_codes?;

    // Generate LSP hints for IDE support
    let lsp_hints = ide_hints::generate_lsp_hints(document);

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
        .map(|value| {
            let type_annotation = extract_type_annotation(value);
            if let Some(type_str) = type_annotation {
                Ok(quote! {
                    {
                        let mut entry = #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value);
                        entry.set_ty(#type_str);
                        entry
                    }
                })
            } else {
                Ok(quote! {
                    #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value)
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
            let value = &prop.value;
            let type_annotation = extract_type_annotation(&prop.value);

            if let Some(type_str) = type_annotation {
                Ok(quote! {
                    {
                        let mut entry = #SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value);
                        entry.set_ty(#type_str);
                        node.entries_mut().push(entry);
                    }
                })
            } else {
                Ok(quote! {
                    node.entries_mut().push(#SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value));
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
        .map(|value| {
            let type_annotation = extract_type_annotation(value);

            if let Some(type_str) = type_annotation {
                Ok(quote! {
                    {
                        let mut entry = #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value);
                        entry.set_ty(#type_str);
                        entry
                    }
                })
            } else {
                Ok(quote! {
                    #SERDE_KDL_KDL_EXPORT::KdlEntry::new(#value)
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
            let value = &prop.value;
            let type_annotation = extract_type_annotation(&prop.value);

            if let Some(type_str) = type_annotation {
                Ok(quote! {
                    {
                        let mut entry = #SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value);
                        entry.set_ty(#type_str);
                        child_node.entries_mut().push(entry);
                    }
                })
            } else {
                Ok(quote! {
                    child_node.entries_mut().push(#SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value));
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

#[cfg(not(feature = "ide-hints"))]
mod ide_hints {
    pub fn generate_lsp_hints(document: &KdlDocument) -> TokenStream2 {
        TokenStream2::new()
    }
}

#[cfg(feature = "ide-hints")]
mod ide_hints {
    use crate::ast::KdlString;

    use super::*;
    /// Generate phantom const bindings for LSP/IDE support.
    /// This creates bindings that use the same identifiers as the KDL input,
    /// allowing IDEs to provide syntax highlighting and other features.
    pub fn generate_lsp_hints(document: &KdlDocument) -> TokenStream2 {
        let mut hints = TokenStream2::new();
        collect_hints_from_nodes(&document.nodes, &mut hints);
        hints
    }

    fn collect_hints_from_nodes(nodes: &[KdlNode], hints: &mut TokenStream2) {
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
                hints.extend(quote! {
                    #[allow(non_snake_case, non_camel_case_types, dead_code, unused)]
                    { enum #node_ident { #ident(#SERDE_KDL_KDL_EXPORT::KdlValue) } }
                });

                // Generate hint for property values
                hints.extend(value_hint(&prop.value));
            }

            // Generate hint for argument values
            for arg in &node.arguments {
                hints.extend(value_hint(arg));
            }

            // Recurse into children
            collect_hints_from_nodes(&node.children, hints);
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

    fn sanitize_ident(input: &str) -> String {
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
        output
    }
}
