// Boolean keywords are case-sensitive: must be lowercase #true, not #True
use serde_kdl_macros::kdl;

fn main() {
    let _ = kdl! {
        node #True
    };
}
