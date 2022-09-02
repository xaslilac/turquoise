<h1 align="center">
<img alt="Turquoise" src="https://cdn.mckayla.cloud/-/e95ffa0bde554ba19b0f7c2c59dbc073/turquoise.svg" height="100" />
</h1>

A long long time ago I tried to make a programming language called Teal. It was basically
just a lot of `input.replace(/some_regular_expression/, 'something_else')` type things
that turned "Teal" into JavaScript.

Turquoise is kind of just a different evolution of that. It's a "scripting" language (as
in it aims to be easy to write) that can compile to JavaScript or Rust (and by extension
a native executable). It's a little more rigourous that just hap-hazard text transforms,
and actually does the whole "lexing" and "AST" thing that's so popular these days.

```rust
// main.q
fn main() {
  println("Hello sailor!");
}
```

```sh
take run
```
