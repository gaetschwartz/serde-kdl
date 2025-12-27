use crate::parse::value::bare_identifiers;

#[allow(dead_code)]
struct SpanDisplayImpl(proc_macro2::Span);

impl std::fmt::Display for SpanDisplayImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = self.0.start();
        let end = self.0.end();

        write!(
            f,
            "{}:{}-{}:{}",
            start.line, start.column, end.line, end.column
        )
    }
}

pub struct DebugToken<'a, T>(pub &'a T);

impl<T> std::fmt::Debug for DebugToken<'_, T>
where
    T: HasSpan,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            std::any::type_name::<T>(),
            SpanDisplayImpl(self.0.span())
        )
    }
}

pub struct DebugSynToken<'a, T>(pub &'a T);

impl<T> std::fmt::Debug for DebugSynToken<'_, T>
where
    T: syn::spanned::Spanned,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            std::any::type_name::<T>(),
            SpanDisplayImpl(self.0.span())
        )
    }
}

pub trait DisplaySpan {
    fn display(&self) -> impl std::fmt::Display;
}

impl<T> DisplaySpan for T
where
    T: HasSpan,
{
    fn display(&self) -> impl std::fmt::Display {
        SpanDisplayImpl(self.span())
    }
}

pub trait HasSpan {
    fn span(&self) -> proc_macro2::Span;
}
impl HasSpan for proc_macro2::Span {
    fn span(&self) -> proc_macro2::Span {
        *self
    }
}
macro_rules! impl_hasspan_for_syn_spanned {
    ($($ty:ty),*) => {
        $(
            impl HasSpan for $ty {
                fn span(&self) -> proc_macro2::Span {
                  syn::spanned::Spanned::span(&self)
                }
            }
        )*
    };
}
impl_hasspan_for_syn_spanned!(
    syn::Ident,
    syn::Type,
    syn::Expr,
    syn::Pat,
    syn::Item,
    syn::Attribute,
    syn::Lit,
    syn::LitBool,
    syn::LitStr,
    syn::LitInt,
    syn::LitFloat,
    syn::Path,
    syn::token::Pound,
    bare_identifiers::inf,
    bare_identifiers::nan,
    bare_identifiers::null
);
