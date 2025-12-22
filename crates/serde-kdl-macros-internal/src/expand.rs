//! Code generation for KDL documents
//!
//! This module handles the generation of Rust code from parsed KDL AST structures.

use crate::{
    ast::{KDL_NODE, SERDE_KDL_KDL_EXPORT},
    parse::{
        comments::MaybeSlashed, document::KdlDocument, node::KdlNode,
        type_annotation::IntoSetTypeAnnotation as _, value::KdlLit,
    },
};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Result, spanned::Spanned as _};

pub fn generate_kdl_code(document: &KdlDocument) -> Result<TokenStream2> {
    let nodes = document.nodes().map(expand_node_creation);
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
    let entries = node.entries().map(|prop| {
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
    let children_code = node.children().flatten().as_ref().map(|children| {
        let child_codes = children.nodes().map(expand_node_creation);
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

#[cfg(not(feature = "ide-hints"))]
mod ide_hints {
    use super::*;

    pub fn write_hints(_document: &KdlDocument) -> syn::Result<TokenStream2> {
        Ok(quote! {})
    }
}

#[cfg(feature = "ide-hints")]
mod ide_hints {
    use super::*;
    use crate::parse::{
        entry::KdlEntry,
        value::{KdlValue, MaybeMinus, PoundLiteral},
    };
    use quote::{ToTokens, quote_spanned};

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
        for node in &document.nodes {
            match node {
                MaybeSlashed::Item(n) => write_node_hints(n, hints)?,
                MaybeSlashed::Slashed(sd, n) => {
                    write_slashdash_hint(sd, hints)?;
                    write_ignored_node_hints(n, hints)?;
                }
            }
        }

        Ok(())
    }

    fn write_node_hints(node: &KdlNode, hints: &mut TokenStream2) -> syn::Result<()> {
        // Generate hint for node names that are identifiers
        let node_ident = node.name.to_ident()?;
        hints.extend(to_struct_hint(&node_ident));

        // Generate hint for active entries
        for entry in &node.entries {
            match entry {
                MaybeSlashed::Item(e) => {
                    write_entry_hints(e, &node_ident, hints)?;
                }
                MaybeSlashed::Slashed(sd, e) => {
                    write_slashdash_hint(sd, hints)?;
                    write_ignored_entry_hints(e, &node_ident, hints)?;
                }
            }
        }

        // Recurse into active children
        if let Some(children) = &node.children {
            match children {
                MaybeSlashed::Item(c) => {
                    for child_node in &c.nodes {
                        match child_node {
                            MaybeSlashed::Item(n) => write_node_hints(n, hints)?,
                            MaybeSlashed::Slashed(sd, n) => {
                                write_slashdash_hint(sd, hints)?;
                                write_ignored_node_hints(n, hints)?;
                            }
                        }
                    }
                }
                MaybeSlashed::Slashed(sd, c) => {
                    write_slashdash_hint(sd, hints)?;
                    for child_node in &c.nodes {
                        write_ignored_node_hints(child_node.inner(), hints)?;
                    }
                }
            }
        }

        // Generate hint for node type annotation
        if let Some(ty) = node.ty() {
            let type_token = quote_spanned!(ty.span()=> type);
            hints.extend(to_type_hint(type_token));
        }

        Ok(())
    }

    fn write_entry_hints(
        entry: &KdlEntry,
        _node_ident: &syn::Ident,
        hints: &mut TokenStream2,
    ) -> syn::Result<()> {
        // Generate hint for values
        hints.extend(value_hint(&entry.value));

        if let Some(ty) = entry.ty() {
            let type_token = quote_spanned!(ty.span()=> type);
            hints.extend(to_type_hint(type_token));
        }

        if let Some(name) = &entry.name {
            let ident = name.to_ident()?;
            hints.extend(to_enum_hint(&ident));
        }

        Ok(())
    }

    /// Generate hints for an ignored node (slashed out with `/-`)
    /// Uses `#[deprecated]` attribute to show strikethrough in IDEs
    fn write_ignored_node_hints(node: &KdlNode, hints: &mut TokenStream2) -> syn::Result<()> {
        let node_ident = node.name.to_ident()?;

        hints.extend(to_arg_hint(&node_ident));

        // Generate hint for node type annotation
        if let Some(ty) = node.ty() {
            let type_token = ty.to_ident()?;
            hints.extend(to_arg_hint(type_token));
        }

        // Generate hints for entries within the ignored node (also deprecated)
        for entry in &node.entries {
            let entry = entry.inner();
            write_ignored_entry_hints(entry, &node_ident, hints)?;
        }

        // Recurse into children (all are considered ignored)
        if let Some(children) = &node.children {
            let children = children.inner();
            for child_node in &children.nodes {
                let child_node = child_node.inner();
                write_ignored_node_hints(child_node, hints)?;
            }
        }

        Ok(())
    }

    /// Generate hints for an ignored entry (slashed out with `/-`)
    /// Uses `#[deprecated]` attribute to show strikethrough in IDEs
    fn write_ignored_entry_hints(
        entry: &KdlEntry,
        _node_ident: &syn::Ident,
        hints: &mut TokenStream2,
    ) -> syn::Result<()> {
        // Generate hint for values (with deprecated)
        hints.extend(ignored_value_hint(&entry.value));

        if let Some(ty) = entry.ty() {
            let type_token = ty.to_ident()?;
            hints.extend(to_arg_hint(type_token));
        }

        if let Some(name) = &entry.name {
            let ident = name.to_ident()?;
            hints.extend(to_arg_hint(&ident));
        }

        Ok(())
    }

    fn ignored_value_hint(value: &KdlValue) -> TokenStream2 {
        let span = value.span();
        let ident = format_ident!("ignored", span = span);
        to_arg_hint(ident)
    }

    fn value_hint(value: &KdlValue) -> TokenStream2 {
        match value {
            KdlValue::Lit(KdlLit::Null(PoundLiteral { value, .. })) => {
                to_const_kdl_value_hint(value, quote! { Null })
            }
            KdlValue::Lit(KdlLit::Nan(PoundLiteral { value, .. })) => {
                to_const_kdl_value_hint(value, quote! { Float(f64::NAN) })
            }
            KdlValue::Lit(KdlLit::Infinity(PoundLiteral {
                value: MaybeMinus {
                    minus: None, value, ..
                },
                ..
            })) => to_const_kdl_value_hint(value, quote! { Float(f64::INFINITY) }),
            KdlValue::Lit(KdlLit::Infinity(PoundLiteral {
                value:
                    MaybeMinus {
                        minus: Some(minus),
                        value,
                    },
                ..
            })) => {
                let minus_hint = to_const_value_hint(
                    format_ident!("minus", span = minus.span()),
                    quote! { () },
                    quote! { () },
                );
                let ident = format_ident!("neg_inf", span = value.span());
                let main_hint = to_const_kdl_value_hint(ident, quote! { Float(f64::NEG_INFINITY) });
                quote! {
                    #minus_hint
                    #main_hint
                }
            }
            _ => quote! {},
        }
    }

    fn to_struct_hint(ident: impl ToTokens) -> TokenStream2 {
        quote! {
            #[allow(non_snake_case, non_camel_case_types, unused)]
            { struct #ident; }
        }
    }

    fn to_enum_hint(variant: impl ToTokens) -> TokenStream2 {
        quote! {
            #[allow(non_snake_case, non_camel_case_types, unused)]
            { enum ide_hints { #variant(#SERDE_KDL_KDL_EXPORT::KdlValue) } }
        }
    }

    fn to_type_hint(ty: impl ToTokens) -> TokenStream2 {
        let type_token = quote_spanned!(ty.span()=> type);
        quote! {
            #[allow(unused, deprecated)]
            { #type_token _Ty = (); }
        }
    }

    fn to_arg_hint(arg: impl ToTokens) -> TokenStream2 {
        quote! {
            #[allow(non_snake_case, non_upper_case_globals, unused)]
            { fn ide_hints(#arg: ()) {} }
        }
    }

    fn to_const_kdl_value_hint(const_ident: impl ToTokens, value: impl ToTokens) -> TokenStream2 {
        quote! {
            #[allow(non_snake_case, non_upper_case_globals, unused)]
            { const #const_ident: #SERDE_KDL_KDL_EXPORT::KdlValue = #SERDE_KDL_KDL_EXPORT::KdlValue::#value; }
        }
    }
    fn to_const_value_hint(
        const_ident: impl ToTokens,
        ty: impl ToTokens,
        value: impl ToTokens,
    ) -> TokenStream2 {
        quote! {
            #[allow(non_snake_case, non_upper_case_globals, unused)]
            { const #const_ident: #ty = #value; }
        }
    }

    fn write_slashdash_hint(
        slashdash: &crate::parse::comments::SlashDash,
        hints: &mut TokenStream2,
    ) -> syn::Result<()> {
        let (slash_span, dash_span) = slashdash.spans();
        let slash_ident = format_ident!("slash", span = slash_span);
        let dash_ident = format_ident!("dash", span = dash_span);
        hints.extend(to_arg_hint(slash_ident));
        hints.extend(to_arg_hint(dash_ident));
        Ok(())
    }
}
