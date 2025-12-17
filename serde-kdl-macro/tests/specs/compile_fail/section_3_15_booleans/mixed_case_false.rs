// Boolean keywords are case-sensitive: must be lowercase #false, not #False
use serde_kdl_macro::kdl;

fn main() {
    let _ = kdl! {
        node #False
    };
}
