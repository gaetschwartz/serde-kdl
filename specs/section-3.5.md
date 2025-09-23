### 3.5. Argument

An Argument is a bare Value (Section 3.7) attached to a Node (Section 3.2), with no
associated key. It shares the same space as Properties (Section 3.4), and may be interleaved with them.

A Node may have any number of Arguments, which should be evaluated left to
right. KDL implementations *MUST* preserve the order of Arguments relative to
each other (not counting Properties).

Arguments *MAY* be prefixed with `/-` to "comment out" the entire token and
make it act as plain whitespace, even if it spreads across multiple lines.

#### 3.5.1. Example

```kdl
my-node 1 2 3 a b c
```