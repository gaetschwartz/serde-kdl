use crate::error::{Error, Result};
use kdl::{KdlDocument, KdlNode, KdlValue};
use serde::de::Visitor;
use serde::{forward_to_deserialize_any, Deserializer as DeserializerTrait};

mod entry;
mod map;
mod node;
mod seq;
mod structs;
mod variants;

use map::MapDeserializer;
use seq::SeqDeserializer;
use structs::{RootStructDeserializer, StructDeserializer};
use variants::EnumDeserializer;

/// A deserializer that converts KDL documents directly to Rust values.
pub struct Deserializer<'de> {
    document: &'de KdlDocument,
    current_node: Option<&'de KdlNode>,
}

impl<'de> Deserializer<'de> {
    /// Create a new deserializer from a KDL document.
    #[must_use]
    pub fn new(document: &'de KdlDocument) -> Self {
        Self {
            document,
            current_node: None,
        }
    }

    /// Get the current node or the first document node.
    fn get_node(&self) -> Result<&'de KdlNode> {
        if let Some(node) = self.current_node {
            Ok(node)
        } else {
            let nodes = self.document.nodes();
            if nodes.is_empty() {
                return Err(Error::MissingRootNode);
            }
            Ok(&nodes[0])
        }
    }

    fn deserialize_from_node<V>(&mut self, node: &'de KdlNode, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // If the node has entries, try to deserialize from the first entry
        if let Some(entry) = node.entries().first() {
            self.deserialize_from_value(entry.value(), visitor)
        } else if let Some(children) = node.children() {
            if children.nodes().is_empty() {
                visitor.visit_unit()
            } else {
                // If the node has children, treat it as a map
                let map_de = MapDeserializer::new(children.nodes());
                visitor.visit_map(map_de)
            }
        } else {
            // Empty node - treat as unit
            visitor.visit_unit()
        }
    }

    fn deserialize_from_value<V>(&mut self, value: &'de KdlValue, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match value {
            KdlValue::String(s) => visitor.visit_str(s),
            KdlValue::Integer(i) => {
                // serde visitors for most numeric types only accept i64, not i128
                if let Ok(v) = i64::try_from(*i) {
                    visitor.visit_i64(v)
                } else {
                    visitor.visit_i128(*i)
                }
            }
            KdlValue::Float(f) => visitor.visit_f64(*f),
            KdlValue::Bool(b) => visitor.visit_bool(*b),
            KdlValue::Null => visitor.visit_unit(),
        }
    }
}

impl<'de> DeserializerTrait<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // If we have a current node, deserialize from it
        if let Some(node) = self.current_node {
            return self.deserialize_from_node(node, visitor);
        }

        // Otherwise, try to get the root node
        let nodes = self.document.nodes();
        if nodes.is_empty() {
            return Err(Error::MissingRootNode);
        }

        if nodes.len() > 1 {
            return Err(Error::MultipleRootNodes { count: nodes.len() });
        }

        let root_node = &nodes[0];
        self.current_node = Some(root_node);
        self.deserialize_from_node(root_node, visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(Error::UnsupportedType("byte arrays".to_string()))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let _ = visitor;
        Err(Error::UnsupportedType("byte arrays".to_string()))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Check for #null value indicating None
        if let Some(node) = self.current_node {
            // Check if node has a single #null entry
            if node.entries().len() == 1 {
                if let Some(entry) = node.entries().first() {
                    if matches!(entry.value(), KdlValue::Null) {
                        return visitor.visit_none();
                    }
                }
            }

            let has_children = node.children().is_some_and(|c| !c.nodes().is_empty());
            if node.entries().is_empty() && !has_children {
                visitor.visit_none()
            } else {
                visitor.visit_some(self)
            }
        } else {
            // Check if we have any nodes at all
            if self.document.nodes().is_empty() {
                visitor.visit_none()
            } else {
                visitor.visit_some(self)
            }
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(node) = self.current_node {
            // If we have multiple entries in a node, treat them as a sequence (tuples)
            if node.entries().len() > 1 {
                let seq_de = SeqDeserializer::from_entries(node.entries());
                return visitor.visit_seq(seq_de);
            }
            // If we have children, treat them as a sequence (Vec with "-" wrapper)
            if let Some(children) = node.children() {
                if !children.nodes().is_empty() {
                    let seq_de = SeqDeserializer::from_children(children.nodes());
                    return visitor.visit_seq(seq_de);
                }
            }
            // Empty children block means empty sequence
            if node.children().is_some() {
                return visitor.visit_seq(SeqDeserializer::from_entries(&[]));
            }
        }

        // Try root level nodes as sequence
        let nodes = self.document.nodes();
        if nodes.len() == 1 {
            let root_node = &nodes[0];

            // Check for entries (tuple-like)
            if root_node.entries().len() > 1 {
                let seq_de = SeqDeserializer::from_entries(root_node.entries());
                return visitor.visit_seq(seq_de);
            }

            // Check for children (Vec with "-" wrapper pattern)
            if let Some(children) = root_node.children() {
                let seq_de = SeqDeserializer::from_children(children.nodes());
                return visitor.visit_seq(seq_de);
            }
        }

        // Fallback to empty sequence
        visitor.visit_seq(SeqDeserializer::from_entries(&[]))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(node) = self.current_node {
            if let Some(children) = node.children() {
                if !children.nodes().is_empty() {
                    let map_de = MapDeserializer::new(children.nodes());
                    return visitor.visit_map(map_de);
                }
            }
        }

        // Try root level as map
        let nodes = self.document.nodes();
        if nodes.len() == 1 {
            let root_node = &nodes[0];
            if let Some(children) = root_node.children() {
                if !children.nodes().is_empty() {
                    let map_de = MapDeserializer::new(children.nodes());
                    return visitor.visit_map(map_de);
                }
            }
        }

        visitor.visit_map(MapDeserializer::new(&[]))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(node) = self.current_node {
            // Nested - use current node's children
            let struct_de = StructDeserializer::new(node);
            return visitor.visit_map(struct_de);
        }

        // Root level - treat document nodes as struct fields
        let nodes = self.document.nodes();
        let root_de = RootStructDeserializer::new(nodes);
        visitor.visit_map(root_de)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let node = self.get_node()?;

        // Check for type annotation (e.g., (VariantName)field_name)
        if let Some(ty) = node.ty() {
            let variant_name = ty.value();
            return visitor.visit_enum(EnumDeserializer::new(variant_name, node));
        }

        // Check if this is a simple string enum variant
        if node.entries().len() == 1 {
            if let Some(entry) = node.entries().first() {
                if let KdlValue::String(s) = entry.value() {
                    use serde::de::value::StrDeserializer;
                    return visitor.visit_enum(StrDeserializer::<Error>::new(s.as_str()));
                }
            }
        }

        // No type annotation found - error
        Err(Error::Serde(
            "enum requires type annotation, e.g., (Variant)field_name".to_string(),
        ))
    }

    forward_to_deserialize_any!(
        bool
        u8 u16 u32 u64 u128
        i8 i16 i32 i64 i128
        f32 f64
        char str string
        identifier ignored_any newtype_struct  tuple tuple_struct unit unit_struct
    );
}
