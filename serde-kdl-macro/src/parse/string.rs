//! String parsing and escape sequence processing
//!
//! This module handles parsing of KDL strings, including all escape sequences
//! and validation according to Section 3.11 of the KDL specification.

use syn::Result;

/// Processes all escape sequences in quoted strings according to Section 3.11
pub(crate) fn process_string_escapes(value: &str, span: proc_macro2::Span) -> Result<String> {
    let mut result = String::new();
    let mut chars = value.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next_ch) = chars.peek() {
                match next_ch {
                    // Standard escape sequences (Section 3.11.1)
                    'n' => {
                        chars.next(); // consume 'n'
                        result.push('\u{000A}'); // Line Feed
                    }
                    'r' => {
                        chars.next(); // consume 'r'
                        result.push('\u{000D}'); // Carriage Return
                    }
                    't' => {
                        chars.next(); // consume 't'
                        result.push('\u{0009}'); // Character Tabulation (Tab)
                    }
                    '\\' => {
                        chars.next(); // consume '\'
                        result.push('\u{005C}'); // Reverse Solidus (Backslash)
                    }
                    '"' => {
                        chars.next(); // consume '"'
                        result.push('\u{0022}'); // Quotation Mark (Double Quote)
                    }
                    'b' => {
                        chars.next(); // consume 'b'
                        result.push('\u{0008}'); // Backspace
                    }
                    'f' => {
                        chars.next(); // consume 'f'
                        result.push('\u{000C}'); // Form Feed
                    }
                    's' => {
                        chars.next(); // consume 's'
                        result.push('\u{0020}'); // Space
                    }
                    // Unicode escape sequence
                    'u' => {
                        chars.next(); // consume 'u'

                        // Expect opening brace
                        if chars.next() != Some('{') {
                            return Err(syn::Error::new(
                                span,
                                "Invalid Unicode escape: expected '{' after '\\u'",
                            ));
                        }

                        // Collect hex digits until closing brace
                        let mut hex_digits = String::new();
                        let mut found_closing_brace = false;

                        while let Some(hex_ch) = chars.next() {
                            if hex_ch == '}' {
                                found_closing_brace = true;
                                break;
                            }
                            if hex_ch.is_ascii_hexdigit() {
                                hex_digits.push(hex_ch);
                            } else {
                                return Err(syn::Error::new(
                                    span,
                                    format!(
                                        "Invalid character '{}' in Unicode escape sequence",
                                        hex_ch
                                    ),
                                ));
                            }
                        }

                        if !found_closing_brace {
                            return Err(syn::Error::new(
                                span,
                                "Unterminated Unicode escape sequence",
                            ));
                        }

                        if hex_digits.is_empty() {
                            return Err(syn::Error::new(span, "Empty Unicode escape sequence"));
                        }

                        if hex_digits.len() > 6 {
                            return Err(syn::Error::new(
                                span,
                                format!(
                                    "Unicode escape sequence too long: {} hex digits (max 6)",
                                    hex_digits.len()
                                ),
                            ));
                        }

                        // Parse hex number
                        match u32::from_str_radix(&hex_digits, 16) {
                            Ok(code_point) => match char::from_u32(code_point) {
                                Some(unicode_char) => result.push(unicode_char),
                                None => {
                                    return Err(syn::Error::new(
                                        span,
                                        format!("Invalid Unicode code point: U+{:X}", code_point),
                                    ))
                                }
                            },
                            Err(_) => {
                                return Err(syn::Error::new(
                                    span,
                                    format!("Invalid hex number in Unicode escape: {}", hex_digits),
                                ))
                            }
                        }
                    }
                    // Escaped whitespace (Section 3.11.1.1)
                    // When \ is followed by whitespace, both \ and all whitespace are discarded
                    c if c.is_whitespace() => {
                        // Consume all following whitespace characters
                        while let Some(&ws_ch) = chars.peek() {
                            if ws_ch.is_whitespace() {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        // Don't add anything to result - both \ and whitespace are discarded
                    }
                    // Invalid escape sequences (Section 3.11.1.2)
                    _ => {
                        return Err(syn::Error::new(
                            span,
                            format!("Invalid escape sequence: \\{}", next_ch),
                        ));
                    }
                }
            } else {
                // Backslash at end of string - invalid
                return Err(syn::Error::new(
                    span,
                    "Invalid escape sequence: backslash at end of string",
                ));
            }
        } else {
            result.push(ch);
        }
    }

    Ok(result)
}

/// Processes a multi-line string according to Section 3.12 of the KDL specification
/// This includes:
/// 1. Newline normalization (CR LF -> LF)
/// 2. Whitespace escape processing (before dedentation)
/// 3. Dedentation based on the final line's prefix
/// 4. Validation of multi-line string format
pub(crate) fn process_multiline_string(
    raw_content: &str,
    span: proc_macro2::Span,
) -> Result<String> {
    // Step 1: Validate basic format
    // The raw_content should start with a newline and end with whitespace followed by """
    let lines: Vec<&str> = raw_content.lines().collect();

    if lines.is_empty() {
        return Err(syn::Error::new(
            span,
            "Multi-line string cannot be empty - must start with a newline",
        ));
    }

    // Step 2: Normalize newlines (CR LF -> LF)
    // This normalization happens on the raw content before any processing
    let normalized_content = normalize_newlines(raw_content);

    // Handle the case where string ends with newline (lines() doesn't include trailing empty line)
    let mut normalized_lines: Vec<&str> = normalized_content.lines().collect();
    if normalized_content.ends_with('\n') {
        normalized_lines.push(""); // Add the implicit empty final line
    }

    if normalized_lines.len() < 2 {
        return Err(syn::Error::new(
            span,
            "Multi-line string must have at least 2 lines (opening newline and closing line)",
        ));
    }

    // The first line must be empty (only the opening newline)
    if !normalized_lines[0].is_empty() {
        return Err(syn::Error::new(
            span,
            "Multi-line string must immediately start with a newline after opening \"\"\"",
        ));
    }

    // Step 3: Process whitespace escapes on all lines except first and last
    let mut processed_lines = Vec::new();
    processed_lines.push(normalized_lines[0].to_string()); // Keep the empty first line

    for line in &normalized_lines[1..normalized_lines.len() - 1] {
        let processed_line = process_string_escapes(line, span)?;
        processed_lines.push(processed_line);
    }

    // Process whitespace escapes on the final line as well (per Section 3.12.3)
    let final_line_processed =
        process_string_escapes(normalized_lines[normalized_lines.len() - 1], span)?;
    processed_lines.push(final_line_processed);

    // Step 4: Determine dedentation prefix from the final line
    let final_line = &processed_lines[processed_lines.len() - 1];

    // Final line must contain only whitespace (after escape processing)
    if final_line.chars().any(|c| !c.is_whitespace()) {
        return Err(syn::Error::new(
            span,
            "Multi-line string final line must contain only whitespace before closing \"\"\"",
        ));
    }

    let dedent_prefix = final_line;

    // Step 5: Apply dedentation to intermediate lines
    let mut result_lines = Vec::new();

    for line_content in &processed_lines[1..processed_lines.len() - 1] {
        let dedented_line = if line_content.trim().is_empty() {
            // Empty lines (containing only whitespace) are preserved as empty
            String::new()
        } else {
            // Non-empty lines must start with at least the dedentation prefix
            if !line_content.starts_with(dedent_prefix) {
                return Err(syn::Error::new(
                    span,
                    format!("Multi-line string line does not start with required whitespace prefix. Expected prefix: {:?}, but line starts with: {:?}",
                            dedent_prefix,
                            line_content.chars().take(dedent_prefix.len()).collect::<String>())
                ));
            }

            // Remove the dedentation prefix
            line_content[dedent_prefix.len()..].to_string()
        };

        result_lines.push(dedented_line);
    }

    // Join the lines with LF characters
    Ok(result_lines.join("\n"))
}

/// Normalize newline sequences according to Section 3.12.1
/// Converts CR LF sequences to single LF, but preserves individual CR and LF
fn normalize_newlines(input: &str) -> String {
    // Replace CR LF sequences with single LF
    input.replace("\r\n", "\n")
}

/// Processes a raw string according to Section 3.13 of the KDL specification
/// Raw strings do NOT process any escape sequences - all content is preserved literally
/// This includes:
/// - No \n, \t, \\, \" etc. processing
/// - No Unicode escapes (\u{...})
/// - No whitespace escapes (\ + whitespace)
/// - Content is preserved exactly as-is
pub(crate) fn process_raw_string(
    value: &str,
    _hash_count: usize,
    _span: proc_macro2::Span,
) -> Result<String> {
    // Raw strings preserve all content literally - no escape processing at all
    // The hash_count parameter is stored in the AST but doesn't affect content processing
    // Validation against disallowed code points happens at the AST level
    Ok(value.to_string())
}

/// Processes a raw multi-line string according to Section 3.13 of the KDL specification
/// This is like process_multiline_string but WITHOUT any escape processing
/// This includes:
/// 1. Newline normalization (CR LF -> LF) - same as regular multi-line strings
/// 2. NO escape processing (unlike regular multi-line strings)
/// 3. Dedentation based on the final line's prefix - same as regular multi-line strings
/// 4. Validation of multi-line string format - same as regular multi-line strings
pub(crate) fn process_raw_multiline_string(
    raw_content: &str,
    _hash_count: usize,
    span: proc_macro2::Span,
) -> Result<String> {
    // Step 1: Validate basic format (same as regular multi-line strings)
    let lines: Vec<&str> = raw_content.lines().collect();

    if lines.is_empty() {
        return Err(syn::Error::new(
            span,
            "Raw multi-line string cannot be empty - must start with a newline",
        ));
    }

    // Step 2: Normalize newlines (CR LF -> LF) - same as regular multi-line strings
    let normalized_content = normalize_newlines(raw_content);

    // Handle the case where string ends with newline (lines() doesn't include trailing empty line)
    let mut normalized_lines: Vec<&str> = normalized_content.lines().collect();
    if normalized_content.ends_with('\n') {
        normalized_lines.push(""); // Add the implicit empty final line
    }

    if normalized_lines.len() < 2 {
        return Err(syn::Error::new(
            span,
            "Raw multi-line string must have at least 2 lines (opening newline and closing line)",
        ));
    }

    // The first line must be empty (only the opening newline)
    if !normalized_lines[0].is_empty() {
        return Err(syn::Error::new(
            span,
            "Raw multi-line string must immediately start with a newline after opening \"\"\"",
        ));
    }

    // Step 3: NO escape processing for raw strings (this is the key difference)
    // We preserve all content exactly as-is, including any \n, \t, \u{...} sequences
    let mut processed_lines = Vec::new();
    processed_lines.push(normalized_lines[0].to_string()); // Keep the empty first line

    // Copy all intermediate lines without any escape processing
    for line in &normalized_lines[1..normalized_lines.len() - 1] {
        processed_lines.push(line.to_string());
    }

    // Copy the final line without escape processing as well
    let final_line = normalized_lines[normalized_lines.len() - 1];
    processed_lines.push(final_line.to_string());

    // Step 4: Determine dedentation prefix from the final line (same as regular multi-line)
    let final_line = &processed_lines[processed_lines.len() - 1];

    // Final line must contain only whitespace (no escape processing means literal check)
    if final_line.chars().any(|c| !c.is_whitespace()) {
        return Err(syn::Error::new(
            span,
            "Raw multi-line string final line must contain only whitespace before closing \"\"\"",
        ));
    }

    let dedent_prefix = final_line;

    // Step 5: Apply dedentation to intermediate lines (same as regular multi-line)
    let mut result_lines = Vec::new();

    for line_content in &processed_lines[1..processed_lines.len() - 1] {
        let dedented_line = if line_content.trim().is_empty() {
            // Empty lines (containing only whitespace) are preserved as empty
            String::new()
        } else {
            // Non-empty lines must start with at least the dedentation prefix
            if !line_content.starts_with(dedent_prefix) {
                return Err(syn::Error::new(
                    span,
                    format!("Raw multi-line string line does not start with required whitespace prefix. Expected prefix: {:?}, but line starts with: {:?}",
                            dedent_prefix,
                            line_content.chars().take(dedent_prefix.len()).collect::<String>())
                ));
            }

            // Remove the dedentation prefix
            line_content[dedent_prefix.len()..].to_string()
        };

        result_lines.push(dedented_line);
    }

    // Join the lines with LF characters
    Ok(result_lines.join("\n"))
}
