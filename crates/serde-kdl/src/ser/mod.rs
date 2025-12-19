use std::mem;

use crate::{
    error::{Error, Result},
    DEFAULT_NODE_NAME,
};
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};
use serde::Serializer as SerializerTrait;

mod map;
mod seq;
mod structs;
mod tuple;

pub use map::SerializeMapImpl;
pub use seq::SerializeSeqImpl;
pub use structs::{SerializeStructImpl, SerializeStructVariantImpl};
pub use tuple::{SerializeTupleImpl, SerializeTupleStructImpl, SerializeTupleVariantImpl};

/// Node context for serialization: (`node_name`, children)
type NodeContext = (String, Vec<KdlNode>);

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
        self.node_stack.push((name, Vec::new()));
    }

    fn pop_node_context(&mut self) -> Result<()> {
        if let Some((name, children)) = self.node_stack.pop() {
            let mut node = KdlNode::new(name);

            // Check if there's a current_node that should be incorporated (for newtype variants)
            if let Some(mut current) = self.current_node.take() {
                // If the current node has entries, copy them to our node
                for entry in current.entries_mut() {
                    if entry.name().is_none() {
                        // This is a value entry, add it directly
                        node.entries_mut()
                            .push(mem::replace(entry, KdlEntry::new(KdlValue::Null)));
                    }
                }
                // If the current node has children, add them
                if let Some(current_children) = current.children_mut() {
                    if !current_children.nodes().is_empty() {
                        let mut child_doc = node.children_mut().take().unwrap_or_default();
                        child_doc
                            .nodes_mut()
                            .extend(mem::take(current_children.nodes_mut()));
                        *node.children_mut() = Some(child_doc);
                    }
                }
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
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        // KDL 6.5 now supports i128 natively
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Integer(i128::from(v)))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        // KDL 6.5 now supports i128 natively
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Integer(v))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        // KDL 6.5 now supports i128 natively, but u128 can exceed i128::MAX
        if v <= i128::MAX as u128 {
            self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Integer(v as i128))
        } else {
            Err(Error::UnsupportedType(format!(
                "u128 value {} exceeds KDL's supported integer range (i128::MAX = {})",
                v,
                i128::MAX
            )))
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Float(f64::from(v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        // Note: KDL specification mentions #inf, #-inf, and #nan for special values,
        // but kdl crate 4.7 doesn't support parsing these. For now, serialize
        // special float values as regular floats (which will be inf, -inf, NaN in text).
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Float(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::String(v.to_string()))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        Err(Error::UnsupportedType("byte arrays".to_string()))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Null)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.create_value_node(DEFAULT_NODE_NAME, KdlValue::Null)
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
        // Create a node with type annotation for the variant
        let mut node = KdlNode::new(DEFAULT_NODE_NAME);
        node.set_ty(kdl::KdlIdentifier::from(variant));
        self.current_node = Some(node);
        Ok(())
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
        // Serialize the inner value first
        value.serialize(&mut *self)?;

        // Add the type annotation to the current node
        if let Some(node) = &mut self.current_node {
            node.set_ty(kdl::KdlIdentifier::from(variant));
        }
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeSeqImpl {
            ser: self,
            child_nodes: Vec::with_capacity(len.unwrap_or(0)),
        })
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
        Ok(SerializeTupleVariantImpl {
            ser: self,
            variant: variant.to_string(),
            items: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMapImpl {
            ser: self,
            pending_key: None,
            items: Vec::with_capacity(len.unwrap_or(0)),
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
        Ok(SerializeStructVariantImpl {
            ser: self,
            variant: variant.to_string(),
        })
    }
}

pub(crate) trait SerializeFieldValue<T> {
    fn into_option_node(self) -> Result<Option<KdlNode>>;

    fn into_node(self) -> Result<KdlNode>
    where
        Self: Sized,
    {
        let opt_node = self.into_option_node()?;
        Ok(opt_node.unwrap_or_else(|| KdlNode::new(crate::DEFAULT_NODE_NAME)))
    }
}

impl<T> SerializeFieldValue<T> for T
where
    T: Sized + serde::Serialize,
{
    fn into_option_node(self) -> Result<Option<KdlNode>> {
        let mut serializer = Serializer::new_for_field();
        self.serialize(&mut serializer)?;
        let mut document = serializer.into_document();
        if let Some(node) = document.nodes_mut().pop() {
            Ok(Some(node))
        } else {
            Ok(None)
        }
    }
}
