use crate::error::{Error, Result};
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};
use serde::de::{DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserializer as DeserializerTrait;

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
            KdlValue::RawString(s) => visitor.visit_str(s),
            KdlValue::Base10(i) => visitor.visit_i64(*i),
            KdlValue::Base16(i) => visitor.visit_i64(*i),
            KdlValue::Base8(i) => visitor.visit_i64(*i),
            KdlValue::Base2(i) => visitor.visit_i64(*i),
            KdlValue::Base10Float(f) => visitor.visit_f64(*f),
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

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // i128 support: KDL spec supports i128, but kdl crate 4.7 stores integers as i64
        // We can deserialize any i64 value as i128 since i64 range is subset of i128
        if let Some(node) = self.current_node {
            if let Some(entry) = node.entries().first() {
                match entry.value() {
                    KdlValue::Base10(i)
                    | KdlValue::Base16(i)
                    | KdlValue::Base8(i)
                    | KdlValue::Base2(i) => visitor.visit_i128(i128::from(*i)),
                    _ => self.deserialize_any(visitor),
                }
            } else {
                self.deserialize_any(visitor)
            }
        } else {
            // Try root level
            let nodes = self.document.nodes();
            if let Some(root_node) = nodes.first() {
                if let Some(entry) = root_node.entries().first() {
                    match entry.value() {
                        KdlValue::Base10(i)
                        | KdlValue::Base16(i)
                        | KdlValue::Base8(i)
                        | KdlValue::Base2(i) => visitor.visit_i128(i128::from(*i)),
                        _ => self.deserialize_any(visitor),
                    }
                } else {
                    self.deserialize_any(visitor)
                }
            } else {
                self.deserialize_any(visitor)
            }
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // u128 support: KDL spec supports wide integers, but kdl crate 4.7 stores integers as i64
        // We can deserialize non-negative i64 values as u128
        if let Some(node) = self.current_node {
            if let Some(entry) = node.entries().first() {
                match entry.value() {
                    KdlValue::Base10(i)
                    | KdlValue::Base16(i)
                    | KdlValue::Base8(i)
                    | KdlValue::Base2(i) => {
                        if *i >= 0 {
                            visitor.visit_u128(*i as u128)
                        } else {
                            Err(Error::UnexpectedValueType {
                                field: "u128".to_string(),
                                expected: "non-negative integer".to_string(),
                                found: format!("negative integer {i}"),
                            })
                        }
                    }
                    _ => self.deserialize_any(visitor),
                }
            } else {
                self.deserialize_any(visitor)
            }
        } else {
            // Try root level
            let nodes = self.document.nodes();
            if let Some(root_node) = nodes.first() {
                if let Some(entry) = root_node.entries().first() {
                    match entry.value() {
                        KdlValue::Base10(i)
                        | KdlValue::Base16(i)
                        | KdlValue::Base8(i)
                        | KdlValue::Base2(i) => {
                            if *i >= 0 {
                                visitor.visit_u128(*i as u128)
                            } else {
                                Err(Error::UnexpectedValueType {
                                    field: "u128".to_string(),
                                    expected: "non-negative integer".to_string(),
                                    found: format!("negative integer {i}"),
                                })
                            }
                        }
                        _ => self.deserialize_any(visitor),
                    }
                } else {
                    self.deserialize_any(visitor)
                }
            } else {
                self.deserialize_any(visitor)
            }
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "bytes")]
        {
            // For bytes, we expect a hex string
            if let Some(node) = self.current_node {
                if let Some(entry) = node.entries().first() {
                    match entry.value() {
                        KdlValue::String(s) | KdlValue::RawString(s) => {
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
            } else {
                // Try root level
                let nodes = self.document.nodes();
                if let Some(root_node) = nodes.first() {
                    if let Some(entry) = root_node.entries().first() {
                        match entry.value() {
                            KdlValue::String(s) | KdlValue::RawString(s) => {
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
                } else {
                    Err(Error::UnsupportedType(
                        "no root node found for bytes".to_string(),
                    ))
                }
            }
        }
        #[cfg(not(feature = "bytes"))]
        {
            let _ = visitor; // Silence unused parameter warning
            Err(Error::UnsupportedType("byte arrays".to_string()))
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "bytes")]
        {
            // For byte_buf, we expect a hex string and return owned bytes
            if let Some(node) = self.current_node {
                if let Some(entry) = node.entries().first() {
                    match entry.value() {
                        KdlValue::String(s) | KdlValue::RawString(s) => {
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
            } else {
                // Try root level
                let nodes = self.document.nodes();
                if let Some(root_node) = nodes.first() {
                    if let Some(entry) = root_node.entries().first() {
                        match entry.value() {
                            KdlValue::String(s) | KdlValue::RawString(s) => {
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
                } else {
                    Err(Error::UnsupportedType(
                        "no root node found for bytes".to_string(),
                    ))
                }
            }
        }
        #[cfg(not(feature = "bytes"))]
        {
            let _ = visitor; // Silence unused parameter warning
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

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
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
                            if let KdlValue::String(s) | KdlValue::RawString(s) = entry.value() {
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
                            if let KdlValue::String(s) | KdlValue::RawString(s) = entry.value() {
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

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
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
        // Look for a node with the expected struct name
        let nodes = self.document.nodes();

        if let Some(struct_node) = nodes.iter().find(|n| n.name().value() == name) {
            self.current_node = Some(struct_node);

            // Create a struct deserializer from the node's children and properties
            let struct_de = StructDeserializer::new(struct_node);
            visitor.visit_map(struct_de)
        } else if nodes.len() == 1 {
            // If there's only one root node and it doesn't match the name,
            // still try to deserialize it as the struct
            let root_node = &nodes[0];
            self.current_node = Some(root_node);
            let struct_de = StructDeserializer::new(root_node);
            visitor.visit_map(struct_de)
        } else {
            Err(Error::UnexpectedNodeName {
                expected: name.to_string(),
                found: nodes
                    .first()
                    .map_or_else(|| "none".to_string(), |n| n.name().value().to_string()),
            })
        }
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
        // For enums, we expect either a simple string value or a node with variant name
        if let Some(node) = self.current_node {
            // Check if this is a simple string enum variant (node name is "root" and has string entry)
            if node.name().value() == "root" && node.entries().len() == 1 {
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
        } else {
            let nodes = self.document.nodes();
            if nodes.is_empty() {
                return Err(Error::MissingRootNode);
            }

            let root_node = &nodes[0];
            // Check if this is a simple string enum variant (node name is "root" and has string entry)
            if root_node.name().value() == "root" && root_node.entries().len() == 1 {
                if let Some(entry) = root_node.entries().first() {
                    if let KdlValue::String(s) = entry.value() {
                        use serde::de::value::StrDeserializer;
                        return visitor.visit_enum(StrDeserializer::<Error>::new(s.as_str()));
                    }
                }
            }

            let variant_name = root_node.name().value();
            visitor.visit_enum(EnumDeserializer::new(variant_name, root_node))
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

// Sequence deserializer
struct SeqDeserializer<'de> {
    entries: std::slice::Iter<'de, kdl::KdlEntry>,
    children: std::slice::Iter<'de, KdlNode>,
    mode: SeqMode,
}

enum SeqMode {
    Entries,
    Children,
}

impl<'de> SeqDeserializer<'de> {
    fn from_entries(entries: &'de [kdl::KdlEntry]) -> Self {
        Self {
            entries: entries.iter(),
            children: [].iter(),
            mode: SeqMode::Entries,
        }
    }

    fn from_children(children: &'de [KdlNode]) -> Self {
        Self {
            entries: [].iter(),
            children: children.iter(),
            mode: SeqMode::Children,
        }
    }
}

impl<'de> SeqAccess<'de> for SeqDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.mode {
            SeqMode::Entries => {
                if let Some(entry) = self.entries.next() {
                    // Use a specialized deserializer that can handle both simple values and enums
                    let de = EntryDeserializer::new(entry);
                    seed.deserialize(de).map(Some)
                } else {
                    Ok(None)
                }
            }
            SeqMode::Children => {
                if let Some(child) = self.children.next() {
                    let de = NodeDeserializer::new(child);
                    seed.deserialize(de).map(Some)
                } else {
                    Ok(None)
                }
            }
        }
    }
}

// Map deserializer
struct MapDeserializer<'de> {
    children: std::slice::Iter<'de, KdlNode>,
    current_key: Option<&'de str>,
    current_node: Option<&'de KdlNode>,
}

impl<'de> MapDeserializer<'de> {
    fn new(children: &'de [KdlNode]) -> Self {
        Self {
            children: children.iter(),
            current_key: None,
            current_node: None,
        }
    }
}

impl<'de> MapAccess<'de> for MapDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some(child) = self.children.next() {
            self.current_key = Some(child.name().value());
            self.current_node = Some(child);
            use serde::de::value::StrDeserializer;
            seed.deserialize(StrDeserializer::<Error>::new(child.name().value()))
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(node) = self.current_node.take() {
            let de = NodeDeserializer::new(node);
            seed.deserialize(de)
        } else {
            Err(Error::Serde("no current node for value".to_string()))
        }
    }
}

// Struct deserializer that handles both properties and children
struct StructDeserializer<'de> {
    node: &'de KdlNode,
    entry_index: usize,
    child_index: usize,
    current_field: Option<&'de str>,
    current_value: Option<FieldValue<'de>>,
}

#[derive(Clone)]
enum FieldValue<'de> {
    Property(&'de KdlValue),
    ChildNode(&'de KdlNode),
}

impl<'de> StructDeserializer<'de> {
    fn new(node: &'de KdlNode) -> Self {
        Self {
            node,
            entry_index: 0,
            child_index: 0,
            current_field: None,
            current_value: None,
        }
    }
}

impl<'de> MapAccess<'de> for StructDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // First, look for property entries
        while self.entry_index < self.node.entries().len() {
            let entry = &self.node.entries()[self.entry_index];
            self.entry_index += 1;

            if let Some(name) = entry.name() {
                // This is a property entry
                self.current_field = Some(name.value());
                self.current_value = Some(FieldValue::Property(entry.value()));
                use serde::de::value::StrDeserializer;
                return seed
                    .deserialize(StrDeserializer::<Error>::new(name.value()))
                    .map(Some);
            }
            // Skip non-property entries
        }

        // Then look for child nodes
        if let Some(children) = self.node.children() {
            if self.child_index < children.nodes().len() {
                let child = &children.nodes()[self.child_index];
                self.child_index += 1;

                self.current_field = Some(child.name().value());
                self.current_value = Some(FieldValue::ChildNode(child));
                use serde::de::value::StrDeserializer;
                return seed
                    .deserialize(StrDeserializer::<Error>::new(child.name().value()))
                    .map(Some);
            }
        }

        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(field_value) = self.current_value.take() {
            match field_value {
                FieldValue::Property(value) => {
                    // Create a temporary entry for the property value to handle enums
                    let entry = KdlEntry::new(value.clone());
                    let de = EntryDeserializer::new_owned(entry);
                    seed.deserialize(de)
                }
                FieldValue::ChildNode(node) => {
                    let de = NodeDeserializer::new(node);
                    seed.deserialize(de)
                }
            }
        } else {
            Err(Error::Serde("no current value for field".to_string()))
        }
    }
}

// Enum deserializer
struct EnumDeserializer<'de> {
    variant: &'de str,
    node: &'de KdlNode,
}

impl<'de> EnumDeserializer<'de> {
    fn new(variant: &'de str, node: &'de KdlNode) -> Self {
        Self { variant, node }
    }
}

impl<'de> EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = Error;
    type Variant = VariantDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        use serde::de::value::StrDeserializer;
        let variant_value = seed.deserialize(StrDeserializer::<Error>::new(self.variant))?;
        let variant_de = VariantDeserializer::new(self.node);
        Ok((variant_value, variant_de))
    }
}

// Variant deserializer
struct VariantDeserializer<'de> {
    node: &'de KdlNode,
}

impl<'de> VariantDeserializer<'de> {
    fn new(node: &'de KdlNode) -> Self {
        Self { node }
    }
}

impl<'de> VariantAccess<'de> for VariantDeserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        let de = NodeDeserializer::new(self.node);
        seed.deserialize(de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // First check if entries are directly in the node
        if !self.node.entries().is_empty() {
            let seq_de = SeqDeserializer::from_entries(self.node.entries());
            visitor.visit_seq(seq_de)
        } else if let Some(children) = self.node.children() {
            // Look for a "tuple" child node with the values
            if let Some(tuple_node) = children
                .nodes()
                .iter()
                .find(|n| n.name().value() == "tuple")
            {
                let seq_de = SeqDeserializer::from_entries(tuple_node.entries());
                visitor.visit_seq(seq_de)
            } else {
                visitor.visit_seq(SeqDeserializer::from_entries(&[]))
            }
        } else {
            visitor.visit_seq(SeqDeserializer::from_entries(&[]))
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let struct_de = StructDeserializer::new(self.node);
        visitor.visit_map(struct_de)
    }
}

/// A deserializer that works directly with a single `KdlNode`
struct NodeDeserializer<'de> {
    node: &'de KdlNode,
}

impl<'de> NodeDeserializer<'de> {
    fn new(node: &'de KdlNode) -> Self {
        Self { node }
    }

    fn deserialize_from_value<V>(&self, value: &'de KdlValue, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match value {
            KdlValue::String(s) => visitor.visit_str(s),
            KdlValue::RawString(s) => visitor.visit_str(s),
            KdlValue::Base10(i) => visitor.visit_i64(*i),
            KdlValue::Base16(i) => visitor.visit_i64(*i),
            KdlValue::Base8(i) => visitor.visit_i64(*i),
            KdlValue::Base2(i) => visitor.visit_i64(*i),
            KdlValue::Base10Float(f) => visitor.visit_f64(*f),
            KdlValue::Bool(b) => visitor.visit_bool(*b),
            KdlValue::Null => visitor.visit_unit(),
        }
    }
}

impl<'de> DeserializerTrait<'de> for NodeDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // If the node has multiple entries, treat as a sequence
        if self.node.entries().len() > 1 {
            let seq_de = SeqDeserializer::from_entries(self.node.entries());
            visitor.visit_seq(seq_de)
        } else if let Some(entry) = self.node.entries().first() {
            // Single entry - deserialize the value
            self.deserialize_from_value(entry.value(), visitor)
        } else if let Some(children) = self.node.children() {
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

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if !self.node.entries().is_empty() {
            let seq_de = SeqDeserializer::from_entries(self.node.entries());
            visitor.visit_seq(seq_de)
        } else if let Some(children) = self.node.children() {
            if children.nodes().is_empty() {
                visitor.visit_seq(SeqDeserializer::from_entries(&[]))
            } else {
                let seq_de = SeqDeserializer::from_children(children.nodes());
                visitor.visit_seq(seq_de)
            }
        } else {
            visitor.visit_seq(SeqDeserializer::from_entries(&[]))
        }
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
        // Create a struct deserializer from the node's children
        let struct_de = StructDeserializer::new(self.node);
        visitor.visit_map(struct_de)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "bytes")]
        {
            if let Some(entry) = self.node.entries().first() {
                match entry.value() {
                    KdlValue::String(s) | KdlValue::RawString(s) => {
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
            let _ = visitor; // Silence unused parameter warning
            Err(Error::UnsupportedType("byte arrays".to_string()))
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "bytes")]
        {
            if let Some(entry) = self.node.entries().first() {
                match entry.value() {
                    KdlValue::String(s) | KdlValue::RawString(s) => {
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
            let _ = visitor; // Silence unused parameter warning
            Err(Error::UnsupportedType("byte arrays".to_string()))
        }
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
        // Use the node name as the variant and the node content as the variant data
        let variant_name = self.node.name().value();
        visitor.visit_enum(EnumDeserializer::new(variant_name, self.node))
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        option unit unit_struct newtype_struct tuple
        tuple_struct map identifier ignored_any
    }
}

/// A deserializer that works directly with a single `KdlEntry`
/// This is useful for deserializing sequence elements that could be enums
enum EntryDeserializer<'de> {
    Borrowed(&'de kdl::KdlEntry),
    Owned(Box<kdl::KdlEntry>),
}

impl<'de> EntryDeserializer<'de> {
    fn new(entry: &'de kdl::KdlEntry) -> Self {
        Self::Borrowed(entry)
    }

    fn new_owned(entry: kdl::KdlEntry) -> Self {
        Self::Owned(Box::new(entry))
    }

    fn entry(&self) -> &kdl::KdlEntry {
        match self {
            Self::Borrowed(entry) => entry,
            Self::Owned(entry) => entry.as_ref(),
        }
    }
}

impl<'de> EntryDeserializer<'de> {
    fn deserialize_from_value<V>(&self, value: &KdlValue, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match value {
            KdlValue::String(s) => visitor.visit_str(s),
            KdlValue::RawString(s) => visitor.visit_str(s),
            KdlValue::Base10(i) => visitor.visit_i64(*i),
            KdlValue::Base16(i) => visitor.visit_i64(*i),
            KdlValue::Base8(i) => visitor.visit_i64(*i),
            KdlValue::Base2(i) => visitor.visit_i64(*i),
            KdlValue::Base10Float(f) => visitor.visit_f64(*f),
            KdlValue::Bool(b) => visitor.visit_bool(*b),
            KdlValue::Null => visitor.visit_unit(),
        }
    }
}

impl<'de> DeserializerTrait<'de> for EntryDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_from_value(self.entry().value(), visitor)
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
        // For simple values like strings in sequences, treat the value as the variant name
        if let KdlValue::String(s) = self.entry().value() {
            use serde::de::value::StrDeserializer;
            visitor.visit_enum(StrDeserializer::<Error>::new(s.as_str()))
        } else {
            // For other values, deserialize normally
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "bytes")]
        {
            match self.entry().value() {
                KdlValue::String(s) | KdlValue::RawString(s) => {
                    let bytes = crate::hex::decode_hex(s)?;
                    visitor.visit_bytes(&bytes)
                }
                _ => Err(Error::UnsupportedType(
                    "bytes must be represented as hex strings".to_string(),
                )),
            }
        }
        #[cfg(not(feature = "bytes"))]
        {
            let _ = visitor; // Silence unused parameter warning
            Err(Error::UnsupportedType("byte arrays".to_string()))
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "bytes")]
        {
            match self.entry().value() {
                KdlValue::String(s) | KdlValue::RawString(s) => {
                    let bytes = crate::hex::decode_hex(s)?;
                    visitor.visit_byte_buf(bytes)
                }
                _ => Err(Error::UnsupportedType(
                    "bytes must be represented as hex strings".to_string(),
                )),
            }
        }
        #[cfg(not(feature = "bytes"))]
        {
            let _ = visitor; // Silence unused parameter warning
            Err(Error::UnsupportedType("byte arrays".to_string()))
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "bytes")]
        {
            // Check if this is a hex string that should be deserialized as bytes
            match self.entry().value() {
                KdlValue::String(s) | KdlValue::RawString(s) => {
                    // Try to decode as hex - if successful, use bytes deserializer
                    if let Ok(bytes) = crate::hex::decode_hex(s) {
                        return visitor.visit_seq(BytesSeqDeserializer::new(bytes));
                    }
                }
                _ => {}
            }
        }

        // Fallback to normal any deserialization
        self.deserialize_any(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // If we have a value, it means Some(value)
        visitor.visit_some(self)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        unit unit_struct newtype_struct tuple
        tuple_struct map struct identifier ignored_any
    }
}

/// A specialized sequence deserializer for converting hex strings to bytes
#[cfg(feature = "bytes")]
struct BytesSeqDeserializer {
    bytes: Vec<u8>,
    index: usize,
}

#[cfg(feature = "bytes")]
impl BytesSeqDeserializer {
    fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, index: 0 }
    }
}

#[cfg(feature = "bytes")]
impl<'de> SeqAccess<'de> for BytesSeqDeserializer {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.index < self.bytes.len() {
            let byte = self.bytes[self.index];
            self.index += 1;
            use serde::de::value::U8Deserializer;
            seed.deserialize(U8Deserializer::<Error>::new(byte))
                .map(Some)
        } else {
            Ok(None)
        }
    }
}
