## flint

<img
  align='right'
  src='assets/logo.png'
/>

**flint** is a tool that lets you write custom, lightweight static checkers
using the tree-sitter query language.

Checkers are specified in [YAML](https://en.wikipedia.org/wiki/YAML) files that
follow a certain structure, that is:

```yaml
name: rlint
language: rust
rules:
  cast_abs_to_unsigned:
    severity: warn
    category: suspicious
    message: Shouldn't cast `abs()` to unsigned
    query: |
      (let_declaration
        (type_cast_expression
          (call_expression) @call
          (primitive_type) @type
          (#match? @call "abs")
          (#match? @type "u8|u16|u32|u64")
        )
      ) @raise
    captures:
      - raise
  ...
```
