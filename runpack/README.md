# RunPack core

This crate contains the RunPack interpreter along with the *primitives*, the most essential words that make the language work.

## Installation

Add the following line to your Cargo dependencies:

```
runpack = { git = "https://github.com/asllop/RunPack" }
```

## Usage

The most basic usage:

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

## Vocabulary

The list of words defined by this create:

```
!=     ?         float        not       }
%      @         fract        or
(      @@        if           prod
)      @def      int          setter
*      [         is_block?    size
+      and       is_block?    skip
++     block     is_bool?     string
-      break     is_float?    sub
--     def       is_int?      sum
/      div       is_obj?      swap
<      drop      is_str?      type
<=     dup       is_word?     var
=      either    lex          wipe
>      exe       lex#         word
>=     exist?    loop         {
```

Checkout the documentation of each word for usage details.

## Learn

Read the [tutorial](./TUTORIAL.md).