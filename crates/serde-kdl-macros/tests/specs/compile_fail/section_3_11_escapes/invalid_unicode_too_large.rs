// Test that Unicode escapes beyond U+10FFFF are rejected
use serde_kdl_macros::kdl;

fn main() {
    let _ = kdl! { node "\u{110000}" }; // Beyond maximum Unicode code point
}
