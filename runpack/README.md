# RunPack core

This crate contains the RunPack interpreter along with the *primitives*, the most essential words that make the language work.

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

It's easy to define new words in Rust and then call them from a script in RunPack:

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
// Execute word "pi"
pack.exec("pi").expect("Failed executing 'pi'");
// Check the stack for results
if let Some(Cell::Float(f)) = pack.stack.pop() {
    println!("The number π is {}", f);
}
```

## Learn RunPack

Learning is easy, you only need 1 hour of your time and this introductory [tutorial](TUTORIAL.md). Each module comes with its own documentation and tutorial. Check-out the different crate folders.

Enjoy the trip!

## Documentation

Each module defines a vocabulary of words, that is the API of that module. Checkout the documentation of this module [here](./DOC.md).