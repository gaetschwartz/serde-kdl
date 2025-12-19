use crate::error::{Error, Result};
use kdl::{KdlDocument, KdlNode, KdlValue};
use serde::de::Visitor;
use serde::{forward_to_deserialize_any, Deserializer as DeserializerTrait};

mod seq;
mod map;
mod structs;
mod variants;
mod node;
mod entry;

use seq::SeqDeserializer;
#[cfg(feature = "bytes")]
use seq::BytesSeqDeserializer;
use map::MapDeserializer;
use structs::{StructDeserializer, RootStructDeserializer};
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
        #[cfg(feature = "bytes")]
        {
            let node = self.get_node()?;
            if let Some(entry) = node.entries().first() {
                match entry.value() {
                    KdlValue::String(s) => {
                        let bytes = crate::hex::decode_hex(s)?;
                        visitor.visit_bytes(&bytes)
                    }
                    _ => Err(Error::UnsupportedType(
                        "bytes must be represented as hex strings".to_string(),
                    )),
                }
            } else {
                Err(Error::UnsupportedType(
                    "no value found for bytes".to_string(),
                ))
            }
        }
        #[cfg(not(feature = "bytes"))]
        {
            let _ = visitor;
            Err(Error::UnsupportedType("byte arrays".to_string()))
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "bytes")]
        {
            let node = self.get_node()?;
            if let Some(entry) = node.entries().first() {
                match entry.value() {
                    KdlValue::String(s) => {
                        let bytes = crate::hex::decode_hex(s)?;
                        visitor.visit_byte_buf(bytes)
                    }
                    _ => Err(Error::UnsupportedType(
                        "bytes must be represented as hex strings".to_string(),
                    )),
                }
            } else {
                Err(Error::UnsupportedType(
                    "no value found for bytes".to_string(),
                ))
            }
        }
        #[cfg(not(feature = "bytes"))]
        {
            let _ = visitor;
            Err(Error::UnsupportedType("byte arrays".to_string()))
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // For KDL, we treat missing fields as None and present fields as Some
        if let Some(node) = self.current_node {
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
            // Check for single string entry that might be hex bytes
            #[cfg(feature = "bytes")]
            {
                if node.entries().len() == 1 {
                    if let Some(entry) = node.entries().first() {
                        if entry.name().is_none() {
                            // Not a property
                            if let KdlValue::String(s) = entry.value() {
                                // Try to decode as hex - if successful, use bytes deserializer
                                if let Ok(bytes) = crate::hex::decode_hex(s) {
                                    return visitor.visit_seq(BytesSeqDeserializer::new(bytes));
                                }
                            }
                        }
                    }
                }
            }

            // If we have multiple entries in a node, treat them as a sequence
            if node.entries().len() > 1 {
                let seq_de = SeqDeserializer::from_entries(node.entries());
                return visitor.visit_seq(seq_de);
            }
            // If we have children, treat them as a sequence
            if let Some(children) = node.children() {
                if !children.nodes().is_empty() {
                    let seq_de = SeqDeserializer::from_children(children.nodes());
                    return visitor.visit_seq(seq_de);
                }
            }
        }

        // Try root level nodes as sequence
        let nodes = self.document.nodes();
        if nodes.len() == 1 {
            let root_node = &nodes[0];

            // Check for single string entry that might be hex bytes at root level
            #[cfg(feature = "bytes")]
            {
                if root_node.entries().len() == 1 {
                    if let Some(entry) = root_node.entries().first() {
                        if entry.name().is_none() {
                            // Not a property
                            if let KdlValue::String(s) = entry.value() {
                                // Try to decode as hex - if successful, use bytes deserializer
                                if let Ok(bytes) = crate::hex::decode_hex(s) {
                                    return visitor.visit_seq(BytesSeqDeserializer::new(bytes));
                                }
                            }
                        }
                    }
                }
            }

            if root_node.entries().len() > 1 {
                let seq_de = SeqDeserializer::from_entries(root_node.entries());
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
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(node) = self.current_node {
            // Nested - use current node's properties and children
            let struct_de = StructDeserializer::new(node);
            return visitor.visit_map(struct_de);
        }

        // Root level - check if we should use old or new format
        let nodes = self.document.nodes();

        // Check for old format: single node with struct name that has properties/children
        if let Some(struct_node) = nodes.iter().find(|n| n.name().value() == name) {
            let has_properties = struct_node.entries().iter().any(|e| e.name().is_some());
            let has_children = struct_node
                .children()
                .is_some_and(|c| !c.nodes().is_empty());

            if has_properties || has_children {
                // Old format - use StructDeserializer
                let struct_de = StructDeserializer::new(struct_node);
                return visitor.visit_map(struct_de);
            }
        }

        // New format - treat document nodes as struct fields
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

        // Check if this is a simple string enum variant (default node name with string entry)
        if node.name().value() == crate::DEFAULT_NODE_NAME && node.entries().len() == 1 {
            if let Some(entry) = node.entries().first() {
                if let KdlValue::String(s) = entry.value() {
                    use serde::de::value::StrDeserializer;
                    return visitor.visit_enum(StrDeserializer::<Error>::new(s.as_str()));
                }
            }
        }

        // Complex enum variant - use node name as variant
        let variant_name = node.name().value();
        visitor.visit_enum(EnumDeserializer::new(variant_name, node))
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
