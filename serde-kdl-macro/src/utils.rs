//! Utility functions for KDL processing
//!
//! This module contains utility functions like line continuation processing
//! and other helper functions used throughout the crate.

use syn::Result;

/// Processes line continuations in KDL input according to Section 3.3 of the KDL specification.
///
/// A line continuation is a `\` character followed by zero or more whitespace items
/// (including multiline comments) and an optional single-line comment, terminated by a newline.
/// Following a line continuation, processing of a Node can continue as usual.
pub(crate) fn process_line_continuation_string(input: &str) -> Result<String> {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        if chars[i] == '\\' {
            // Found a potential line continuation
            let mut j = i + 1;

            // Skip whitespace after the backslash (but not newlines)
            while j < chars.len()
                && chars[j].is_whitespace()
                && chars[j] != '\n'
                && chars[j] != '\r'
            {
                j += 1;
            }

            // Check for optional single-line comment
            if j < chars.len() && chars[j] == '/' && j + 1 < chars.len() && chars[j + 1] == '/' {
                j += 2; // Skip //
                        // Skip the rest of the single-line comment until newline
                while j < chars.len() && chars[j] != '\n' && chars[j] != '\r' {
                    j += 1;
                }
            }

            // Must be terminated by a newline
            if j < chars.len() && (chars[j] == '\n' || chars[j] == '\r') {
                // This is a valid line continuation
                j += 1; // consume the newline

                // If it's \r\n, consume both characters
                if j < chars.len() && chars[j - 1] == '\r' && chars[j] == '\n' {
                    j += 1;
                }

                // Skip leading whitespace on the next line after the line continuation
                while j < chars.len() && (chars[j] == ' ' || chars[j] == '\t') {
                    j += 1;
                }

                // Add a space to maintain token separation only if the last character isn't already whitespace
                if !result.is_empty() && !result.chars().last().unwrap().is_whitespace() {
                    result.push(' ');
                }
                i = j;
                continue;
            } else {
                // Not a line continuation, add the backslash as-is
                result.push(chars[i]);
                i += 1;
            }
        } else {
            // Regular character, add it to result
            result.push(chars[i]);
            i += 1;
        }
    }

    Ok(result)
}
