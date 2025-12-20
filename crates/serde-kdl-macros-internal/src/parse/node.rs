//! Node parsing
//!
//! This module handles parsing of KDL nodes including node names,
//! properties, arguments, and children.

use crate::parse::{
    comments::{MaybeSlashed, SlashDash},
    document::KdlDocument,
    entry::KdlEntry,
    identifier::KdlIdentifier,
    type_annotation::MaybeAnnotated,
};
use syn::{
    Result, Token,
    parse::{Parse, ParseStream, discouraged::Speculative as _},
    token::Brace,
};

/// Represents a single KDL node with optional properties, arguments, and children
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct KdlNode {
    pub name: KdlIdentifier,
    pub type_annotation: Option<KdlIdentifier>, // Type annotation for node name
    pub entries: Vec<KdlEntry>,
    pub children: Option<KdlDocument>,
    pub terminator: Terminator,
    /// Entries that are commented out with `/-`
    pub ignored_entries: Vec<KdlEntry>,
    /// Children block that is commented out with `/-`
    pub ignored_children: Option<KdlDocument>,
}

impl KdlNode {
    #[must_use]
    pub fn ty(&self) -> Option<&KdlIdentifier> {
        self.type_annotation.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Terminator {
    Brace,
    Semicolon,
    Eol,
    Eof,
}

impl Parse for KdlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse optional type annotation for node name
        let MaybeAnnotated {
            type_annotation,
            item: name,
        } = input.parse::<MaybeAnnotated<KdlIdentifier>>()?;

        // eprintln!("[node({name})] Name span: {}", SpanDisplay(name.span()));

        // FIXME: This doesn't work in rust-analyzer since it returns dummy spans (1:0)
        let node_line = name.span().end().line;

        let mut entries = Vec::new();
        let mut ignored_entries = Vec::new();
        let mut children = None;
        let mut ignored_children = None;
        let mut terminator = None;

        // Parse arguments and properties
        while !input.is_empty() && !input.peek(Brace) && !input.peek(Token![;]) {
            // Check if this is a slashed children block (/-{ ... })
            // If so, break and let the children parsing handle it
            if SlashDash::peek(input) && input.peek3(Brace) {
                break;
            }

            // Try to parse as argument (literal value or identifier)
            let fork = input.fork();
            let maybe_entry = fork.parse::<MaybeSlashed<KdlEntry>>()?;
            let entry = maybe_entry.inner();

            let value_line = entry.span().start().line;

            if value_line != node_line {
                // Different line = new node
                terminator = Some(Terminator::Eol);
                break;
            }

            input.advance_to(&fork);
            match maybe_entry {
                MaybeSlashed::Item(e) => entries.push(e),
                MaybeSlashed::Slashed(e) => ignored_entries.push(e),
            }
        }

        // Parse children if present
        let fork = input.fork();
        let terminator = if let Some(t) = terminator {
            t
        } else if let Ok(c) = fork.parse::<MaybeSlashed<ChildrenBlock>>() {
            match c {
                MaybeSlashed::Item(c) => {
                    children = Some(KdlDocument::from_nodes(c.nodes));
                }
                MaybeSlashed::Slashed(c) => {
                    ignored_children = Some(KdlDocument::from_nodes(c.nodes));
                }
            };
            input.advance_to(&fork);

            Terminator::Brace
        } else if input.peek(Token![;]) {
            Terminator::Semicolon
        } else if input.is_empty() {
            Terminator::Eof
        } else {
            return Err(syn::Error::new(
                input.span(),
                format!(
                    "Unexpected token after node arguments/properties: {}",
                    input.fork().cursor().token_stream(),
                ),
            ));
        };
        while input.peek(Token![;]) {
            let _sep: Token![;] = input.parse()?;
        }

        let kdl_node = KdlNode {
            name,
            type_annotation,
            entries,
            children,
            terminator,
            ignored_entries,
            ignored_children,
        };

        Ok(kdl_node)
    }
}

impl PartialEq<kdl::KdlNode> for KdlNode {
    fn eq(&self, other: &kdl::KdlNode) -> bool {
        if &self.name != other.name() {
            return false;
        }

        match (&self.type_annotation, other.ty()) {
            (Some(a), Some(b)) if a != b => return false,
            (None, Some(_)) | (Some(_), None) => return false,
            _ => {}
        }

        let my_entries = &self.entries;
        let other_entries = other.entries();

        if my_entries != other_entries {
            return false;
        }

        match (&self.children, other.children()) {
            (Some(a), Some(b)) if a != b => return false,
            (None, Some(_)) | (Some(_), None) => return false,
            _ => {}
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct ChildrenBlock {
    pub nodes: Vec<KdlNode>,
}

impl Parse for ChildrenBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        syn::braced!(content in input);

        let mut nodes = Vec::new();

        while !content.is_empty() {
            let node = content.parse::<KdlNode>()?;
            nodes.push(node);
        }

        Ok(ChildrenBlock { nodes })
    }
}
