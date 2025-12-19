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
    pub(crate) ser: &'a mut Serializer,
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
