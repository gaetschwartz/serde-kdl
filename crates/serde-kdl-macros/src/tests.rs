use std::ops::Bound;

use bolero::{Driver, TypeGenerator, check};
use serde_kdl_macros_internal::parse::document::KdlDocument;

#[test]
fn test_parse_any_kdl_doc() {
    check!()
        .with_type::<KdlTestDoc>()
        .for_each(|KdlTestDoc(doc)| {
            let kdl_str = doc.to_string();

            let tokenstream = match syn::parse_str::<proc_macro2::TokenStream>(&kdl_str) {
                Ok(tokenstream) => tokenstream,
                Err(_) => {
                    // eprintln!("Failed to parse generated KDL string into TokenStream: {e}\nSource:\n{kdl_str}",);
                    return;
                }
            };

            let parsed = syn::parse2::<KdlDocument>(tokenstream).unwrap_or_else(|e| {
                panic!("Failed to parse generated KDL string: {e}\nSource:\n{kdl_str}",)
            });
            assert_eq!(
                &parsed, doc,
                "Roundtrip mismatch for KDL string:\n{kdl_str}"
            );
        });
}

#[derive(Debug)]
struct KdlTestDoc(kdl::KdlDocument);

impl TypeGenerator for KdlTestDoc {
    fn generate<D: Driver>(driver: &mut D) -> Option<Self> {
        let num_nodes: u8 = gen_in_range(driver, 0..=3)?;

        let mut doc = kdl::KdlDocument::new();

        *doc.nodes_mut() = (0..num_nodes)
            .map(|_| gen_node(driver))
            .collect::<Option<Vec<_>>>()?;

        Some(KdlTestDoc(doc))
    }
}

fn gen_node<D: Driver>(driver: &mut D) -> Option<kdl::KdlNode> {
    let node_name = gen_identifier(driver)?;
    let mut node = kdl::KdlNode::new(node_name);

    let entries_count = gen_in_range(driver, 0..=6)?;

    let num_props: u8 = gen_in_range(driver, 0..=entries_count)?;
    for _ in 0..num_props {
        node.entries_mut().push(gen_entry(driver, true)?);
    }
    let num_args: u8 = entries_count - num_props;
    for _ in 0..num_args {
        node.entries_mut().push(gen_entry(driver, false)?);
    }

    let has_child_nodes: bool = driver.gen_bool(None)?;
    if has_child_nodes {
        let KdlTestDoc(child_doc) = KdlTestDoc::generate(driver)?;
        node.set_children(child_doc);
    }

    let has_type_annotation: bool = driver.gen_bool(None)?;
    if has_type_annotation {
        let type_name = gen_identifier(driver)?;
        node.set_ty(type_name);
    }

    Some(node)
}

fn gen_in_range<D: Driver>(driver: &mut D, range: impl std::ops::RangeBounds<u8>) -> Option<u8> {
    driver.gen_u8(range.start_bound(), range.end_bound())
}

fn gen_identifier<D: Driver>(driver: &mut D) -> Option<kdl::KdlIdentifier> {
    let len: u8 = gen_in_range(driver, 1..=20)?;
    let mut name = String::with_capacity(len as usize);
    for _ in 0..len {
        let c = driver.gen_char(Bound::Unbounded, Bound::Unbounded)?;
        name.push(c);
    }
    match name.parse::<kdl::KdlIdentifier>() {
        Ok(i) => Some(i),
        Err(_) => format!("\"{name}\"").parse::<kdl::KdlIdentifier>().ok(),
    }
}

fn gen_value<D: Driver>(driver: &mut D) -> Option<kdl::KdlValue> {
    let choice: u8 = gen_in_range(driver, 0..=7)?;
    match choice {
        0 => {
            // String
            let len: u8 = gen_in_range(driver, 0..=20)?;
            let mut s = String::with_capacity(len as usize);
            for _ in 0..len {
                let c = driver.gen_char(Bound::Unbounded, Bound::Unbounded)?;
                s.push(c);
            }
            Some(kdl::KdlValue::String(s))
        }
        1 => {
            // Integer
            let i: i128 = driver.gen_i128(Bound::Unbounded, Bound::Unbounded)?;
            Some(kdl::KdlValue::Integer(i))
        }
        2 => {
            // Float
            let f: f64 = driver.gen_f64(Bound::Unbounded, Bound::Unbounded)?;
            Some(kdl::KdlValue::Float(f))
        }
        3 => {
            // Boolean
            let b: bool = driver.gen_bool(None)?;
            Some(kdl::KdlValue::Bool(b))
        }
        4 => {
            // Null
            Some(kdl::KdlValue::Null)
        }
        5 => {
            // Nan
            Some(kdl::KdlValue::Float(f64::NAN))
        }
        6 => {
            // Positive Infinity
            Some(kdl::KdlValue::Float(f64::INFINITY))
        }
        7 => {
            // Negative Infinity
            Some(kdl::KdlValue::Float(f64::NEG_INFINITY))
        }
        _ => None,
    }
}

fn gen_entry<D: Driver>(driver: &mut D, is_prop: bool) -> Option<kdl::KdlEntry> {
    let mut entry = if is_prop {
        let key = gen_identifier(driver)?;
        let value = gen_value(driver)?;
        kdl::KdlEntry::new_prop(key, value)
    } else {
        let value = gen_value(driver)?;
        kdl::KdlEntry::new(value)
    };

    let has_type_annotation: bool = driver.gen_bool(None)?;
    if has_type_annotation {
        let type_name = gen_identifier(driver)?;
        entry.set_ty(type_name);
    }
    Some(entry)
}
