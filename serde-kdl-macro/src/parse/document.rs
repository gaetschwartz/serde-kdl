//! Document parsing
//!
//! This module handles parsing of complete KDL documents.

use crate::ast::{KdlDocument, KdlNode};
use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

impl Parse for KdlDocument {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            nodes.push(input.parse::<KdlNode>()?);

            // Skip optional semicolons between nodes
            if input.peek(Token![;]) {
                let _: Token![;] = input.parse()?;
            }
        }

        Ok(KdlDocument { nodes })
    }
}
