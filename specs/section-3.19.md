### 3.19. Disallowed Literal Code Points

The following code points may not appear literally anywhere in the document.
They may be represented in Strings (but not Raw Strings) using Unicode Escapes (Section 3.11.1) (`\u{...}`,
except for non Unicode Scalar Value, which can't be represented even as escapes).

- The codepoints `U+0000-0008` or the codepoints `U+000E-001F` (various
  control characters).

- `U+007F` (the Delete control character).

- Any codepoint that is not a [Unicode Scalar
  Value](https://unicode.org/glossary/#unicode_scalar_value) (`U+D800-DFFF`).

- `U+200E-200F`, `U+202A-202E`, and `U+2066-2069`, the [unicode
  "direction control"
  characters](https://www.w3.org/International/questions/qa-bidi-unicode-controls)

- `U+FEFF`, aka Zero-width Non-breaking Space (ZWNBSP)/Byte Order Mark (BOM),
  except as the first code point in a document.