//! Node parsing
//!
//! This module handles parsing of KDL nodes including node names,
//! properties, arguments, and children.

use std::ops::Deref;

use crate::parse::{
    comments::MaybeSlashed, document::KdlDocument, entry::KdlEntry, identifier::KdlIdentifier,
    type_annotation::MaybeAnnotated,
};
use syn::{
    Result, Token,
    parse::{Parse, ParseStream, discouraged::Speculative as _},
};

/// Represents a single KDL node with optional properties, arguments, and children
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct KdlNode {
    pub name: KdlIdentifier,
    pub type_annotation: Option<KdlIdentifier>, // Type annotation for node name
    pub entries: Vec<MaybeSlashed<KdlEntry>>,
    pub children: Option<MaybeSlashed<ChildrenBlock>>,
    pub terminator: Terminator,
}

impl KdlNode {
    #[must_use]
    pub fn ty(&self) -> Option<&KdlIdentifier> {
        self.type_annotation.as_ref()
    }

    pub fn entries(&self) -> impl Iterator<Item = &KdlEntry> {
        self.entries.iter().filter_map(|e| e.as_option())
    }

    #[must_use]
    pub fn children(&self) -> Option<Option<&KdlDocument>> {
        self.children
            .as_ref()
            .map(|c| c.as_option().map(|cb| &cb.0))
    }
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
        let mut children = None;
        let mut terminator = None;

        // Parse arguments and properties
        while !(input.is_empty()
            || input.peek(Token![;])
            || MaybeSlashed::<ChildrenBlock>::peek(input))
        {
            // Try to parse as argument (literal value or identifier)
            let fork = input.fork();
            let maybe_entry = fork.parse::<MaybeSlashed<KdlEntry>>()?;
            // eprintln!(
            //     "[node({})] Parsed entry at span {}: {:?}\nRest of input: `{}`",
            //     name,
            //     SpanDisplay(maybe_entry.inner().span()),
            //     maybe_entry,
            //     fork
            // );
            let entry = maybe_entry.inner();
            let value_line = entry.span().start().line;
            if value_line != node_line {
                // Different line = new node
                // eprintln!(
                //     "[node({})] Entry at line {} differs from node line {}: ending entries parse",
                //     name, value_line, node_line
                // );
                terminator = Some(Terminator::Eol);
                break;
            }

            input.advance_to(&fork);
            entries.push(maybe_entry);
        }

        // eprintln!("[node({})] Parsed {} entries", name, entries.len());

        // Parse children if present
        let terminator = if let Some(t) = terminator {
            t
        } else if <MaybeSlashed<ChildrenBlock>>::peek(input) {
            let c = input.parse::<MaybeSlashed<ChildrenBlock>>()?;
            children = Some(c);

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
        _ = input.parse::<Option<Token![;]>>()?;

        let kdl_node = KdlNode {
            name,
            type_annotation,
            entries,
            children,
            terminator,
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

        let my_entries = self.entries();
        let other_entries = other.entries();

        if std::iter::Iterator::ne(my_entries, other_entries.iter()) {
            return false;
        }

        match (self.children().flatten(), other.children()) {
            (Some(a), Some(b)) if a != b => return false,
            (None, Some(_)) | (Some(_), None) => return false,
            _ => {}
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct ChildrenBlock(pub KdlDocument);

impl Parse for ChildrenBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        syn::braced!(content in input);

        let mut nodes = Vec::new();

        while !content.is_empty() {
            let node = content.parse::<MaybeSlashed<KdlNode>>()?;
            nodes.push(node);
        }

        Ok(ChildrenBlock(KdlDocument::from_nodes(nodes)))
    }
}

impl Deref for ChildrenBlock {
    type Target = KdlDocument;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Terminator {
    Brace,
    Semicolon,
    Eol,
    Eof,
}
