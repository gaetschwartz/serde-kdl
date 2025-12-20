//! Node parsing
//!
//! This module handles parsing of KDL nodes including node names,
//! properties, arguments, and children.

use crate::{
    ast::{KdlDocument, KdlEntry, KdlIdentifier, KdlNode, KdlValue, Terminator},
    parse::type_annotation::MaybeAnnotated,
};
use syn::{
    parse::{discouraged::Speculative as _, Parse, ParseStream},
    token::Brace,
    Result, Token,
};

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
        while !input.is_empty() && !input.peek(Brace) && !input.peek(Token![;]) {
            // Try to parse as argument (literal value or identifier)
            let fork = input.fork();
            let value = fork.parse::<MaybeAnnotated<KdlValue>>()?;

            // eprintln!(
            //     "[node({name})] Parsed entry: {value:?} at {}",
            //     SpanDisplay(value.span())
            // );
            let value_line = value.span().start().line;

            if value_line != node_line {
                // Different line = new node
                terminator = Some(Terminator::Eol);
                // eprintln!("[node({name})] Value on different line, ending arguments/properties. (node: {}, value: {})",
                //     SpanDisplay(name.span()),
                //     SpanDisplay(value.span()),
                // );
                break;
            }
            input.advance_to(&fork);

            // Check if it's a property (key=value) or argument (value)
            if input.peek(Token![=]) {
                let _: Token![=] = input.parse()?;
                // eprintln!("[node({name})] Found '=' token ");
                // It's a property
                let key = value.try_into()?;
                let prop_value = input.parse::<MaybeAnnotated<KdlValue>>()?;
                let prop_line = prop_value.span().start().line;
                if value_line != prop_line {
                    return Err(syn::Error::new(
                        prop_value.span(),
                        "Property values must be on the same line as their keys.",
                    ));
                }

                entries.push(KdlEntry {
                    name: Some(key),
                    value: prop_value,
                });
            } else {
                // It's an argument
                entries.push(KdlEntry { name: None, value });
            }
        }

        // Parse children if present
        let terminator = if let Some(t) = terminator {
            t
        } else if input.peek(Brace) {
            let content;
            syn::braced!(content in input);

            let children = children.get_or_insert_with(Vec::new);

            while !content.is_empty() {
                let node = content.parse::<KdlNode>()?;
                children.push(node);
            }
            Terminator::Brace
        } else if input.peek(Token![;]) {
            Terminator::Semicolon
        } else if input.is_empty() {
            Terminator::Eof
        } else {
            return Err(syn::Error::new(
                input.span(),
                "Unexpected token after node arguments/properties.",
            ));
        };
        while input.peek(Token![;]) {
            let _sep: Token![;] = input.parse()?;
        }

        let children = children.map(KdlDocument::from_nodes);

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
