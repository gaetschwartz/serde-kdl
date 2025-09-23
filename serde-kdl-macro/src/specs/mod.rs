use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::Result;

mod example;

pub(crate) fn kdl_impl2(input: TokenStream2) -> Result<TokenStream2> {
    crate::kdl_impl(TokenStream::from(input))
}

#[cfg(test)]
#[macro_export]
macro_rules! assert_eq_tk {
    ($left:expr, $right:expr $(,)?) => {{
        let left = $left.to_string();
        let right = $right.to_string();
        pretty_assertions::assert_str_eq!(left, right);
    }};
}
