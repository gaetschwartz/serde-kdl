// Test that Unicode surrogate code points (U+D800-U+DFFF) are rejected
use serde_kdl_macro::kdl;

fn main() {
    let _ = kdl! { node "\u{DFFF}" }; // Surrogate code point (not a valid Unicode scalar)
}
