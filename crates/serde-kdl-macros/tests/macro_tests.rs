#![allow(clippy::approx_constant)]

use insta::assert_snapshot;
use kdl::KdlDocument;
use serde_kdl_macros::kdl;

#[path = "specs/mod.rs"]
mod specs;

mod complex {
    use super::*;

    #[test]
    fn test_complex_cargo_macro() {
        // Test using a variable in the macro
        const CRATE_NAME: &str = "my_crate";
        const VERSION: &str = "0.1.0";
        const OPTIONAL_FEATURE: bool = true;

        let doc = kdl! {
          package name=CRATE_NAME version=VERSION
          dependencies {
            nom "8.0.0"
            thiserror version="1.0.0" {
              features "feature1" "feature2"
            }
            serde_kdl path="./" optional=OPTIONAL_FEATURE
          }
          features "feature3" "feature4"
          package {
            metadata {
              "my-package" float1=#nan float2=#inf float3=#-inf null_value=#null
            }
          }
        };

        assert_snapshot!(doc_to_pretty_string(doc), @r#"
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

    #[test]
    fn test_identifier_with_only_disallowed_characters() {
        let doc = kdl! {
          "node" "property"="value"
        };
        assert_snapshot!(doc_to_pretty_string(doc), @"node property=value");
    }
}

mod variable_tests {
    use super::*;

    #[test]
    fn test_variable_as_property_value() {
        let my_string = "hello";
        let doc = kdl! {
            node prop=my_string
        };
        assert_snapshot!(doc, @"node prop=hello");
    }

    #[test]
    fn test_variable_as_argument() {
        let my_value: i128 = 42;
        let doc = kdl! {
            node my_value
        };
        assert_snapshot!(doc, @"node 42");
    }

    #[test]
    fn test_multiple_variables() {
        let name = "test";
        let count: i128 = 5;
        let enabled = true;
        let doc = kdl! {
            config name=name count=count enabled=enabled
        };
        assert_snapshot!(doc, @"config name=test count=5 enabled=#true");
    }

    #[test]
    fn test_variable_in_nested_node() {
        let inner_val = "nested";
        let doc = kdl! {
            parent {
                child value=inner_val
            }
        };
        assert_snapshot!(doc_to_pretty_string(doc), @r#"
        parent {
            child value=nested
        }
        "#);
    }
}

mod basic_usage_tests {
    use super::*;

    #[test]
    fn test_simple_node() {
        let doc = kdl! {
            simple_node 42
        };

        assert_snapshot!(doc, @"simple_node 42")
    }

    #[test]
    fn test_node_with_string() {
        let doc = kdl! {
            config "my-app"
        };

        assert_snapshot!(doc, @"config my-app")
    }

    #[test]
    fn test_node_with_boolean() {
        let doc = kdl! {
            debug #true
        };

        assert_snapshot!(doc, @"debug #true")
    }

    #[test]
    fn test_node_with_float() {
        let doc = kdl! {
            version 1.5
        };

        assert_snapshot!(doc, @"version 1.5")
    }

    #[test]
    fn test_multiple_arguments() {
        let doc = kdl! {
            connect "localhost" 5432 #true
        };

        assert_snapshot!(doc, @"connect localhost 5432 #true")
    }

    #[test]
    fn test_node_with_properties() {
        let doc = kdl! {
            server host="localhost" port=8080
        };

        assert_snapshot!(doc, @"server host=localhost port=8080")
    }

    #[test]
    fn test_mixed_args_and_properties() {
        let doc = kdl! {
            database "postgres" version=13 ssl=#true
        };

        assert_snapshot!(doc, @"database postgres version=13 ssl=#true")
    }
}

mod nested_structure_tests {
    use super::*;

    #[test]
    fn test_empty_children_block() {
        let doc = kdl! {
            parent {}
        };

        assert_snapshot!(doc, @r"
        parent{
        }
        ")
    }

    #[test]
    fn test_simple_nested_structure() {
        let doc = kdl! {
            config {
                name "my-app"
                debug #true
            }
        };

        assert_snapshot!(doc, @r"
        config{
        name my-app
        debug #true
        }
        ")
    }

    #[test]
    fn test_nested_with_parent_properties() {
        let doc = kdl! {
            server host="localhost" port=8080 {
                ssl #true
                timeout 30
            }
        };

        assert_snapshot!(doc, @r"
        server host=localhost port=8080{
        ssl #true
        timeout 30
        }
        ")
    }
}

mod edge_case_tests {
    use super::*;

    #[test]
    fn test_negative_numbers() {
        let doc = kdl! {
            negative -42
            negative_float -3.14
        };

        assert_snapshot!(doc, @r"
        negative -42
        negative_float -3.14
        ")
    }

    #[test]
    fn test_empty_string() {
        let doc = kdl! {
            empty_string ""
        };

        assert_snapshot!(doc, @r#"empty_string """#)
    }

    #[test]
    fn test_zero_values() {
        let doc = kdl! {
            zero 0
            zero_float 0.0
        };

        assert_snapshot!(doc, @r"
        zero 0
        zero_float 0.0
        ")
    }
}

fn doc_to_pretty_string(mut doc: KdlDocument) -> String {
    doc.autoformat();
    doc.to_string()
}
