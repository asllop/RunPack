# RunPack Tutorial

To follow this tutorial, some programming skills are assumed. At least a basic level of Rust, and essential data structures like stacks, and hash maps.

This tutorial is designed to be read sequentially, but feel free to skip the parts you already now.

## 0. Setup

All RunPack scripts in this tutorial are supposed to be executed in a program like the following:

```rust
use runpack::{Pack, Cell, self};

fn main() {
    println!("RunPack Tutorial\n");

    // YOUR CODE GOES HERE
    let script = r#"
        'Hello, World!' print
    "#;

    let mut pack = Pack::new_with_prelude(script);
    pack.dictionary.native("print", print);
    pack.dictionary.native("print_stack", print_stack);
    pack.run().expect("Failed running the script");
}

fn print(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let Some(cell) = pack.stack.pop() {
        match cell {
            Cell::Empty => println!("<EMPTY>"),
            Cell::Integer(i) => println!("{}", i),
            Cell::Float(f) => println!("{}", f),
            Cell::Boolean(b) => println!("{}", b),
            Cell::String(s) => println!("{}", s),
            Cell::Word(w) => println!("{}", w),
            Cell::Block(b) => println!("{:?}", b),
            Cell::Object(o) => println!("{:?}", o),
        }
        Ok(true)
    }
    else {
        Err(runpack::Error::new("print: couldn't get a cell from the stack".into(), 1000))
    }
}

fn print_stack(pack: &mut Pack) -> Result<bool, runpack::Error>  {
    println!("{:?}", pack.stack);
    Ok(true)
}
```

## 1. Ye Olde Stack

TODO