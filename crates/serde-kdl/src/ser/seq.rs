use super::Serializer;
use crate::{
    DEFAULT_NODE_NAME,
    error::{Error, Result},
    ser::SerializeFieldValue,
};
use kdl::{KdlDocument, KdlNode};
use serde::ser::SerializeSeq;

// Sequence serializer
pub struct SerializeSeqImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) child_nodes: Vec<KdlNode>,
}

impl SerializeSeq for SerializeSeqImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut node = value.into_node()?;
        node.set_name(DEFAULT_NODE_NAME);
        self.child_nodes.push(node);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        // Check if we're at the root level (no wrapper needed)
        let is_root = self.ser.is_root_serializer && self.ser.node_stack.is_empty();

        if is_root {
            // Root-level: add sequence items directly to document
            self.ser.document.nodes_mut().extend(self.child_nodes);
        } else {
            // Nested: wrap in a "-" node with children
            let mut node = KdlNode::new(DEFAULT_NODE_NAME);
            let mut child_doc = KdlDocument::new();
            child_doc.nodes_mut().extend(self.child_nodes);
            *node.children_mut() = Some(child_doc);
            self.ser.current_node = Some(node);
        }
        Ok(())
    }
}
