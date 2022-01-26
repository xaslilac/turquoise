<h1 align="center" style="color: #67D1D1"><img alt="Turquoise" src="media/turquoise.svg" height="30" /></h1>

A long long time ago I tried to make a programming language called Teal. It was basically
just a lot of `input.replace(/some_regular_expression/, 'something_else')` type things
that turned "Teal" into JavaScript.

Turquoise is kind of just a different evolution of that. It's a "scripting" language (as
in it aims to be easy to write) that can compile to JavaScript or Rust (and by extension
a native executable). It's a little more rigourous that just hap-hazard text transforms,
and actually does the whole "lexing" and "AST" thing that's so popular these days.

```rust
// main.b
fn main() {
	println("Hello sailor!");
}
```

```sh
take run
```
