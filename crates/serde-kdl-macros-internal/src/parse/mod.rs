//! Parsing modules for KDL components
//!
//! This module contains submodules for parsing different KDL components:
//! - `document`: Document-level parsing
//! - `node`: Node parsing
//! - `value`: Value parsing
//! - `string`: String parsing and validation
//! - `type_annotation`: Type annotation parsing
//! - `number`: Number parsing (Section 3.14)

#[allow(unused)]
pub mod comments;
pub mod document;
pub mod entry;
pub mod identifier;
pub mod node;
pub mod type_annotation;
pub mod value;
#[allow(unused)]
pub mod whitespace;

#[allow(dead_code)]
pub struct SpanDisplay(pub proc_macro2::Span);

impl std::fmt::Display for SpanDisplay {
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

pub struct DebugToken<'a, T>(&'a T);

impl<'a, T> std::fmt::Debug for DebugToken<'a, T>
where
    T: HasSpan,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            std::any::type_name::<T>(),
            SpanDisplay(self.0.span())
        )
    }
}

pub trait HasSpan {
    fn span(&self) -> proc_macro2::Span;
}
impl<T> HasSpan for T
where
    T: syn::spanned::Spanned,
{
    fn span(&self) -> proc_macro2::Span {
        self.span()
    }
}
