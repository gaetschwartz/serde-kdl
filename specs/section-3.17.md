### 3.17. Whitespace

The following characters should be treated as non-Newline (Section 3.18) [white space](https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt):

**Table 2**

| Name | Code Pt |
|------|----------|
| Character Tabulation | `U+0009` |
| Space | `U+0020` |
| No-Break Space | `U+00A0` |
| Ogham Space Mark | `U+1680` |
| En Quad | `U+2000` |
| Em Quad | `U+2001` |
| En Space | `U+2002` |
| Em Space | `U+2003` |
| Three-Per-Em Space | `U+2004` |
| Four-Per-Em Space | `U+2005` |
| Six-Per-Em Space | `U+2006` |
| Figure Space | `U+2007` |
| Punctuation Space | `U+2008` |
| Thin Space | `U+2009` |
| Hair Space | `U+200A` |
| Narrow No-Break Space | `U+202F` |
| Medium Mathematical Space | `U+205F` |
| Ideographic Space | `U+3000` |

#### 3.17.1. Single-line comments

Any text after `//`, until the next literal Newline (Section 3.18) is "commented out", and is considered to be Whitespace (Section 3.17).

#### 3.17.2. Multi-line comments

In addition to single-line comments using `//`, comments can also be started with `/*` and ended with `*/`. These comments can span multiple lines. They are allowed in all positions where Whitespace (Section 3.17) is allowed and can be nested.

#### 3.17.3. Slashdash comments

Finally, a special kind of comment called a "slashdash", denoted by `/-`, can be used to comment out entire *components* of a KDL document logically, and have those elements not be included as part of the parsed document data.

Slashdash comments can be used before the following, including before their type annotations, if present:

- A Node (Section 3.2): the entire Node is treated as Whitespace, including all props, args, and children.
- An Argument (Section 3.5): the Argument value is treated as Whitespace.
- A Property (Section 3.4) key: the entire property, including both key and value, is treated as Whitespace. A slashdash of just the property value is not allowed.
- A Children Block (Section 3.6): the entire block, including all children within, is treated as Whitespace. Only other children blocks, whether slashdashed or not, may follow a slashdashed children block.

A slashdash may be be followed by any amount of whitespace, including newlines and comments (other than other slashdashes), before the element that it comments out.