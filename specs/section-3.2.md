### 3.2. Node

Being a node-oriented language means that the real core component of any KDL
document is the "node". Every node must have a name, which must be a
String (Section 3.9).

The name may be preceded by a Type Annotation (Section 3.8) to further
clarify its type, particularly in relation to its parent node. (For example,
clarifying that a particular `date` child node is for the *publication* date,
rather than the last-modified date, with `(published)date`.)

Following the name are zero or more Arguments (Section 3.5) or
Properties (Section 3.4), separated by either whitespace (Section 3.17) or a
slash-escaped line continuation (Section 3.3). Arguments and Properties
may be interspersed in any order, much like is common with positional arguments
vs options in command line tools. Collectively, Arguments and Properties may be
referred to as "Entries".

Children (Section 3.6) can be placed after the name and the optional
Entries, possibly separated by either whitespace or a
slash-escaped line continuation.

Arguments are ordered relative to each other and that order must be preserved in
order to maintain the semantics. Properties between Arguments do not affect
Argument ordering.

By contrast, Properties *SHOULD NOT* be assumed to be presented in a given
order. Children (Section 3.6) should be used if an order-sensitive
key/value data structure must be represented in KDL. Cf. JSON objects
preserving key order.

Nodes *MAY* be prefixed with Slashdash (Section 3.17.3) to "comment out"
the entire node, including its properties, arguments, and children, and make
it act as plain whitespace, even if it spreads across multiple lines.

Finally, a node is terminated by either a Newline (Section 3.18), a semicolon
(`;`), the end of a child block (`}`) or the end of the file/stream (an `EOF`).

#### 3.2.1. Example

```kdl
// `foo` will have an Argument value list like `[1, 3]`.
foo 1 key=val 3 {
    bar
    (role)baz 1 2
}
```