# RunPack

_Minimalistic, yet practical, scripting language_

## Introduction

RunPack is a small and modular scripting language written in ~1000 lines of Rust code, and designed to be embedded.

It is a [concatenative](https://en.wikipedia.org/wiki/Concatenative_programming_language), [stack-based](https://en.wikipedia.org/wiki/Stack-oriented_programming), [homoiconic](https://en.wikipedia.org/wiki/Homoiconicity), [async/await oriented](https://en.wikipedia.org/wiki/Async/await) programming language, strongly inspired by [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language)), and to a lesser extent by [Factor](https://en.wikipedia.org/wiki/Factor_(programming_language)), [Racket](https://en.wikipedia.org/wiki/Racket_(programming_language)), and [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)).

Language and implementation are designed to be extensible, providing a lightweight and powerful Rust API that allows to control any aspect of the language. Many features that in other languages are a core part of the compiler, like if/else/while statements, anonymous functions or variables, in RunPack are just words defined using the public API, or even RunPack itself.

RunPack is what you are looking for, if you...

1. Need a scripting language for your Rust application.
2. Want it to be small, simple and modular.
3. Require something fully customizable and hackable.
4. Appreciate concatenative programming languages.

## Installation

Add the following line to your Cargo dependencies:

```
runpack = { git = "https://github.com/asllop/RunPack" }
```

## Usage

To run a simple script that adds two numbers:

```rust
use runpack::{Pack, Cell};

let script = r#"
    "Add two integers, leaving the result in the stack"
    10 20 +
"#;

// Create the pack
let mut pack = Pack::new();
// Append code
pack.code(script);
// Run
pack.run().expect("Error running the script");
// Check results in the stack
if let Some(Cell::Integer(i)) = pack.stack.pop() {
    println!("Result = {}", i);
}
```

It's easy to define a new word in Rust and then call it from a script in RunPack:

```rust
use runpack::{self, Pack, Cell};

let script = r#"
    "Put a string in the stack and then execute the 'hi' word"
    'Andreu' hi
"#;

let mut pack = Pack::new();
// Define a word "hi" in Rust
pack.dictionary.native("hi", hi_word);
pack.code(script);
pack.run().expect("Error running the script");

fn hi_word(pack: &mut Pack) -> Result<bool, runpack::Error> {
    // Get a string from the stack and print it
    if let Some(Cell::String(name)) = pack.stack.pop() {
        println!("Hi {}!", name);
        Ok(true)
    }
    else {
        Err(runpack::Error::new("Couldn't get a string".into()))
    }
}
```

And it's also easy to define a word in RunPack and call it from Rust:

```rust
use runpack::{self, Pack, Cell};

let script = r#"
    "Define the word 'pi' that puts the number π in the stack"
    3.14159 def pi
"#;

let mut pack = Pack::new();
pack.code(script);
pack.run().expect("Error running the script");
// Execute word "pi"
pack.exec("pi").expect("Failed executing 'pi'");
// Check the stack for results
if let Some(Cell::Float(f)) = pack.stack.pop() {
    println!("The number π is {}", f);
}
```

## Learn RunPack

Learning is easy, you only need a couple of hours of your time and this introductory [tutorial](TUTORIAL.md). Additionally, we offer the [RunPack REPL](https://github.com/asllop/RunPack-REPL), a cli tool to facilitate the development of RunPack programs.

Enjoy the trip!

## Documentation

There are two sources of documentation, one is the standard Rust autodoc, generated with `cargo doc` as usual. The other is the vocabulary of RunPack words offered by a module, that comes in a [DOC.md](DOC.md) file.
