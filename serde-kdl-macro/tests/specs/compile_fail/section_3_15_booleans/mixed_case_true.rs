// Boolean keywords are case-sensitive: must be lowercase #true, not #True
use serde_kdl_macro::kdl;

fn main() {
    let _ = kdl! {
        node #True
    };
}
