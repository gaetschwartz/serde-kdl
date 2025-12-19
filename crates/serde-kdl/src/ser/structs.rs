use crate::error::{Error, Result};
use kdl::{KdlDocument, KdlNode};
use serde::ser::{SerializeStruct, SerializeStructVariant};
use super::Serializer;

// Struct serializer
pub struct SerializeStructImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) is_root: bool,
}

impl SerializeStruct for SerializeStructImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        // Serialize the field value using a temporary serializer
        let mut field_serializer = Serializer::new_for_field();
        value.serialize(&mut field_serializer)?;
        let field_doc = field_serializer.into_document();

        // If the field serialized to nothing (empty document), skip it
        // This handles Option::None which should be skipped
        if field_doc.nodes().is_empty() {
            return Ok(());
        }

        // Get the first node from the serialized field value
        let node = field_doc.nodes().first().unwrap();

        // Create a child node with the field name
        let mut field_node = KdlNode::new(key);

        // Copy all entries (arguments) from the serialized value
        for entry in node.entries() {
            field_node.entries_mut().push(entry.clone());
        }

        // Copy children if any (for complex structures)
        if let Some(node_children) = node.children() {
            if !node_children.nodes().is_empty() {
                *field_node.children_mut() = Some(node_children.clone());
            }
        }

        // Copy type annotation if present (for enum variants)
        if let Some(ty) = node.ty() {
            field_node.set_ty(ty.clone());
        }

        // Add the field node to the appropriate location
        if self.is_root {
            // Root level: add field node directly to document
            self.ser.document.nodes_mut().push(field_node);
        } else {
            // Nested struct: add field node to the current context's children
            if let Some((_name, children)) = self.ser.node_stack.last_mut() {
                children.push(field_node);
            }
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

// Struct variant serializer
pub struct SerializeStructVariantImpl<'a> {
    pub(crate) ser: &'a mut Serializer,
    pub(crate) variant: String,
}

impl SerializeStructVariant for SerializeStructVariantImpl<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        // Serialize the field value using a temporary serializer
        let mut field_serializer = Serializer::new_for_field();
        value.serialize(&mut field_serializer)?;
        let field_doc = field_serializer.into_document();

        // If the field serialized to nothing, skip it
        if field_doc.nodes().is_empty() {
            return Ok(());
        }

        // Get the first node from the serialized field value
        let node = field_doc.nodes().first().unwrap();

        // Create a child node with the field name
        let mut field_node = KdlNode::new(key);

        // Copy all entries (arguments) from the serialized value
        for entry in node.entries() {
            field_node.entries_mut().push(entry.clone());
        }

        // Copy children if any (for complex structures)
        if let Some(node_children) = node.children() {
            if !node_children.nodes().is_empty() {
                *field_node.children_mut() = Some(node_children.clone());
            }
        }

        // Copy type annotation if present (for enum variants)
        if let Some(ty) = node.ty() {
            field_node.set_ty(ty.clone());
        }

        // Add to the document directly (will be wrapped with variant type annotation at end)
        self.ser.document.nodes_mut().push(field_node);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        // Create a node with "-" as name and the variant as type annotation
        let mut node = KdlNode::new(crate::DEFAULT_NODE_NAME);
        node.set_ty(kdl::KdlIdentifier::from(self.variant.as_str()));

        // Move all the field nodes we added to the document into this node's children
        let mut child_doc = KdlDocument::new();
        for field_node in self.ser.document.nodes() {
            child_doc.nodes_mut().push(field_node.clone());
        }
        self.ser.document.nodes_mut().clear();

        *node.children_mut() = Some(child_doc);
        self.ser.current_node = Some(node);
        Ok(())
    }
}
