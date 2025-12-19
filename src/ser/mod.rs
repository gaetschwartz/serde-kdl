use crate::error::{Error, Result};
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};
use serde::Serializer as SerializerTrait;
use std::collections::BTreeMap;

mod seq;
mod tuple;
mod map;
mod structs;

pub use seq::SerializeSeqImpl;
pub use tuple::{SerializeTupleImpl, SerializeTupleStructImpl, SerializeTupleVariantImpl};
pub use map::SerializeMapImpl;
pub use structs::{SerializeStructImpl, SerializeStructVariantImpl};

/// Node context for serialization: (`node_name`, children, properties)
type NodeContext = (String, Vec<KdlNode>, Vec<(String, KdlValue)>);

/// A serializer that converts Rust values directly to KDL documents.
pub struct Serializer {
    document: KdlDocument,
    current_node: Option<KdlNode>,
    node_stack: Vec<NodeContext>,
    /// Whether this is the main/root serializer (vs a temp serializer for field values)
    is_root_serializer: bool,
}

impl Serializer {
    /// Create a new serializer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            document: KdlDocument::new(),
            current_node: None,
            node_stack: Vec::new(),
            is_root_serializer: true,
        }
    }

    /// Create a new temp serializer for serializing field values.
    fn new_for_field() -> Self {
        Self {
            document: KdlDocument::new(),
            current_node: None,
            node_stack: Vec::new(),
            is_root_serializer: false,
        }
    }

    /// Consume the serializer and return the generated KDL document.
    #[must_use]
    pub fn into_document(mut self) -> KdlDocument {
        // If we have a current node, add it to the document
        if let Some(node) = self.current_node.take() {
            self.document.nodes_mut().push(node);
        }

        self.document
    }

    fn create_value_node(&mut self, name: &str, value: KdlValue) -> Result<()> {
        let mut node = KdlNode::new(name);
        node.entries_mut().push(KdlEntry::new(value));
        self.current_node = Some(node);
        Ok(())
    }

    fn push_node_context(&mut self, name: String) {
        self.node_stack.push((name, Vec::new(), Vec::new()));
    }

    fn pop_node_context(&mut self) -> Result<()> {
        if let Some((name, children, properties)) = self.node_stack.pop() {
            let mut node = KdlNode::new(name);

            // Check if there's a current_node that should be incorporated (for newtype variants)
            if let Some(current) = self.current_node.take() {
                // If the current node has entries, copy them to our node
                for entry in current.entries() {
                    if entry.name().is_none() {
                        // This is a value entry, add it directly
                        node.entries_mut().push(entry.clone());
                    }
                }
                // If the current node has children, add them
                if let Some(current_children) = current.children() {
                    if !current_children.nodes().is_empty() {
                        let mut child_doc =
                            node.children().cloned().unwrap_or_else(KdlDocument::new);
                        for child in current_children.nodes() {
                            child_doc.nodes_mut().push(child.clone());
                        }
                        *node.children_mut() = Some(child_doc);
                    }
                }
            }

            // Add properties to the node
            for (key, value) in properties {
                let entry = KdlEntry::new_prop(key, value);
                node.entries_mut().push(entry);
            }

            // Add children if any
            if !children.is_empty() {
                let mut child_doc = node.children().cloned().unwrap_or_else(KdlDocument::new);
                for child in children {
                    child_doc.nodes_mut().push(child);
                }
                *node.children_mut() = Some(child_doc);
            }

            if let Some(parent_context) = self.node_stack.last_mut() {
                parent_context.1.push(node);
            } else {
                self.current_node = Some(node);
            }
        }
        Ok(())
    }

    fn add_property_to_current(&mut self, key: &str, value: KdlValue) -> Result<()> {
        if let Some((_name, _children, properties)) = self.node_stack.last_mut() {
            // Add property to the current struct
            properties.push((key.to_string(), value));
        } else {
            // Create a new node for this property
            let mut node = KdlNode::new(key);
            node.entries_mut().push(KdlEntry::new(value));
            self.current_node = Some(node);
        }
        Ok(())
    }
}

impl Default for Serializer {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> SerializerTrait for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SerializeSeqImpl<'a>;
    type SerializeTuple = SerializeTupleImpl<'a>;
    type SerializeTupleStruct = SerializeTupleStructImpl<'a>;
    type SerializeTupleVariant = SerializeTupleVariantImpl<'a>;
    type SerializeMap = SerializeMapImpl<'a>;
    type SerializeStruct = SerializeStructImpl<'a>;
    type SerializeStructVariant = SerializeStructVariantImpl<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        // KDL 6.5 now supports i128 natively
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        // KDL 6.5 now supports i128 natively
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Integer(v))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        // KDL 6.5 now supports i128 natively, but u128 can exceed i128::MAX
        if v <= i128::MAX as u128 {
            self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Integer(v as i128))
        } else {
            Err(Error::UnsupportedType(format!(
                "u128 value {} exceeds KDL's supported integer range (i128::MAX = {})",
                v,
                i128::MAX
            )))
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Float(f64::from(v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        // Note: KDL specification mentions #inf, #-inf, and #nan for special values,
        // but kdl crate 4.7 doesn't support parsing these. For now, serialize
        // special float values as regular floats (which will be inf, -inf, NaN in text).
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Float(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        #[cfg(feature = "bytes")]
        {
            let hex_string = crate::hex::encode_hex(v);
            self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::String(hex_string))
        }
        #[cfg(not(feature = "bytes"))]
        {
            let _ = v; // Silence unused parameter warning
            Err(Error::UnsupportedType("byte arrays".to_string()))
        }
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Null)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.create_value_node(crate::DEFAULT_NODE_NAME, KdlValue::Null)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        let node = KdlNode::new(name);
        self.current_node = Some(node);
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.create_value_node(
            crate::DEFAULT_NODE_NAME,
            KdlValue::String(variant.to_string()),
        )
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        self.push_node_context(name.to_string());
        value.serialize(&mut *self)?;
        self.pop_node_context()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        self.push_node_context(variant.to_string());
        value.serialize(&mut *self)?;
        self.pop_node_context()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        #[cfg(feature = "bytes")]
        {
            Ok(SerializeSeqImpl {
                ser: self,
                items: Vec::with_capacity(len.unwrap_or(0)),
                child_nodes: Vec::with_capacity(len.unwrap_or(0)),
                bytes_candidate: Some(Vec::with_capacity(len.unwrap_or(0))),
            })
        }
        #[cfg(not(feature = "bytes"))]
        {
            Ok(SerializeSeqImpl {
                ser: self,
                items: Vec::with_capacity(len.unwrap_or(0)),
                child_nodes: Vec::with_capacity(len.unwrap_or(0)),
            })
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(SerializeTupleImpl {
            ser: self,
            items: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.push_node_context(name.to_string());
        Ok(SerializeTupleStructImpl {
            ser: self,
            items: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.push_node_context(variant.to_string());
        Ok(SerializeTupleVariantImpl {
            ser: self,
            items: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMapImpl {
            ser: self,
            pending_key: None,
            items: BTreeMap::new(),
        })
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        // Only use transparent mode for the actual root serializer with empty node stack
        let is_root = self.is_root_serializer && self.node_stack.is_empty();
        if !is_root {
            // Nested struct - wrap in named node
            self.push_node_context(name.to_string());
        }
        Ok(SerializeStructImpl { ser: self, is_root })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.push_node_context(variant.to_string());
        Ok(SerializeStructVariantImpl { ser: self })
    }
}
