use std::mem;

use super::Serializer;
use crate::{
    error::{Error, Result},
    ser::SerializeFieldValue,
    DEFAULT_NODE_NAME,
};
use kdl::{KdlDocument, KdlNode, KdlValue};
use serde::ser::SerializeMap;

// Map serializer
pub struct SerializeMapImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) pending_key: Option<String>,
    pub(crate) items: Vec<KdlNode>,
}

impl SerializeMap for SerializeMapImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let node = key.into_option_node()?;
        if let Some(mut node) = node {
            if let Some(mut entry) = node.entries_mut().pop() {
                if entry.name().is_some_and(|e| e.value() != DEFAULT_NODE_NAME) {
                    return Err(Error::InvalidMapKey(Box::new(node)));
                }
                let KdlValue::String(s) = entry.value_mut() else {
                    return Err(Error::InvalidMapKey(Box::new(node)));
                };
                self.pending_key = Some(mem::take(s))
            } else {
                return Err(Error::InvalidMapKey(Box::new(node)));
            }
        }

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let Some(key) = self.pending_key.take() else {
            return Err(Error::KeyMissingInMapSerialization);
        };

        let mut node = value.into_node()?;
        node.set_name(key);
        self.items.push(node);

        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok> {
        self.items
            .sort_by(|a, b| a.name().value().cmp(b.name().value()));
        // Check if we're at the root level (no wrapper needed for flatten)
        let is_root = self.ser.is_root_serializer && self.ser.node_stack.is_empty();

        if is_root {
            // Root-level flatten: add items directly to document
            self.ser.document.nodes_mut().extend(self.items);
        } else {
            // Non-root: create wrapper node
            let mut node = KdlNode::new(DEFAULT_NODE_NAME);
            if !self.items.is_empty() {
                let mut child_doc = KdlDocument::new();
                child_doc.nodes_mut().extend(self.items);
                *node.children_mut() = Some(child_doc);
            }
            self.ser.current_node = Some(node);
        }
        Ok(())
    }
}
