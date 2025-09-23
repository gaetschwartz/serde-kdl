use proc_macro2::TokenStream as TokenStream2;
use syn::Result;

pub(crate) fn kdl_impl2(input: TokenStream2) -> Result<TokenStream2> {
    // Preprocess input to handle line continuations
    let processed_input = preprocess_line_continuations_proc_macro2(input)?;
    let document = syn::parse2::<crate::ast::KdlDocument>(processed_input)?;

    crate::codegen::generate_kdl_code(&document)
}

/// Preprocesses the input TokenStream2 to handle KDL line continuations for testing.
fn preprocess_line_continuations_proc_macro2(input: TokenStream2) -> Result<TokenStream2> {
    let input_str = input.to_string();
    let processed_str = crate::utils::process_line_continuation_string(&input_str)?;

    // Parse the processed string back into a TokenStream2
    match processed_str.parse() {
        Ok(tokens) => Ok(tokens),
        Err(e) => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Failed to parse processed KDL: {}", e),
        )),
    }
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

// #[cfg(test)]
// mod section_3_1_test;

// #[cfg(test)]
// mod section_3_2_test;

#[cfg(test)]
mod section_3_3;

// #[cfg(test)]
// mod section_3_4_test;

// #[cfg(test)]
// mod section_3_5_test;

// #[cfg(test)]
// mod section_3_6_test;

#[cfg(test)]
mod section_3_7;

#[cfg(test)]
mod section_3_8;

#[cfg(test)]
mod section_3_9;

#[cfg(test)]
mod section_3_10;

#[cfg(test)]
mod section_3_11;

#[cfg(test)]
mod section_3_12;

#[cfg(test)]
mod section_3_13;

// #[cfg(test)]
// mod section_3_14;

#[cfg(test)]
mod section_3_15;

#[cfg(test)]
mod section_3_16;

#[cfg(test)]
mod section_3_17;

#[cfg(test)]
mod section_3_18;

#[cfg(test)]
mod section_3_19;

// #[cfg(test)]
// mod section_4_1_test;
