use serde_kdl_macros::kdl;

fn main() {
    // Invalid: keyword numbers are case-sensitive
    let _ = kdl! { node #NaN };
}
