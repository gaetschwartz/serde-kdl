### 3.10. Identifier String

An Identifier String (sometimes referred to as just an "identifier") is
composed of any [Unicode Scalar
Value](https://unicode.org/glossary/#unicode_scalar_value) other than
non-initial characters (Section 3.10.1), followed by any number of
Unicode Scalar Values other than non-identifier
characters (Section 3.10.2).

A handful of patterns are disallowed, to avoid confusion with other values:

* idents that appear to start with a Number (Section 3.14) (like `1.0v2` or
  `-1em`) or the "almost a number" pattern of a decimal point without a
  leading digit (like `.1`).

* idents that are the language keywords (`inf`, `-inf`, `nan`, `true`,
`false`, and `null`) without their leading `#`.

Identifiers that match these patterns *MUST* be treated as a syntax error; such
values can only be written as quoted or raw strings. The precise details of the
identifier syntax is specified in the Full Grammar in Section 4.

#### 3.10.1. Non-initial characters

The following characters cannot be the first character in an
Identifier String (Section 3.10):

* Any decimal digit (0-9)

* Any non-identifier characters (Section 3.10.2)

Additionally, the following initial characters impose limitations on subsequent
characters:

* the `+` and `-` characters can only be used as an initial character if
the second character is *not* a digit. If the second character is `.`, then
the third character must *not* be a digit.

* the `.` character can only be used as an initial character if
the second character is *not* a digit.

This allows identifiers to look like `--this` or `.md`, and removes the
ambiguity of having an identifier look like a number.

#### 3.10.2. Non-identifier characters

The following characters cannot be used anywhere in a Identifier String (Section 3.10):

* Any of `(){}[]/\"#;=`

* Any Whitespace (Section 3.17) or Newline (Section 3.18).

* Any disallowed literal code points (Section 3.19) in KDL
documents.