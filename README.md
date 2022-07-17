# RunPack

_Extremely simple, yet practical, scripting language_

## Introduction

RunPack is a small and modular scripting language written in ~800 lines of Rust code (the core), designed to be embedded into other applications.

It is a [stack-based](https://en.wikipedia.org/wiki/Stack-oriented_programming), [concatenative](https://en.wikipedia.org/wiki/Concatenative_programming_language), [homoiconic](https://en.wikipedia.org/wiki/Homoiconicity) programming language, strongly inspired by [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language)), and to a lesser extent by [Lua](https://en.wikipedia.org/wiki/Lua_(programming_language)), [Factor](https://en.wikipedia.org/wiki/Factor_(programming_language)) and [Racket](https://en.wikipedia.org/wiki/Racket_(programming_language)).

Language and implementation are designed to be extensible. RunPack provides a lightweight and powerful Rust API that allows to control any aspect of the language by defining custom "words" (functions). Actually, things that in other languages are a core part of the compiler, like if-else statements, anonymous functions or variables, in RunPack are just words defined using the public API.

## Embedding

To embed RunPack into your applications, start by including the dependencies in your cargo file:

```toml
runpack = "0.1.0"
```

Then use it:

```rust
use runpack::{Pack, Cell};

let script = r#"
    "Add two numbers and leave the result in the stack"
    10 20 +
"#;
let mut pack = Pack::new(script);
pack.run().expect("Error running the script");
if let Some(Cell::Integer(n)) = pack.stack.pop() {
    println!("Result is {}", n);
}
```

<!-- ## TODO: Coding -->
