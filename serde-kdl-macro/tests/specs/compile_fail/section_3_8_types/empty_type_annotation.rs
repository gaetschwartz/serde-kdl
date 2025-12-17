use serde_kdl_macro::kdl;

fn main() {
    // Empty type annotation - type name is required
    let _ = kdl! { node ()"value" };
}
