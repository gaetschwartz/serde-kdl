### 3.9. String

Strings in KDL represent textual UTF-8 Values (Section 3.7). A String is either an
Identifier String (Section 3.10) (like `foo`), a
Quoted String (Section 3.11) (like `"foo"`)
or a Multi-Line String (Section 3.12).
Both Quoted and Multiline strings come in normal
and Raw String (Section 3.13) variants (like `#"foo"#`):

- Identifier Strings let you write short, "single-word" strings with a
  minimum of syntax

- Quoted Strings let you write strings "like normal", with whitespace and escapes.

- Multi-Line Strings let you write strings across multiple lines
  and with indentation that's not part of the string value.

- Raw Strings don't allow any escapes,
  allowing you to not worry about the string's content containing anything that
  might look like an escape.

Strings *MUST* be represented as UTF-8 values.

Strings *MUST NOT* include the code points for
disallowed literal code points (Section 3.19) directly.
Quoted and Multi-Line Strings may include these code points as *values*
by representing them with their corresponding `\u{...}` escape.