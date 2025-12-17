use serde_kdl_macro::kdl;

fn main() {
    // Invalid: keyword numbers are case-sensitive
    let _ = kdl! { node #NaN };
}
