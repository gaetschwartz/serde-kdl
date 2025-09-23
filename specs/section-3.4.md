### 3.4. Property

A Property is a key/value pair attached to a Node (Section 3.2). A Property is
composed of a String (Section 3.9), followed immediately by an equals sign (`=`, `U+003D`),
and then a Value (Section 3.7).

Properties should be interpreted left-to-right, with rightmost properties with
identical names overriding earlier properties. That is:

```kdl
node a=1 a=2
```

In this example, the node's `a` value must be `2`, not `1`.

No other guarantees about order should be expected by implementers.
Deserialized representations may iterate over properties in any order and
still be spec-compliant.

Properties *MAY* be prefixed with `/-` to "comment out" the entire token and
make it act as plain whitespace, even if it spreads across multiple lines.