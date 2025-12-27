use crate::parse::node::ChildrenBlock;
use crate::utils::HasSpan;
use proc_macro2::Span;
use syn::Token;
use syn::parse::ParseStream;

struct Commented<T> {
    pub comments: Vec<Comment>,
    pub item: T,
}

struct Comment {
    pub content: String,
    pub span: Span,
}

impl<T: syn::parse::Parse> syn::parse::Parse for Commented<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
    /// An item preceded by `/-` so it's commented out (but still parsed)
    Slashed(SlashDash, T),
    /// A plain item without any slashing
    Item(T),
}

impl<T: syn::parse::Parse> syn::parse::Parse for MaybeSlashed<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let slash_dash = if SlashDash::peek(input) {
            Some(input.parse::<SlashDash>()?)
        } else {
            None
        };
        let item_span_start = input.span().start();
        let item = input.parse::<T>()?;
        if let Some(slash_dash) = slash_dash {
            if syn::spanned::Spanned::span(&slash_dash.1).end().line != item_span_start.line {
                return Err(syn::Error::new(
                    syn::spanned::Spanned::span(&slash_dash.1),
                    "Expected slashed item to be on the same line as `/-`",
                ));
            }
            Ok(MaybeSlashed::Slashed(slash_dash, item))
        } else {
            Ok(MaybeSlashed::Item(item))
        }
    }
}

impl<T> From<MaybeSlashed<T>> for Option<T> {
    fn from(value: MaybeSlashed<T>) -> Self {
        match value {
            MaybeSlashed::Slashed(_, _) => None,
            MaybeSlashed::Item(item) => Some(item),
        }
    }
}

impl<T> MaybeSlashed<T> {
    /// Returns true if the item is slashed (commented out with `/-`)
    pub fn is_slashed(&self) -> bool {
        matches!(self, MaybeSlashed::Slashed(_, _))
    }

    /// Returns a reference to the inner item, regardless of whether it's slashed
    pub fn inner(&self) -> &T {
        match self {
            MaybeSlashed::Slashed(_, item) | MaybeSlashed::Item(item) => item,
        }
    }

    pub fn as_option(&self) -> Option<&T> {
        match self {
            MaybeSlashed::Slashed(_, _) => None,
            MaybeSlashed::Item(item) => Some(item),
        }
    }
}

impl MaybeSlashed<ChildrenBlock> {
    pub fn peek(input: ParseStream<'_>) -> bool {
        if SlashDash::peek(input) {
            input.peek3(syn::token::Brace)
        } else {
            input.peek(syn::token::Brace)
        }
    }
}

impl<T: HasSpan> HasSpan for MaybeSlashed<T> {
    fn span(&self) -> Span {
        match self {
            MaybeSlashed::Slashed(slash_dash, item) => {
                slash_dash.span().join(item.span()).unwrap_or(item.span())
            }
            MaybeSlashed::Item(item) => item.span(),
        }
    }
}

/// Represents a KDL slash-dash comment marker (`/-`)
///
/// This handles both joint tokens (from actual macro invocations) and
/// separate tokens (from `quote!` macro in tests).
#[derive(Clone, Copy)]
pub struct SlashDash(Token![/], Token![-]);

impl SlashDash {
    /// Check if the next tokens are `/-`
    pub fn peek(input: ParseStream<'_>) -> bool {
        input.peek(Token![/]) && input.peek2(Token![-])
    }

    /// Returns the span covering both the slash and dash tokens
    #[must_use]
    pub fn spans(&self) -> (Span, Span) {
        (
            syn::spanned::Spanned::span(&self.0),
            syn::spanned::Spanned::span(&self.1),
        )
    }
}

impl syn::parse::Parse for SlashDash {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let slash = input.parse::<Token![/]>()?;
        let dash = input.parse::<Token![-]>()?;
        if syn::spanned::Spanned::span(&slash).end().line
            != syn::spanned::Spanned::span(&dash).start().line
        {
            return Err(syn::Error::new(
                syn::spanned::Spanned::span(&dash),
                "Expected `/-` to be on the same line",
            ));
        }
        Ok(SlashDash(slash, dash))
    }
}

impl HasSpan for SlashDash {
    fn span(&self) -> Span {
        let (start, end) = self.spans();
        start.join(end).unwrap_or(start)
    }
}

impl std::fmt::Debug for SlashDash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SlashDash").finish()
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
            parsed.item.entries().collect::<Vec<_>>(),
            vec![
                &KdlEntry::new_prop(KdlIdentifier::ident_test("key"), "value"),
                &KdlEntry::new(KdlValue::from(42))
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
        // Verify we can still access the slashed content
        assert_eq!(parsed.inner().value(), "This is a slashed value");
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
