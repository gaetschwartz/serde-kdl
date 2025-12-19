use crate::error::{Error, Result};
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};
use serde::ser::SerializeMap;
use std::collections::BTreeMap;
use super::Serializer;

// Map serializer
pub struct SerializeMapImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) pending_key: Option<String>,
    pub(crate) items: BTreeMap<String, KdlNode>,
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
