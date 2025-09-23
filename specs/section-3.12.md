### 3.12. Multi-line String

Multi-Line Strings support multiple lines with literal, non-escaped
Newlines. They must use a special multi-line syntax, and they automatically
"dedent" the string, allowing its value to be indented to a visually matching
level as desired.

A Multi-Line String is opened and closed by *three* double-quote characters,
like `"""`.
Its first line *MUST* immediately start with a Newline (Section 3.18)
after its opening `"""`.
Its final line *MUST* contain only whitespace
before the closing `"""`.
All in-between lines that contain non-newline, non-whitespace characters
*MUST* start with *at least* the exact same whitespace as the final line
(precisely matching codepoints, not merely counting characters or "size");
they may contain additional whitespace following this prefix. The lines in
between may contain unescaped `"` (but no unescaped `"""` as this would close
the string).

The value of the Multi-Line String omits the first and last Newline, the
Whitespace of the last line, and the matching Whitespace prefix on all
intermediate lines. The first and last Newline can be the same character (that
is, empty multi-line strings are legal).

In other words, the final line specifies the whitespace prefix that will be
removed from all other lines.

Multi-line Strings that do not immediately start with a Newline and whose final
`"""` is not preceeded by optional whitespace and a Newline are illegal. This
also means that `"""` may not be used for a single-line String (e.g.
`"""foo"""`).

#### 3.12.1. Newline Normalization

Literal Newline sequences in Multi-line Strings must be normalized to a single
`U+000A` (`LF`) during deserialization. This means, for example, that `CR LF`
becomes a single `LF` during parsing.

This normalization does not apply to non-literal Newlines entered using escape
sequences. That is:

```kdl
multi-line """
    \r\n[CRLF]
    foo[CRLF]
    """
```

becomes:

```kdl
single-line "\r\n\nfoo"
```

For clarity: this normalization applies to each individual Newline sequence.
That is, the literal sequence `CRLF CRLF` becomes `LF LF`, not `LF`.

#### 3.12.2. Examples

##### 3.12.2.1. Indented multi-line string

```kdl
multi-line """
        foo
    This is the base indentation
            bar
    """
```

This example's string value will be:

```
    foo
This is the base indentation
        bar
```

which is equivalent to

```kdl
"    foo\nThis is the base indentation\n        bar"
```

when written as a single-line string.

##### 3.12.2.2. Shorter last-line indent

If the last line wasn't indented as far,
it won't dedent the rest of the lines as much:

```kdl
multi-line """
        foo
    This is no longer on the left edge
            bar
  """
```

This example's string value will be:

```
      foo
  This is no longer on the left edge
          bar
```

Equivalent to

```kdl
"      foo\n  This is no longer on the left edge\n          bar"
```

##### 3.12.2.3. Empty lines

Empty lines can contain any whitespace, or none at all, and will be reflected as empty in the value:

```kdl
multi-line """
    Indented a bit

    A second indented paragraph.
    """
```

This example's string value will be:

```
Indented a bit.

A second indented paragraph.
```

Equivalent to

```kdl
"Indented a bit.\n\nA second indented paragraph."
```

##### 3.12.2.4. Syntax errors

The following yield **syntax errors**:

```kdl
multi-line """can't be single line"""
```

```kdl
multi-line """
  closing quote with non-whitespace prefix"""
```

```kdl
multi-line """stuff
  """
```

```kdl
// Every line must share the exact same prefix as the closing line.
multi-line """[\n]
[tab]a[\n]
[space][space]b[\n]
[space][tab][\n]
[tab]"""
```

#### 3.12.3. Interaction with Whitespace Escapes

Multi-line strings support the same mechanism for escaping whitespace as Quoted
Strings.

When processing a Multi-line String, implementations MUST dedent the string
*after* resolving all whitespace escapes, but *before* resolving other backslash
escapes. This means a whitespace escape that attempts to escape the final line's
newline and/or whitespace prefix can be invalid: if removing escaped whitespace
places the closing `"""` on a line with non-whitespace characters, this escape
is invalid.

For example, the following example is illegal:

```kdl
  """
  foo
  bar\
  """

  // equivalent to
  """
  foo
  bar"""
```

while the following example is allowed

```kdl
  """
  foo \
bar
  baz
  \   """

  // equivalent to
  """
  foo bar
  baz
  """
```