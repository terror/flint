## flint

<img
  align='right'
  src='assets/logo.png'
/>

**flint** is a tool that lets you write custom, lightweight static checkers
using the tree-sitter query language.

Checkers are specified in [YAML](https://en.wikipedia.org/wiki/YAML) files that
follow a certain structure, for instance:

```yaml
name: <name>
language: <language>
rules:
  <rule-name>:
    message: <message>
    query: <query>
    captures: <captures>
  ...
```
