use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::Result;

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

#[cfg(test)]
mod section_3_1_test;

#[cfg(test)]
mod section_3_2_test;

#[cfg(test)]
mod section_3_3_test;

#[cfg(test)]
mod section_3_4_test;

#[cfg(test)]
mod section_3_5_test;

#[cfg(test)]
mod section_3_6_test;

#[cfg(test)]
mod section_3_7_test;

#[cfg(test)]
mod section_3_8_test;

#[cfg(test)]
mod section_3_9_test;

#[cfg(test)]
mod section_3_10_test;

#[cfg(test)]
mod section_3_11_test;

#[cfg(test)]
mod section_3_12_test;

#[cfg(test)]
mod section_3_13_test;

#[cfg(test)]
mod section_3_14_test;

#[cfg(test)]
mod section_3_15_test;

#[cfg(test)]
mod section_3_16_test;

#[cfg(test)]
mod section_3_17_test;

#[cfg(test)]
mod section_3_18_test;

#[cfg(test)]
mod section_3_19_test;

#[cfg(test)]
mod section_4_1_test;