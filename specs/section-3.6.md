### 3.6. Children Block

A children block is a block of Nodes (Section 3.2), surrounded by `{` and `}`. They
are an optional part of nodes, and create a hierarchy of KDL nodes.

Regular node termination rules apply, which means multiple nodes can be
included in a single-line children block, as long as they're all terminated by
`;`.

#### 3.6.1. Example

```kdl
parent {
    child1
    child2
}

parent { child1; child2; }
```