//! Tests for KDL Section 3.7: Value
//!
//! According to the KDL specification Section 3.7:
//! - A value is either: String (Section 3.9), Number (Section 3.14), Boolean (Section 3.15), or Null (Section 3.16)
//! - Values MUST be either Arguments (Section 3.5) or values of Properties (Section 3.4)
//! - Only String values may be used as Node names or Property keys
//! - Values (both as arguments and in properties) MAY be prefixed by a single Type Annotation (Section 3.8)

use super::doc_to_string;
use insta::assert_snapshot;
use serde_kdl_macros::kdl;

// Test using a variable in the macro
const CRATE_NAME: &str = "my_crate";
const VERSION: &str = "0.1.0";
const OPTIONAL_FEATURE: bool = true;

#[test]
fn test_semicolon_separated_nodes() {
    let doc = kdl! {
      package name=CRATE_NAME version=VERSION;
      dependencies {
        nom "8.0.0";
        thiserror version="1.0.0" {
          features "feature1" "feature2"
        }
        serde_kdl path="./" optional=OPTIONAL_FEATURE
      }
      features "feature3" "feature4";
      package {
        metadata {
          "my-package" float1=#nan float2=#inf float3=#-inf null_value=#null;
        }
      }
    };

    assert_snapshot!(doc_to_string(doc), @r#"
    package name=my_crate version="0.1.0"
    dependencies {
        nom "8.0.0"
        thiserror version="1.0.0" {
            features feature1 feature2
        }
        serde_kdl path="./" optional=#true
    }
    features feature3 feature4
    package {
        metadata {
            my-package float1=#nan float2=#inf float3=#-inf null_value=#null
        }
    }
    "#);
}
