### 3.11. Quoted String

A Quoted String is delimited by `"` on either side of any number of literal
string characters except unescaped `"` and `\`.

Literal Newline (Section 3.18) characters can only be included
if they are Escaped Whitespace (Section 3.11.1.1),
which discards them from the string value.
Actually including a newline in the value requires using a newline escape sequence,
like `\n`,
or using a Multi-Line String (Section 3.12)
which is actually designed for strings stretching across multiple lines.

Like Identifier Strings, Quoted Strings *MUST NOT* include any of the
disallowed literal code-points (Section 3.19) as code
points in their body.

Quoted Strings have a Raw String (Section 3.13) variant,
which disallows escapes.

#### 3.11.1. Escapes

In addition to literal code points, a number of "escapes" are supported in Quoted Strings.
"Escapes" are the character `\` followed by another character, and are
interpreted as described in the following table:

| Name | Escape | Code Pt |
|------|--------|----------|
| Line Feed | `\n` | `U+000A` |
| Carriage Return | `\r` | `U+000D` |
| Character Tabulation (Tab) | `\t` | `U+0009` |
| Reverse Solidus (Backslash) | `\\` | `U+005C` |
| Quotation Mark (Double Quote) | `\"` | `U+0022` |
| Backspace | `\b` | `U+0008` |
| Form Feed | `\f` | `U+000C` |
| Space | `\s` | `U+0020` |
| Unicode Escape | `\u{(1-6 hex chars)}` | Code point described by hex characters, as long as it represents a [Unicode Scalar Value](https://unicode.org/glossary/#unicode_scalar_value) |
| Whitespace Escape | See below | N/A |

##### 3.11.1.1. Escaped Whitespace

In addition to escaping individual characters, `\` can also escape whitespace.
When a `\` is followed by one or more literal whitespace characters, the `\`
and all of that whitespace are discarded. For example,

```kdl
"Hello World"
```

and

```kdl
"Hello \    World"
```

are semantically identical. See whitespace (Section 3.17)
and newlines (Section 3.18) for how whitespace is defined.

Note that only literal whitespace is escaped; whitespace escapes (`\n` and
such) are retained. For example, these strings are all semantically identical:

```kdl
"Hello\       \nWorld"

    "Hello\n\
    World"

"Hello\nWorld"

"""
  Hello
  World
  """
```

##### 3.11.1.2. Invalid escapes

Except as described in the escapes table, above, `\` *MUST NOT* precede any
other characters in a string.