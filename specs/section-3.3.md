### 3.3. Line Continuation

Line continuations allow Nodes (Section 3.2) to be spread across multiple lines.

A line continuation is a `\` character followed by zero or more whitespace
items (including multiline comments) and an optional single-line comment. It
must be terminated by a Newline (Section 3.18) (including the Newline that is
part of single-line comments).

Following a line continuation, processing of a Node can continue as usual.

#### 3.3.1. Example

```kdl
my-node 1 2 \  // comments are ok after \
        3 4    // This is the actual end of the Node.
```