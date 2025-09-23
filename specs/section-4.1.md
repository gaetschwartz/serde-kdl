### 4.1. Grammar language

The grammar language syntax is a combination of ABNF with some regex spice thrown in.
Specifically:

- Single quotes (`'`) are used to denote literal text. `\` within a literal
  string is used for escaping other single-quotes, for initiating unicode
  characters using hex values (`\u{FEFF}`), and for escaping `\` itself
  (`\\`).

- `*` is used for "zero or more", `+` is used for "one or more", and `?` is
  used for "zero or one". Per standard regex semantics, `*` and `+` are *greedy*;
  they match as many instances as possible without failing the match.

- `*?` (used only in raw strings) indicates a *non-greedy* match;
  it matches as *few* instances as possible without failing the match.

- `¶` is a *cut point*. It always matches and consumes no characters,
  but once matched, the parser is not allowed to backtrack past that point in the source.
  If a parser would rewind past the cut point, it must instead fail the overall parse,
  as if it had run out of options.
  (This is only used with the `raw-string` production,
  to ensure the first instance of the appropriate closing quote sequence
  is guaranteed to be the end of the raw string,
  rather than allowing it to potentially consume more of the document unexpectedly.)

- `()` can be used to group matches that must be matched together.

- `a | b` means `a or b`, whichever matches first. If multiple items are before
  a `|`, they are a single group. `a b c | d` is equivalent to `(a b c) | d`.

- `[]` are used for regex-style character matches, where any character between
  the brackets will be a single match. `\` is used to escape `\`, `[`, and
  `]`. They also support character ranges (`0-9`), and negation (`^`)

- `-` is used for "except for" or "minus" whatever follows it. For example,
  `a - 'x'` means "any `a`, except something that matches the literal `'x'`".

- The prefix `^` means "something that does not match" whatever follows it.
  For example, `^foo` means "must not match `foo`".

- A single definition may be split over multiple lines. Newlines are treated as
  spaces.

- `//` followed by text on its own line is used as comment syntax.