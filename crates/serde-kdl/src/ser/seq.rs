use super::Serializer;
use crate::{
    error::{Error, Result},
    ser::SerializeFieldValue,
    DEFAULT_NODE_NAME,
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
        // Create the parent node with "-" as the name
        let mut node = KdlNode::new(DEFAULT_NODE_NAME);

        // If we have child nodes, add them in a children block
        // Even if empty, we still create an empty children block for empty sequences
        let mut child_doc = KdlDocument::new();
        child_doc.nodes_mut().extend(self.child_nodes);
        *node.children_mut() = Some(child_doc);

        self.ser.current_node = Some(node);
        Ok(())
    }
}
