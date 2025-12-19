// Boolean keywords are case-sensitive: must be lowercase #false, not #FALSE
use serde_kdl_macros::kdl;

fn main() {
    let _ = kdl! {
        node #FALSE
    };
}
