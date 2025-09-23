use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    token::{Brace, Eq},
    Ident, Lit, LitStr, Path, Result, Token,
};

#[cfg(test)]
mod specs;
#[cfg(test)]
pub(crate) use specs::kdl_impl2;

const SERDE_KDL_KDL_EXPORT: SerdeKdlPrivate = SerdeKdlPrivate(&["kdl"]);
const KDL_NODE: SerdeKdlPrivate = SerdeKdlPrivate(&["kdl", "KdlNode"]);
const KDL_ENTRY: SerdeKdlPrivate = SerdeKdlPrivate(&["kdl", "KdlEntry"]);

/// A KDL macro that allows writing KDL syntax directly in Rust code.
///
/// # Examples
///
/// ```rust
/// use serde_kdl_macro::kdl;
///
/// // Simple node with value
/// let doc = kdl! { node 42 };
///
/// // Node with properties
/// let doc = kdl! { node key="value" 42 };
///
/// // Nested nodes
/// let doc = kdl! {
///     parent {
///         child "value"
///     }
/// };
/// ```
#[proc_macro]
pub fn kdl(input: TokenStream) -> TokenStream {
    match kdl_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

pub(crate) fn kdl_impl(input: TokenStream) -> Result<TokenStream2> {
    let document = syn::parse::<KdlDocument>(input)?;

    generate_kdl_code(&document)
}

/// Represents a complete KDL document containing multiple nodes
#[derive(Debug, Clone)]
struct KdlDocument {
    nodes: Vec<KdlNode>,
}

/// Represents a single KDL node with optional properties, arguments, and children
#[derive(Debug, Clone)]
struct KdlNode {
    name: String,
    properties: Vec<KdlProperty>,
    arguments: Vec<KdlValue>,
    children: Vec<KdlNode>,
    has_children_block: bool,
}

/// Represents a KDL property (key="value" pair)
#[derive(Debug, Clone)]
struct KdlProperty {
    key: Ident,
    value: KdlValue,
}

/// Represents a KDL value (string, number, boolean, etc.)
#[derive(Clone)]
enum KdlValue {
    String(LitStr),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
    TypeAnnotated {
        type_annotation: Ident,
        value: Box<KdlValue>,
    },
}

impl std::fmt::Debug for KdlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KdlValue::String(s) => write!(f, "String({})", s.value()),
            KdlValue::Integer(i) => write!(f, "Integer({})", i),
            KdlValue::Float(fl) => write!(f, "Float({})", fl),
            KdlValue::Boolean(b) => write!(f, "Boolean({})", b),
            KdlValue::Null => write!(f, "Null"),
            KdlValue::TypeAnnotated {
                type_annotation,
                value,
            } => {
                write!(f, "TypeAnnotated({}, {:?})", type_annotation, value)
            }
        }
    }
}

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

impl Parse for KdlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse node name (could be an identifier with dashes)
        let name = parse_node_name(input)?;
        let mut properties = Vec::new();
        let mut arguments = Vec::new();
        let mut children = Vec::new();
        let mut has_children_block = false;

        // Parse arguments and properties
        while !input.is_empty() && !input.peek(Brace) && !input.peek(Token![;]) {
            // Look ahead to see if this is a property (identifier followed by =)
            let checkpoint = input.fork();

            // Try to parse as property first (identifier followed by =)
            if checkpoint.peek(Ident) && checkpoint.peek2(Eq) {
                let key: Ident = input.parse()?;
                let _eq: Eq = input.parse()?;
                let value: KdlValue = input.parse()?;
                properties.push(KdlProperty { key, value });
            } else {
                // Check if this looks like the start of a new node
                // (identifier not followed by = that comes after we've already parsed some content)
                if !arguments.is_empty() || !properties.is_empty() {
                    if input.peek(Ident) {
                        // This looks like a new node name, stop parsing this node
                        break;
                    }
                }

                // Try to parse as argument (literal value or identifier)
                match input.parse::<KdlValue>() {
                    Ok(value) => arguments.push(value),
                    Err(e) => {
                        // If we can't parse as a value and there are more tokens, there's likely a syntax error
                        if !input.is_empty() && !input.peek(Brace) && !input.peek(Token![;]) {
                            return Err(e);
                        }
                        break; // No more tokens to parse
                    }
                }
            }
        }

        // Parse children if present
        if input.peek(Brace) {
            has_children_block = true;
            let content;
            syn::braced!(content in input);

            while !content.is_empty() {
                children.push(content.parse::<KdlNode>()?);
            }
        }

        Ok(KdlNode {
            name,
            properties,
            arguments,
            children,
            has_children_block,
        })
    }
}

// Helper function to parse node names that might have dashes
fn parse_node_name(input: ParseStream) -> Result<String> {
    let first_part: Ident = input.parse()?;
    let mut name_parts = vec![first_part.to_string()];

    // Check for dash-separated parts like "runs-on"
    while input.peek(syn::token::Minus) && input.peek2(Ident) {
        let _minus: syn::token::Minus = input.parse()?;
        let next_part: Ident = input.parse()?;
        name_parts.push("-".to_string());
        name_parts.push(next_part.to_string());
    }

    let full_name = name_parts.join("");
    Ok(full_name)
}

impl Parse for KdlValue {
    fn parse(input: ParseStream) -> Result<Self> {
        // Check for type annotation: (type)value
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let type_annotation: Ident = content.parse()?;
            let value = Box::new(input.parse::<KdlValue>()?);
            return Ok(KdlValue::TypeAnnotated {
                type_annotation,
                value,
            });
        }

        // Check for negative numbers first
        if input.peek(syn::token::Minus) {
            // Look ahead to see if this is a negative number or an identifier with dash
            let checkpoint = input.fork();
            let _minus: syn::token::Minus = checkpoint.parse()?;

            if checkpoint.peek(Lit) {
                // It's a negative number
                let _minus: syn::token::Minus = input.parse()?;
                let lit: Lit = input.parse()?;
                match lit {
                    Lit::Int(lit_int) => {
                        let value = -(lit_int.base10_parse::<i64>()?);
                        Ok(KdlValue::Integer(value))
                    }
                    Lit::Float(lit_float) => {
                        let value = -(lit_float.base10_parse::<f64>()?);
                        Ok(KdlValue::Float(value))
                    }
                    _ => Err(syn::Error::new(lit.span(), "Invalid negative literal")),
                }
            } else {
                // It's likely an identifier starting with minus, parse as a string identifier
                parse_identifier_sequence(input)
            }
        }
        // Check for string literals
        else if input.peek(LitStr) {
            let lit_str: LitStr = input.parse()?;
            Ok(KdlValue::String(lit_str))
        }
        // Check for boolean literals
        else if input.peek(syn::LitBool) {
            let boolean: syn::LitBool = input.parse()?;
            Ok(KdlValue::Boolean(boolean.value))
        }
        // Check for other literals (int, float, bool, str)
        else if input.peek(Lit) {
            let lit: Lit = input.parse()?;
            match lit {
                Lit::Int(lit_int) => {
                    let value = lit_int.base10_parse::<i64>()?;
                    Ok(KdlValue::Integer(value))
                }
                Lit::Float(lit_float) => {
                    let value = lit_float.base10_parse::<f64>()?;
                    Ok(KdlValue::Float(value))
                }
                Lit::Bool(lit_bool) => Ok(KdlValue::Boolean(lit_bool.value)),
                Lit::Str(lit_str) => Ok(KdlValue::String(lit_str)),
                _ => Err(syn::Error::new(lit.span(), "Unsupported literal type")),
            }
        }
        // Check for # syntax (like #true, #false)
        else if input.peek(syn::token::Pound) {
            let _pound: syn::token::Pound = input.parse()?;
            if input.peek(Ident) {
                let ident: Ident = input.parse()?;
                let ident_str = ident.to_string();
                if ident_str == "true" {
                    Ok(KdlValue::Boolean(true))
                } else if ident_str == "false" {
                    Ok(KdlValue::Boolean(false))
                } else if ident_str == "null" {
                    Ok(KdlValue::Null)
                } else {
                    let span = input.span();
                    Ok(KdlValue::String(LitStr::new(
                        &format!("#{}", ident_str),
                        span,
                    )))
                }
            } else if input.peek(syn::LitBool) {
                // Handle #true and #false when true/false are literals, not identifiers
                let boolean: syn::LitBool = input.parse()?;
                Ok(KdlValue::Boolean(boolean.value))
            } else {
                Err(syn::Error::new(
                    input.span(),
                    "Expected identifier or literal after #",
                ))
            }
        }
        // Check for identifiers
        else if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            let identifier = ident.to_string();
            if identifier == "null" {
                Ok(KdlValue::Null)
            } else if identifier == "true" {
                Ok(KdlValue::Boolean(true))
            } else if identifier == "false" {
                Ok(KdlValue::Boolean(false))
            } else {
                // Check if there are more tokens that form a compound identifier
                let mut identifier_parts = vec![identifier];

                // Try to parse dash-separated identifiers like "ubuntu-latest"
                while input.peek(syn::token::Minus) && input.peek2(Ident) {
                    let _minus: syn::token::Minus = input.parse()?;
                    let next_part: Ident = input.parse()?;
                    identifier_parts.push("-".to_string());
                    identifier_parts.push(next_part.to_string());
                }

                let full_identifier = identifier_parts.join("");
                let span = input.span();
                Ok(KdlValue::String(LitStr::new(&full_identifier, span)))
            }
        } else {
            Err(syn::Error::new(
                input.span(),
                "Expected string, number, boolean, null, or identifier",
            ))
        }
    }
}

// Helper function to parse identifier sequences like "ubuntu-latest"
fn parse_identifier_sequence(input: ParseStream) -> Result<KdlValue> {
    let mut parts = String::new();

    // Handle first token (could be minus or identifier)
    if input.peek(syn::token::Minus) {
        let _minus: syn::token::Minus = input.parse()?;
        parts.push('-');
    }

    // Parse the rest of the sequence
    while input.peek(Ident) {
        let ident: Ident = input.parse()?;
        parts.push_str(&ident.to_string());

        // Check for more dash-identifier pairs
        if input.peek(syn::token::Minus) && input.peek2(Ident) {
            let _minus: syn::token::Minus = input.parse()?;
            parts.push('-');
        } else {
            break;
        }
    }

    if parts.is_empty() {
        return Err(syn::Error::new(input.span(), "Expected identifier"));
    }

    let span = input.span();
    Ok(KdlValue::String(LitStr::new(&parts, span)))
}

fn generate_kdl_code(document: &KdlDocument) -> Result<TokenStream2> {
    let node_codes: Result<Vec<_>> = document.nodes.iter().map(generate_node_code).collect();
    let node_codes = node_codes?;

    Ok(quote! {
        {
            let mut document = #SERDE_KDL_KDL_EXPORT::KdlDocument::new();
            #(#node_codes)*
            document
        }
    })
}

fn generate_node_code(node: &KdlNode) -> Result<TokenStream2> {
    let name = &node.name;

    // Generate argument codes
    let arg_codes: Result<Vec<_>> = node.arguments.iter().map(generate_value_code).collect();
    let arg_codes = arg_codes?;

    // Generate property codes
    let prop_codes: Result<Vec<_>> = node
        .properties
        .iter()
        .map(|prop| {
            let key = prop.key.to_string();
            let value_code = generate_value_code(&prop.value)?;
            Ok(quote! {
                node.entries_mut().push(#SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value_code));
            })
        })
        .collect();
    let prop_codes = prop_codes?;

    // Generate children codes
    let children_code = if node.has_children_block {
        let child_codes: Result<Vec<_>> =
            node.children.iter().map(generate_child_node_code).collect();
        let child_codes = child_codes?;

        quote! {
            {
                let mut children_doc = #SERDE_KDL_KDL_EXPORT::KdlDocument::new();
                #(
                    {
                        #child_codes
                        children_doc.nodes_mut().push(child_node);
                    }
                )*
                node.set_children(children_doc);
            }
        }
    } else {
        quote! {}
    };

    let kdl_entry = &KDL_ENTRY;
    Ok(quote! {
        {
            let mut node = #KDL_NODE::new(#name);
            #(
                {
                    let entry = <#kdl_entry>::new(#arg_codes);
                    node.entries_mut().push(entry);
                }
            )*
            #(#prop_codes)*
            #children_code
            document.nodes_mut().push(node);
        }
    })
}

fn generate_child_node_code(node: &KdlNode) -> Result<TokenStream2> {
    let name = &node.name;

    // Generate argument codes
    let arg_codes: Result<Vec<_>> = node.arguments.iter().map(generate_value_code).collect();
    let arg_codes = arg_codes?;

    // Generate property codes
    let prop_codes: Result<Vec<_>> = node
        .properties
        .iter()
        .map(|prop| {
            let key = prop.key.to_string();
            let value_code = generate_value_code(&prop.value)?;
            Ok(quote! {
                child_node.entries_mut().push(#SERDE_KDL_KDL_EXPORT::KdlEntry::new_prop(#key, #value_code));
            })
        })
        .collect();
    let prop_codes = prop_codes?;

    // Generate nested children codes
    let nested_children_code = if node.has_children_block {
        let nested_child_codes: Result<Vec<_>> =
            node.children.iter().map(generate_child_node_code).collect();
        let nested_child_codes = nested_child_codes?;

        quote! {
            {
                let mut nested_children_doc = #SERDE_KDL_KDL_EXPORT::KdlDocument::new();
                #(
                    {
                        #nested_child_codes
                        nested_children_doc.nodes_mut().push(child_node);
                    }
                )*
                child_node.set_children(nested_children_doc);
            }
        }
    } else {
        quote! {}
    };

    let kdl_entry = &KDL_ENTRY;
    Ok(quote! {
        let mut child_node = #KDL_NODE::new(#name);
        #(
            child_node.entries_mut().push(#kdl_entry::new(#arg_codes));
        )*
        #(#prop_codes)*
        #nested_children_code
    })
}

fn generate_value_code(value: &KdlValue) -> Result<TokenStream2> {
    match value {
        KdlValue::String(lit_str) => {
            let s = lit_str.value();
            Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::String(#s.to_string()) })
        }
        KdlValue::Integer(i) => Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Base10(#i) }),
        KdlValue::Float(f) => Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Base10Float(#f) }),
        KdlValue::Boolean(b) => Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Bool(#b) }),
        KdlValue::Null => Ok(quote! { #SERDE_KDL_KDL_EXPORT::KdlValue::Null }),
        KdlValue::TypeAnnotated {
            type_annotation,
            value,
        } => {
            let type_str = type_annotation.to_string();
            let inner_value = generate_value_code(value)?;
            Ok(quote! {
                {
                    let mut val = #inner_value;
                    val.set_type(Some(#type_str));
                    val
                }
            })
        }
    }
}

struct SerdeKdlPrivate<'a>(&'a [&'static str]);

impl ToTokens for SerdeKdlPrivate<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let path = Path {
            leading_colon: Some(Token![::](proc_macro2::Span::call_site())),
            segments: self
                .0
                .iter()
                .map(|s| syn::PathSegment {
                    ident: format_ident!("{}", s),
                    arguments: syn::PathArguments::None,
                })
                .collect(),
        };
        path.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_simple_node_parsing() {
        let input: KdlDocument = parse_quote! { node 42 };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].name, "node");
        assert_eq!(input.nodes[0].arguments.len(), 1);
        assert!(matches!(input.nodes[0].arguments[0], KdlValue::Integer(42)));
    }

    #[test]
    fn test_node_with_properties() {
        let input: KdlDocument = parse_quote! { node key="value" 42 };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].properties.len(), 1);
        assert_eq!(input.nodes[0].properties[0].key.to_string(), "key");
        assert_eq!(input.nodes[0].arguments.len(), 1);
    }

    #[test]
    fn test_nested_nodes() {
        let input: KdlDocument = parse_quote! {
            parent {
                child "value"
            }
        };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].children.len(), 1);
        assert_eq!(input.nodes[0].children[0].name, "child");
    }

    #[test]
    fn test_multiple_root_nodes() {
        let input: KdlDocument = parse_quote! {
            node1 "value1"
            node2 "value2"
        };
        assert_eq!(input.nodes.len(), 2);
        assert_eq!(input.nodes[0].name, "node1");
        assert_eq!(input.nodes[1].name, "node2");
    }

    #[test]
    fn test_hyphenated_identifiers() {
        let input: KdlDocument = parse_quote! { runs-on "ubuntu-latest" };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].name, "runs-on");
        assert_eq!(input.nodes[0].arguments.len(), 1);
    }

    #[test]
    fn test_boolean_values() {
        let input: KdlDocument = parse_quote! { node true false #true #false };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].arguments.len(), 4);
        assert!(matches!(
            input.nodes[0].arguments[0],
            KdlValue::Boolean(true)
        ));
        assert!(matches!(
            input.nodes[0].arguments[1],
            KdlValue::Boolean(false)
        ));
        assert!(matches!(
            input.nodes[0].arguments[2],
            KdlValue::Boolean(true)
        ));
        assert!(matches!(
            input.nodes[0].arguments[3],
            KdlValue::Boolean(false)
        ));
    }

    #[test]
    fn test_mixed_arguments_and_properties() {
        let input: KdlDocument = parse_quote! { node "arg1" key="value" "arg2" };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].arguments.len(), 2);
        assert_eq!(input.nodes[0].properties.len(), 1);
        assert_eq!(input.nodes[0].properties[0].key.to_string(), "key");
    }

    #[test]
    fn test_keywords_as_identifiers() {
        let input: KdlDocument = parse_quote! { r#override #true };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].name, "r#override");
        assert_eq!(input.nodes[0].arguments.len(), 1);
        assert!(matches!(
            input.nodes[0].arguments[0],
            KdlValue::Boolean(true)
        ));
    }

    #[test]
    fn test_string_values() {
        let input: KdlDocument = parse_quote! { node "string" "other-value" };
        assert_eq!(input.nodes.len(), 1);
        assert_eq!(input.nodes[0].arguments.len(), 2);
        assert!(matches!(input.nodes[0].arguments[0], KdlValue::String(_)));
        assert!(matches!(input.nodes[0].arguments[1], KdlValue::String(_)));
    }
}
