use insta::assert_snapshot;
use rstest::rstest;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_kdl::{from_str, to_string};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Opts {
    o1: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    o2: Option<u8>,
}

impl Opts {
    fn some(o1: u8, o2: u8) -> Self {
        Opts {
            o1: Some(o1),
            o2: Some(o2),
        }
    }
    fn none() -> Self {
        Opts { o1: None, o2: None }
    }
}

#[rstest]
#[case(42_i32, "- 42")]
#[case(i128::MAX, "- 170141183460469231731687303715884105727")]
#[case(true, "- #true")]
#[case(format!("hello"), "- hello")]
#[case(2.5_f64, "- 2.5")]
#[case(Opts::some(42, 255), "o1 42\no2 255")]
#[case(Opts::none(), "o1 #null")]
fn test_serde<S: Serialize + DeserializeOwned + PartialEq + Debug>(
    #[case] value: S,
    #[case] expected: &str,
) {
    let res = to_string(&value).unwrap();
    let expected_str = format!("{expected}\n");
    assert_eq!(
        res, expected_str,
        "Expected {expected_str:?} but got {res:?}"
    );

    let deserialized: S = from_str(&res).expect("Failed to deserialize");
    assert_eq!(
        deserialized, value,
        "Expected {value:?} but got {deserialized:?}"
    );
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Cargo {
    package: Package,
    dependencies: HashMap<String, DependencyValue>,
    features: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum DependencyValue {
    Version(String),
    Object {
        #[serde(flatten)]
        r#ref: DependencyRef,
        #[serde(skip_serializing_if = "Option::is_none")]
        features: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        optional: Option<bool>,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum DependencyRef {
    Git(String),
    Path(String),
    Version(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
}

#[test]
fn test_serde_cargo() {
    let cargo = Cargo {
        package: Package {
            name: "my_crate".to_string(),
            version: "0.1.0".to_string(),
        },
        dependencies: HashMap::from([
            (
                "nom".to_string(),
                DependencyValue::Version("8.0.0".to_string()),
            ),
            (
                "thiserror".to_string(),
                DependencyValue::Object {
                    r#ref: DependencyRef::Version("1.0.0".to_string()),
                    features: Some(vec!["feature1".to_string(), "feature2".to_string()]),
                    optional: None,
                },
            ),
            (
                "serde_kdl".to_string(),
                DependencyValue::Object {
                    r#ref: DependencyRef::Path("./".to_string()),
                    features: None,
                    optional: Some(true),
                },
            ),
        ]),
        features: vec!["feature3".to_string(), "feature4".to_string()],
    };
    let pretty = serde_kdl::to_string_pretty(&cargo).unwrap();
    assert_snapshot!(pretty, @r#"
    package {
        name my_crate
        version "0.1.0"
    }
    dependencies {
        nom "8.0.0"
        serde_kdl {
            optional #true
            path "./"
        }
        thiserror {
            features {
                - feature1
                - feature2
            }
            version "1.0.0"
        }
    }
    features {
        - feature3
        - feature4
    }
    "#);

    let parsed: Cargo = serde_kdl::from_str(&pretty).unwrap();
    assert_eq!(parsed, cargo);
}
