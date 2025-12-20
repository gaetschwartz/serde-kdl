//! Tests for KDL Section 3.16: Null
//!
//! This module tests KDL null values (`#null`):
//! - Null as arguments and properties
//! - Null with type annotations
//! - Null mixed with other value types

use super::doc_to_string;
use insta::assert_snapshot;
use serde_kdl_macros::kdl;

const CRATE_NAME: &str = "my_crate";
const OPTIONAL_FEATURE: bool = true;

#[test]
fn test_slash_dashes() {
    // Test null in various contexts: as arguments, properties, and with type annotations
    let doc = kdl! {
      package name=CRATE_NAME /-version=VERSION;
      dependencies {
        nom /-"ignored" "7.0.0";
        thiserror version="1.0.0" {
          features "feature1" /-"ignored"
        }
        serde_kdl path="./" optional=OPTIONAL_FEATURE {
            "this should be ignored"
        }
      }
      /-package {
        metadata {
          "my-package" float1=#nan float2=#inf float3=#-inf null_value=#null;
        }
      }
    };
    assert_snapshot!(doc_to_string(doc), @r#"
    package name=my_crate
    dependencies {
        nom "7.0.0"
        thiserror version="1.0.0" {
            features feature1
        }
        serde_kdl path="./" optional=#true {
            "this should be ignored"
        }
    }
    "#);
}
