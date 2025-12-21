//! Code generation for KDL documents
//!
//! This module handles the generation of Rust code from parsed KDL AST structures.

use crate::{
    ast::{KDL_NODE, SERDE_KDL_KDL_EXPORT},
    parse::{
        document::KdlDocument, node::KdlNode, type_annotation::IntoSetTypeAnnotation as _,
        value::KdlLit,
    },
};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Result, spanned::Spanned as _};

pub fn generate_kdl_code(document: &KdlDocument) -> Result<TokenStream2> {
    let nodes = document.nodes.iter().map(expand_node_creation);
    // Generate LSP hints for IDE support
    let mut hints = quote! {};
    hints.extend(ide_hints::write_hints(document)?);

    Ok(quote! { {
        { mod ide_hints { fn ide_hints() { #hints } } }
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
    use super::*;

    pub fn write_hints(_document: &KdlDocument) -> syn::Result<TokenStream2> {
        Ok(quote! {})
    }
}

#[cfg(feature = "ide-hints")]
mod ide_hints {
    use crate::parse::{entry::KdlEntry, value::KdlValue};

    use super::*;
    use proc_macro2::Span;
    use quote::quote_spanned;

    /// Generate phantom const bindings for LSP/IDE support.
    /// This creates bindings that use the same identifiers as the KDL input,
    /// allowing IDEs to provide syntax highlighting and other features.
    pub fn write_hints(document: &KdlDocument) -> syn::Result<TokenStream2> {
        let mut hints = quote! {};
        write_hints_from_document(document, &mut hints)?;
        Ok(hints)
    }

    fn write_hints_from_document(
        document: &KdlDocument,
        hints: &mut TokenStream2,
    ) -> syn::Result<()> {
        // Generate hints for active nodes
        write_hints_from_nodes(&document.nodes, hints)?;

        // Generate hints for ignored nodes (with deprecated attribute)
        for node in &document.ignored_nodes {
            write_ignored_node_hints(node, hints)?;
        }

        Ok(())
    }

    fn write_hints_from_nodes(nodes: &[KdlNode], hints: &mut TokenStream2) -> syn::Result<()> {
        for node in nodes {
            write_node_hints(node, hints)?;
        }
        Ok(())
    }

    fn write_node_hints(node: &KdlNode, hints: &mut TokenStream2) -> syn::Result<()> {
        // Generate hint for node names that are identifiers
        let node_ident = node.name.to_ident()?;
        hints.extend(quote! {
            #[allow(non_snake_case, non_camel_case_types, unused)]
            { struct #node_ident; }
        });

        // Generate hint for active entries
        for entry in &node.entries {
            write_entry_hints(entry, &node_ident, hints)?;
        }

        // Generate hint for ignored entries (with deprecated attribute)
        for entry in &node.ignored_entries {
            write_ignored_entry_hints(entry, &node_ident, hints)?;
        }

        // Recurse into active children
        if let Some(children) = &node.children {
            write_hints_from_document(children, hints)?;
        }

        // Generate hints for ignored children (with deprecated attribute)
        if let Some(ignored_children) = &node.ignored_children {
            for child_node in &ignored_children.nodes {
                write_ignored_node_hints(child_node, hints)?;
            }
            for child_node in &ignored_children.ignored_nodes {
                write_ignored_node_hints(child_node, hints)?;
            }
        }

        // Generate hint for node type annotation
        if let Some(ty) = node.ty() {
            let type_token = quote_spanned!(ty.span()=> type);
            hints.extend(quote! {
                #[allow(unused)]
                { #type_token _Ty = (); }
            });
        }

        Ok(())
    }

    fn write_entry_hints(
        entry: &KdlEntry,
        node_ident: &syn::Ident,
        hints: &mut TokenStream2,
    ) -> syn::Result<()> {
        // Generate hint for values
        hints.extend(value_hint(&entry.value));

        if let Some(ty) = entry.ty() {
            let type_token = quote_spanned!(ty.span()=> type);
            hints.extend(quote! {
                #[allow(unused)]
                { #type_token _Ty = (); }
            });
        }

        let Some(name) = &entry.name else {
            return Ok(());
        };
        let ident = name.to_ident()?;

        let enum_ident = format_ident!("{ident}", span = Span::call_site());
        hints.extend(quote! {
            #[allow(non_snake_case, non_camel_case_types, unused)]
            { enum #enum_ident { #ident(#SERDE_KDL_KDL_EXPORT::KdlValue) } }
        });

        Ok(())
    }

    /// Generate hints for an ignored node (slashed out with `/-`)
    /// Uses `#[deprecated]` attribute to show strikethrough in IDEs
    fn write_ignored_node_hints(node: &KdlNode, hints: &mut TokenStream2) -> syn::Result<()> {
        let node_ident = node.name.to_ident()?;

        hints.extend(quote! {
            #[deprecated(note = "slashed out with /-")]
            #[allow(non_snake_case, non_camel_case_types, unused, deprecated)]
            { struct #node_ident; }
        });

        // Generate hints for entries within the ignored node (also deprecated)
        for entry in &node.entries {
            write_ignored_entry_hints(entry, &node_ident, hints)?;
        }
        for entry in &node.ignored_entries {
            write_ignored_entry_hints(entry, &node_ident, hints)?;
        }

        // Recurse into children (all are considered ignored)
        if let Some(children) = &node.children {
            for child_node in &children.nodes {
                write_ignored_node_hints(child_node, hints)?;
            }
            for child_node in &children.ignored_nodes {
                write_ignored_node_hints(child_node, hints)?;
            }
        }
        if let Some(ignored_children) = &node.ignored_children {
            for child_node in &ignored_children.nodes {
                write_ignored_node_hints(child_node, hints)?;
            }
            for child_node in &ignored_children.ignored_nodes {
                write_ignored_node_hints(child_node, hints)?;
            }
        }

        // Generate hint for node type annotation
        if let Some(ty) = node.ty() {
            let type_token = quote_spanned!(ty.span()=> type);
            hints.extend(quote! {
                #[deprecated(note = "slashed out with /-")]
                #[allow(unused, deprecated)]
                { #type_token _Ty = (); }
            });
        }

        Ok(())
    }

    /// Generate hints for an ignored entry (slashed out with `/-`)
    /// Uses `#[deprecated]` attribute to show strikethrough in IDEs
    fn write_ignored_entry_hints(
        entry: &KdlEntry,
        node_ident: &syn::Ident,
        hints: &mut TokenStream2,
    ) -> syn::Result<()> {
        // Generate hint for values (with deprecated)
        hints.extend(ignored_value_hint(&entry.value));

        if let Some(ty) = entry.ty() {
            let type_token = quote_spanned!(ty.span()=> type);
            hints.extend(quote! {
                #[deprecated(note = "slashed out with /-")]
                #[allow(unused, deprecated)]
                { #type_token _Ty = (); }
            });
        }

        let Some(name) = &entry.name else {
            return Ok(());
        };
        let ident = name.to_ident()?;

        let enum_ident = format_ident!("{ident}", span = Span::call_site());
        hints.extend(quote! {
            #[deprecated(note = "slashed out with /-")]
            #[allow(non_snake_case, non_camel_case_types, unused, deprecated)]
            { enum #enum_ident { #ident(#SERDE_KDL_KDL_EXPORT::KdlValue) } }
        });

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

    fn ignored_value_hint(value: &KdlValue) -> TokenStream2 {
        match value {
            KdlValue::Lit(KdlLit::Null(null)) => {
                quote! {
                    #[deprecated(note = "slashed out with /-")]
                    #[allow(non_snake_case, non_upper_case_globals, unused, deprecated)]
                    { const #null: #SERDE_KDL_KDL_EXPORT::KdlValue = #SERDE_KDL_KDL_EXPORT::KdlValue::Null; }
                }
            }
            KdlValue::Lit(KdlLit::Nan(nan)) => {
                quote! {
                    #[deprecated(note = "slashed out with /-")]
                    #[allow(non_snake_case, non_upper_case_globals, unused, deprecated)]
                    { const #nan: #SERDE_KDL_KDL_EXPORT::KdlValue = #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::NAN); }
                }
            }
            KdlValue::Lit(KdlLit::Infinity(inf)) => {
                quote! {
                    #[deprecated(note = "slashed out with /-")]
                    #[allow(non_snake_case, non_upper_case_globals, unused, deprecated)]
                    { const #inf: #SERDE_KDL_KDL_EXPORT::KdlValue = #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::INFINITY); }
                }
            }
            KdlValue::Lit(KdlLit::NegInfinity(inf)) => {
                let ident = format_ident!("neg_inf", span = inf.span());
                quote! {
                    #[deprecated(note = "slashed out with /-")]
                    #[allow(non_snake_case, non_upper_case_globals, unused, deprecated)]
                    { const #ident: #SERDE_KDL_KDL_EXPORT::KdlValue = #SERDE_KDL_KDL_EXPORT::KdlValue::Float(f64::NEG_INFINITY); }
                }
            }
            _ => quote! {},
        }
    }
}
