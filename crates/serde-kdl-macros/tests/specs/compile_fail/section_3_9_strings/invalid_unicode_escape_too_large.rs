//! Test that Unicode escapes with code points beyond U+10FFFF fail to compile
//!
//! According to the Unicode standard, valid code points are U+0000 to U+10FFFF.
//! This tests that code points beyond this range are rejected.

use serde_kdl_macros::kdl;

fn main() {
    let _ = kdl! {
        node "\u{110000}"
    };
}
