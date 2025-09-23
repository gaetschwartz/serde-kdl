### 3.1. Document

The toplevel concept of KDL is a Document. A Document is composed of zero or more Nodes (Section 3.2), separated by newlines and whitespace, and eventually terminated by an EOF.

All KDL documents should be UTF-8 encoded and conform to the specifications in this document.

#### 3.1.1. Example

The following is a document composed of two toplevel nodes:

```kdl
foo {
    bar
}
baz
```