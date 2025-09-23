### 3.18. Newline

The following character sequences [should be treated as new lines](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-5/#G41643):

**Table 3**

| Acronym | Name | Code Pt |
|---------|------|----------|
| CRLF | Carriage Return and Line Feed | `U+000D` + `U+000A` |
| CR | Carriage Return | `U+000D` |
| LF | Line Feed | `U+000A` |
| NEL | Next Line | `U+0085` |
| VT | Vertical tab | `U+000B` |
| FF | Form Feed | `U+000C` |
| LS | Line Separator | `U+2028` |
| PS | Paragraph Separator | `U+2029` |

Note that for the purpose of new lines, the specific sequence `CRLF` is considered *a single newline*.
