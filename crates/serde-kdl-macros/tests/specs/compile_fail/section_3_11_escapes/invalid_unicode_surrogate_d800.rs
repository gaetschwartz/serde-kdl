// Test that Unicode surrogate code points (U+D800-U+DFFF) are rejected
use serde_kdl_macros::kdl;

fn main() {
    let _ = kdl! { node "\u{D800}" }; // Surrogate code point (not a valid Unicode scalar)
}
