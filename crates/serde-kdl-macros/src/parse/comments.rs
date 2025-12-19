use proc_macro2::Span;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::*, parse::type_annotation::MaybeAnnotated};
    use quote::quote;

    #[test]
    fn parse_comment() {
        let kdl_input = quote! {
            /// This is a comment
            node key="value" 42
        };

        let parsed: Commented<crate::ast::KdlNode> =
            syn::parse2(kdl_input).expect("Failed to parse");

        assert_eq!(parsed.comments.len(), 1);
        assert_eq!(parsed.comments[0].content, " This is a comment");
        assert_eq!(parsed.item.name.value(), "node");
        assert_eq!(
            parsed.item.properties,
            vec![KdlProperty::new(KdlIdentifier::ident("key"), "value")]
        );

        assert_eq!(
            parsed.item.arguments,
            vec![MaybeAnnotated::new(KdlValue::from(42), None)]
        );
    }
}
