//! Test that invalid escape sequences fail to compile
//!
//! Only specific escape sequences are valid in KDL strings.

use serde_kdl_macros::kdl;

fn main() {
    let _ = kdl! {
        node "invalid \x escape"
    };
}
