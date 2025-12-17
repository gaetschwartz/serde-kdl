use serde_kdl_macro::kdl;

fn main() {
    // Invalid characters in type annotation - special chars not allowed without quotes
    let _ = kdl! { node (my@type)"value" };
}
