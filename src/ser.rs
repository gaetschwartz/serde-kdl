use crate::error::{Error, Result};
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::Serializer as SerializerTrait;
use std::collections::BTreeMap;

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

// Sequence serializer
pub struct SerializeSeqImpl<'a> {
    ser: &'a mut Serializer,
    items: Vec<KdlValue>,
    child_nodes: Vec<KdlNode>,
    #[cfg(feature = "bytes")]
    bytes_candidate: Option<Vec<u8>>,
}

impl SerializeSeq for SerializeSeqImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        // Create a temporary serializer for the item
        let mut item_serializer = Serializer::new_for_field();
        value.serialize(&mut item_serializer)?;

        let document = item_serializer.into_document();
        if let Some(node) = document.nodes().first() {
            // Check if this is a complex structure (has multiple entries, properties, or children)
            // Also consider enum variants (nodes with meaningful names) as complex
            let is_complex = node.entries().len() > 1
                || !node.entries().iter().all(|e| e.name().is_none()) // has properties
                || node.children().is_some_and(|c| !c.nodes().is_empty()) // has children
                || node.name().value() != crate::DEFAULT_NODE_NAME; // enum variants or other meaningful node names

            if is_complex {
                // Complex structure - store as child node
                self.child_nodes.push(node.clone());
                #[cfg(feature = "bytes")]
                {
                    // Complex elements mean this can't be a simple byte array
                    self.bytes_candidate = None;
                }
            } else if let Some(entry) = node.entries().first() {
                // Simple value - store in items for inline representation
                let value = entry.value().clone();

                #[cfg(feature = "bytes")]
                {
                    // Check if this value could be a byte (u8 in range 0-255)
                    if let Some(ref mut bytes) = self.bytes_candidate {
                        if let KdlValue::Integer(i) = &value {
                            if *i >= 0 && *i <= 255 {
                                bytes.push(*i as u8);
                            } else {
                                // Value out of u8 range, not a byte array
                                self.bytes_candidate = None;
                            }
                        } else {
                            // Non-integer value, not a byte array
                            self.bytes_candidate = None;
                        }
                    }
                }

                self.items.push(value);
            } else {
                // Node without value - create a null
                self.items.push(KdlValue::Null);
                #[cfg(feature = "bytes")]
                {
                    // Null values mean this can't be a simple byte array
                    self.bytes_candidate = None;
                }
            }
        } else {
            self.items.push(KdlValue::Null);
            #[cfg(feature = "bytes")]
            {
                // Null values mean this can't be a simple byte array
                self.bytes_candidate = None;
            }
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let mut node = KdlNode::new(crate::DEFAULT_NODE_NAME);

        #[cfg(feature = "bytes")]
        {
            // Check if this sequence is a byte array that should be serialized as hex
            if let Some(bytes) = self.bytes_candidate {
                if !bytes.is_empty()
                    && self.child_nodes.is_empty()
                    && self.items.len() == bytes.len()
                {
                    // This looks like a byte array - serialize as hex string instead
                    let hex_string = crate::hex::encode_hex(&bytes);
                    node.entries_mut()
                        .push(KdlEntry::new(KdlValue::String(hex_string)));
                    self.ser.current_node = Some(node);
                    return Ok(());
                }
            }
        }

        // Add simple values as entries
        for item in self.items {
            node.entries_mut().push(KdlEntry::new(item));
        }

        // Add complex structures as children
        if !self.child_nodes.is_empty() {
            let mut child_doc = KdlDocument::new();
            for child in self.child_nodes {
                child_doc.nodes_mut().push(child);
            }
            *node.children_mut() = Some(child_doc);
        }

        self.ser.current_node = Some(node);
        Ok(())
    }
}

// Tuple serializer
pub struct SerializeTupleImpl<'a> {
    ser: &'a mut Serializer,
    items: Vec<KdlValue>,
}

impl SerializeTuple for SerializeTupleImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut item_serializer = Serializer::new_for_field();
        value.serialize(&mut item_serializer)?;

        let document = item_serializer.into_document();
        if let Some(node) = document.nodes().first() {
            if let Some(entry) = node.entries().first() {
                self.items.push(entry.value().clone());
            } else {
                self.items.push(KdlValue::Null);
            }
        } else {
            self.items.push(KdlValue::Null);
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let mut node = KdlNode::new(crate::DEFAULT_NODE_NAME);
        for item in self.items {
            node.entries_mut().push(KdlEntry::new(item));
        }
        self.ser.current_node = Some(node);
        Ok(())
    }
}

// Tuple struct serializer
pub struct SerializeTupleStructImpl<'a> {
    ser: &'a mut Serializer,
    items: Vec<KdlValue>,
}

impl SerializeTupleStruct for SerializeTupleStructImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut item_serializer = Serializer::new_for_field();
        value.serialize(&mut item_serializer)?;

        let document = item_serializer.into_document();
        if let Some(node) = document.nodes().first() {
            if let Some(entry) = node.entries().first() {
                self.items.push(entry.value().clone());
            } else {
                self.items.push(KdlValue::Null);
            }
        } else {
            self.items.push(KdlValue::Null);
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        // Add the tuple values as entries to the current struct node
        if let Some((_, children, _)) = self.ser.node_stack.last_mut() {
            let mut node = KdlNode::new("tuple");
            for item in self.items {
                node.entries_mut().push(KdlEntry::new(item));
            }
            children.push(node);
        }
        self.ser.pop_node_context()
    }
}

// Tuple variant serializer
pub struct SerializeTupleVariantImpl<'a> {
    ser: &'a mut Serializer,
    items: Vec<KdlValue>,
}

impl SerializeTupleVariant for SerializeTupleVariantImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut item_serializer = Serializer::new_for_field();
        value.serialize(&mut item_serializer)?;

        let document = item_serializer.into_document();
        if let Some(node) = document.nodes().first() {
            if let Some(entry) = node.entries().first() {
                self.items.push(entry.value().clone());
            } else {
                self.items.push(KdlValue::Null);
            }
        } else {
            self.items.push(KdlValue::Null);
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        if let Some((_, children, _)) = self.ser.node_stack.last_mut() {
            let mut node = KdlNode::new("tuple");
            for item in self.items {
                node.entries_mut().push(KdlEntry::new(item));
            }
            children.push(node);
        }
        self.ser.pop_node_context()
    }
}

// Map serializer
pub struct SerializeMapImpl<'a> {
    ser: &'a mut Serializer,
    pending_key: Option<String>,
    items: BTreeMap<String, KdlNode>,
}

impl SerializeMap for SerializeMapImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut key_serializer = Serializer::new_for_field();
        key.serialize(&mut key_serializer)?;

        let document = key_serializer.into_document();
        if let Some(node) = document.nodes().first() {
            if let Some(entry) = node.entries().first() {
                match entry.value() {
                    KdlValue::String(s) => self.pending_key = Some(s.clone()),
                    other => self.pending_key = Some(format!("{other}")),
                }
            } else {
                self.pending_key = Some(node.name().value().to_string());
            }
        }

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let key = self
            .pending_key
            .take()
            .unwrap_or_else(|| "unknown".to_string());

        let mut value_serializer = Serializer::new_for_field();
        value.serialize(&mut value_serializer)?;

        let document = value_serializer.into_document();
        if let Some(node) = document.nodes().first() {
            // Create a new node with the map key as name
            let mut key_node = KdlNode::new(key.clone());

            // Copy all entries (values and properties)
            for entry in node.entries() {
                key_node.entries_mut().push(entry.clone());
            }

            // Copy children if any
            if let Some(children) = node.children() {
                if !children.nodes().is_empty() {
                    *key_node.children_mut() = Some(children.clone());
                }
            }

            self.items.insert(key, key_node);
        } else {
            // Empty document - create node with null value
            let mut key_node = KdlNode::new(key.clone());
            key_node.entries_mut().push(KdlEntry::new(KdlValue::Null));
            self.items.insert(key, key_node);
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let mut node = KdlNode::new(crate::DEFAULT_NODE_NAME);
        if !self.items.is_empty() {
            let mut child_doc = KdlDocument::new();
            for (_key, child_node) in self.items {
                child_doc.nodes_mut().push(child_node);
            }
            *node.children_mut() = Some(child_doc);
        }
        self.ser.current_node = Some(node);
        Ok(())
    }
}

// Struct serializer
pub struct SerializeStructImpl<'a> {
    ser: &'a mut Serializer,
    is_root: bool,
}

impl SerializeStruct for SerializeStructImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        // Serialize the field value
        let mut field_serializer = Serializer::new_for_field();
        value.serialize(&mut field_serializer)?;
        let field_doc = field_serializer.into_document();

        // Handle Option<T> specially - skip None values (empty doc means None)
        if std::any::type_name::<T>().starts_with("core::option::Option")
            && field_doc.nodes().is_empty()
        {
            return Ok(());
        }

        if self.is_root {
            // Root level: add field as document node
            self.serialize_root_field(key, &field_doc)?;
        } else {
            // Nested: add to node context
            self.serialize_nested_field(key, &field_doc)?;
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        if self.is_root {
            // Root level: nothing to do, fields already added to document
            Ok(())
        } else {
            // Nested: finalize the node context
            self.ser.pop_node_context()
        }
    }
}

impl SerializeStructImpl<'_> {
    fn serialize_root_field(&mut self, key: &str, field_doc: &KdlDocument) -> Result<()> {
        if let Some(node) = field_doc.nodes().first() {
            let mut field_node = KdlNode::new(key);

            // Copy all entries from the serialized field
            for entry in node.entries() {
                field_node.entries_mut().push(entry.clone());
            }

            // Copy children if any
            if let Some(node_children) = node.children() {
                if !node_children.nodes().is_empty() {
                    let mut child_doc = KdlDocument::new();
                    for child in node_children.nodes() {
                        child_doc.nodes_mut().push(child.clone());
                    }
                    *field_node.children_mut() = Some(child_doc);
                }
            }

            // Add directly to document
            self.ser.document.nodes_mut().push(field_node);
        }
        Ok(())
    }

    fn serialize_nested_field(&mut self, key: &str, field_doc: &KdlDocument) -> Result<()> {
        if let Some(node) = field_doc.nodes().first() {
            if node.entries().len() == 1 && node.children().is_none_or(|c| c.nodes().is_empty()) {
                // Single simple value - use as property
                let entry = &node.entries()[0];
                self.ser
                    .add_property_to_current(key, entry.value().clone())?;
            } else {
                // Multiple entries (array) or complex structure - use as child node
                if let Some((_, children, _)) = self.ser.node_stack.last_mut() {
                    let mut field_node = KdlNode::new(key);

                    // Copy all entries from the serialized field
                    for entry in node.entries() {
                        field_node.entries_mut().push(entry.clone());
                    }

                    // Copy children if any
                    if let Some(node_children) = node.children() {
                        if !node_children.nodes().is_empty() {
                            let mut child_doc = KdlDocument::new();
                            for child in node_children.nodes() {
                                child_doc.nodes_mut().push(child.clone());
                            }
                            *field_node.children_mut() = Some(child_doc);
                        }
                    }

                    children.push(field_node);
                }
            }
        }
        Ok(())
    }
}

// Struct variant serializer
pub struct SerializeStructVariantImpl<'a> {
    ser: &'a mut Serializer,
}

impl SerializeStructVariant for SerializeStructVariantImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut field_serializer = Serializer::new_for_field();
        value.serialize(&mut field_serializer)?;
        let field_doc = field_serializer.into_document();

        if let Some(node) = field_doc.nodes().first() {
            if let Some(entry) = node.entries().first() {
                self.ser
                    .add_property_to_current(key, entry.value().clone())?;
            } else if let Some(node_children) = node.children() {
                if !node_children.nodes().is_empty() {
                    if let Some((_, children, _)) = self.ser.node_stack.last_mut() {
                        let mut field_node = KdlNode::new(key);
                        let mut child_doc = KdlDocument::new();
                        for child in node_children.nodes() {
                            child_doc.nodes_mut().push(child.clone());
                        }
                        *field_node.children_mut() = Some(child_doc);
                        children.push(field_node);
                    }
                }
            } else if let Some((_, children, _)) = self.ser.node_stack.last_mut() {
                children.push(KdlNode::new(format!("{}_{}", key, node.name().value())));
            }
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.ser.pop_node_context()
    }
}
