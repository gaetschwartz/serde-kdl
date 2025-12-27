//! A KDL macro that allows writing KDL syntax directly in Rust code.
//!
//! This crate provides a procedural macro for embedding KDL (KDL Document Language)
//! syntax directly in Rust source code. The macro parses the KDL at compile time
//! and generates the corresponding kdl crate data structures.

use proc_macro2::TokenStream as TokenStream2;

use crate::expand::generate_kdl_code;

// Module declarations
pub mod ast;
pub mod expand;
pub mod parse;
pub mod utils;
pub mod validation;

#[macro_export]
macro_rules! trace {
    (name = $name:expr, value = ?$value:expr, tt = $tt:expr) => {
        $crate::trace!(name = $name, value = format_args!("{:?}", $value), span = $crate::utils::DisplaySpan::display(&$value), tt = $tt);
    };
    (name = $name:expr, value = ?$value:expr, tt = $tt:expr, $($arg:tt)*) => {
        $crate::trace!(name = $name, value = format_args!("{:?}", $value), span = $crate::utils::DisplaySpan::display(&$value), tt = $tt, $($arg)*);
    };
    (name = $name:expr, value = $value:expr, tt = $tt:expr) => {
        $crate::trace!(name = $name, value = format_args!("{:?}", $value), span = $crate::utils::DisplaySpan::display(&$value), tt = $tt);
    };
    (name = $name:expr, value = $value:expr, tt = $tt:expr, $($arg:tt)*) => {
        $crate::trace!(name = $name, value = format_args!("{:?}", $value), span = $crate::utils::DisplaySpan::display(&$value), tt = $tt, $($arg)*);
    };
    (name = $name:expr, value = $value:expr, span = $span:expr, tt = $tt:expr) => {
        $crate::trace!("[{name}] value: {value}\n{indent} span: {span} \n{indent} tt: `{tt}`", name = $name, value = $value, span = $span, tt = $tt, indent = " ".repeat($name.to_string().len() + 2));
    };
    (name = $name:expr, value = $value:expr, span = $span:expr, tt = $tt:expr, $($arg:tt)*) => {
        $crate::trace!("[{name}] {args}\n{indent} value: {value}\n{indent} span: {span} \n{indent} tt: `{tt}`", name = $name, args = format_args!($($arg)*), value = $value, span = $span, tt = $tt, indent = " ".repeat($name.to_string().len() + 2));
    };
    (name = $name:expr, tt = $tt:expr, $($arg:tt)*) => {
        $crate::trace!("[{name}] {args} \n{indent} tt: `{tt}`", name = $name, args = format_args!($($arg)*), tt = $tt, indent = " ".repeat($name.to_string().len() + 2));
    };
    (name = $name:expr, $($arg:tt)*) => {
        $crate::trace!("[{name}] {args}",name = $name, args = format_args!($($arg)*));
    };
    ($($arg:tt)*) => {
        #[cfg(kdl_macros_debug)]
        {
            eprintln!($($arg)*);
        }
        #[cfg(not(kdl_macros_debug))]
        {
            let _ = format_args!($($arg)*);
        }
    };
}

pub fn kdl_impl(input: TokenStream2) -> syn::Result<TokenStream2> {
    let document = syn::parse2::<parse::document::KdlDocument>(input)?;
    trace!("Parsed KDL Document: {:#?}", document);
    generate_kdl_code(&document)
}
