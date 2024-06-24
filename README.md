## flint

<img align='right' src='assets/logo.png' />

**flint** is a tool that lets you write custom, lightweight static checkers
using the tree-sitter query language.

Checkers are specified in [YAML](https://en.wikipedia.org/wiki/YAML) files that
follow a certain structure, that is:

```yaml
name: Rust

language: rust

rules:
  env_string_literals:
    severity: warn
    category: style
    message: Calls to (std::|)env::(var|remove_var) should not use string literals
    query: |
      ((call_expression
        function: (_) @function
        arguments: (arguments (string_literal))) @raise
      (#match? @function "(std::|)env::(var|remove_var)"))
    captures:
      - raise
```

See [examples/rust.yaml]() for a more fully expanded example.

### Installation

```
git clone https://github.com/terror/flint.git
cd flint
cargo install --path .
```
