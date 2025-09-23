### 3.13. Raw String

Both Quoted ([Section 3.11](#quoted-string)) and Multi-Line Strings ([Section 3.12](#multi-line-string)) have
Raw String variants, which are identical in syntax except they do not support
`\`-escapes. This includes line-continuation escapes (`\` + `ws` collapsing to
nothing). They otherwise share the same properties as far as literal
Newline ([Section 3.18](#newline)) characters go, multi-line rules, and the requirement of
UTF-8 representation.

The Raw String variants are indicated by preceding the strings's opening quotes
with one or more `#` characters. The string is then closed by its normal closing
quotes, followed by a *matching* number of `#` characters. This means that the
string may contain any combination of `"` and `#` characters other than its
closing delimiter (e.g., if a raw string starts with `##"`, it can contain `"`
or `"#`, but not `"##` or `"###`).

Like other Strings, Raw Strings *MUST NOT* include any of the disallowed
literal code-points ([Section 3.19](#disallowed-literal-code-points)) as code points in their
body. Unlike with Quoted Strings, these cannot simply be escaped, and are thus
unrepresentable when using Raw Strings.

#### 3.13.1. Example

```kdl
just-escapes #"\n will be literal"#
```

The string contains the literal characters `\n will be literal`.

```kdl
quotes-and-escapes ##"hello\n\r\asd"#world"##
```

The string contains the literal characters `hello\n\r\asd"#world`

```kdl
raw-multi-line #"""
    Here's a """
        multiline string
        """
    without escapes.
    """#
```

The string contains the value

```
Here's a """
    multiline string
    """
without escapes.
```

or equivalently,

```kdl
"Here's a \"\"\"\n    multiline string\n    \"\"\"\nwithout escapes."
```

as a Quoted String.