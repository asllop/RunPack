# RunPack

_Minimalistic, yet practical, scripting language_

## Introduction

RunPack is a small and modular scripting language written in ~800 lines of Rust code, and designed to be embedded.

It is a [concatenative](https://en.wikipedia.org/wiki/Concatenative_programming_language), [stack-based](https://en.wikipedia.org/wiki/Stack-oriented_programming), [homoiconic](https://en.wikipedia.org/wiki/Homoiconicity) programming language, strongly inspired by [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language)), and to a lesser extent by [Lua](https://en.wikipedia.org/wiki/Lua_(programming_language)), [Factor](https://en.wikipedia.org/wiki/Factor_(programming_language)) and [Racket](https://en.wikipedia.org/wiki/Racket_(programming_language)).

Language and implementation are designed to be extensible, providing a lightweight and powerful Rust API that allows to control any aspect of the language. Many features that in other languages are a core part of the compiler, like if/else/while statements, anonymous functions or variables, in RunPack are just words defined using the public API, or even RunPack itself.

RunPack is what you are looking for, if you...

1. Need a scripting language for your Rust application.
2. Want it to be small and simple.
3. Look for something fully customizable and hackable.
4. Appreciate concatenative programming languages.

## Embedding

To start using RunPack, first include it in your cargo file:

```toml
runpack = "0.1.0"
```

Also include as a dependency any other [RunPack module](#modules) you will need.

Then use it:

```rust
use runpack::{Pack, Cell};

let script = r#"
    "Add two integers, leaving the result in the stack"
    10 20 +
"#;

let mut pack = Pack::new();
pack.code(script);
pack.run().expect("Error running the script");

if let Some(Cell::Integer(i)) = pack.stack.pop() {
    println!("Result = {}", i);
}
```

It's easy to define new words in Rust and then call them from a script in RunPack:

```rust
use runpack::{self, Pack, Cell};

let script = r#"
    "Put a string in the stack and then execute the 'hi' word"
    'Andreu' hi
"#;

let mut pack = Pack::new();
pack.dictionary.native("hi", hi_word);
pack.code(script);
pack.run().expect("Error running the script");

fn hi_word(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let Some(Cell::String(name)) = pack.stack.pop() {
        println!("Hi {}!", name);
        Ok(true)
    }
    else {
        Err(runpack::Error::new("Couldn't get a string".into(), 1000))
    }
}
```

And it's also very easy to call a word defined in RunPack from Rust:

```rust
use runpack::{self, Pack, Cell};

let script = r#"
    "Define the word 'pi' that puts the number π in the stack"
    3.14159 def pi
"#;

let mut pack = Pack::new();
pack.code(script);
pack.run().expect("Error running the script");
pack.exec("pi").expect("Failed executing 'pi'");

if let Some(Cell::Float(f)) = pack.stack.pop() {
    println!("The number π is {}", f);
}
```

## Modules

The [RunPack core](runpack/) contains only the most common functionality, due to its modular nature, all domain-specific features are spread across different crates.

As of now, the available modules are:

- [runpack](runpack/): Contains the interpreter and the core words, the minimum necessary to make the language work.
- [runpack_obj](runpack_obj/): Vocabulary to operate with maps and vectors.
- [runpack_async](runpack_async/): Asyncronous infrastructure.

Additionally, we have the [RunPack REPL](https://github.com/asllop/RunPack-REPL), a cli tool to facilitate the development of RunPack programs.

## Learn RunPack

Learning is easy, you only need 1 hour of your time and this introductory [tutorial](runpack/TUTORIAL.md). Each module comes with its own documentation and tutorial, check-out the different crate folders in the present repository.

Enjoy the trip!
