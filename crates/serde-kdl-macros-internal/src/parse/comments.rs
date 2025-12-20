use proc_macro2::Span;
use syn::Token;

struct Commented<T> {
    pub comments: Vec<Comment>,
    pub item: T,
}

struct Comment {
    pub content: String,
    pub span: Span,
}

impl<T: syn::parse::Parse> syn::parse::Parse for Commented<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attributes = input.call(syn::Attribute::parse_outer)?;
        let comments = attributes
            .into_iter()
            .filter(|a| a.path().is_ident("doc"))
            .filter_map(|a| match &a.meta {
                syn::Meta::NameValue(nv) => {
                    if let syn::Expr::Lit(syn::PatLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = &nv.value
                    {
                        Some(Comment {
                            content: s.value(),
                            span: s.span(),
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        let item: T = input.parse()?;

        Ok(Commented { comments, item })
    }
}

#[derive(Debug, Clone)]
pub enum MaybeSlashed<T> {
    /// An item preceded by `/-` so it's commented out
    Slashed,
    /// A plain item without any slashing
    Item(T),
}

impl<T: syn::parse::Parse> syn::parse::Parse for MaybeSlashed<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let slashed = if input.peek(Token![/]) && input.peek2(Token![-]) {
            let _slash: Token![/] = input.parse()?;
            let _dash: Token![-] = input.parse()?;
            Some(())
        } else {
            None
        };
        let item = input.parse::<T>()?;
        if slashed.is_some() {
            Ok(MaybeSlashed::Slashed)
        } else {
            Ok(MaybeSlashed::Item(item))
        }
    }
}

impl<T> From<MaybeSlashed<T>> for Option<T> {
    fn from(value: MaybeSlashed<T>) -> Self {
        match value {
            MaybeSlashed::Slashed => None,
            MaybeSlashed::Item(item) => Some(item),
        }
    }
}

impl<T> MaybeSlashed<T> {
    pub fn is_slashed(&self) -> bool {
        matches!(self, MaybeSlashed::Slashed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::*,
        parse::{
            entry::KdlEntry,
            identifier::KdlIdentifier,
            node::{ChildrenBlock, KdlNode},
            type_annotation::MaybeAnnotated,
            value::KdlValue,
        },
    };
    use quote::quote;

    #[test]
    fn parse_comment() {
        let kdl_input = quote! {
            /// This is a comment
            node key="value" 42
        };

        let parsed: Commented<KdlNode> = syn::parse2(kdl_input).expect("Failed to parse");

        assert_eq!(parsed.comments.len(), 1);
        assert_eq!(parsed.comments[0].content, " This is a comment");
        assert_eq!(parsed.item.name.value(), "node");
        assert_eq!(
            parsed.item.entries,
            vec![
                KdlEntry::new_prop(KdlIdentifier::ident_test("key"), "value"),
                KdlEntry::new(KdlValue::from(42))
            ]
        );
    }

    #[test]
    fn parse_slashed_literal() {
        let kdl_input = quote! {
            /- "This is a slashed value"
        };

        let parsed: MaybeSlashed<syn::LitStr> = syn::parse2(kdl_input).expect("Failed to parse");

        assert!(parsed.is_slashed());
    }

    #[test]
    fn parse_slashed_children_block() {
        let kdl_input = quote! {
            /- {
                child key="value";
            }
        };

        let parsed: MaybeSlashed<ChildrenBlock> = syn::parse2(kdl_input).expect("Failed to parse");

        assert!(parsed.is_slashed());
    }
}
