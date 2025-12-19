//! Test that malformed Unicode escapes fail to compile
//!
//! Unicode escapes must be in the form \u{...} with valid hex digits.

use serde_kdl_macros::kdl;

fn main() {
    let _ = kdl! {
        node "\u{GGGG}"
    };
}
