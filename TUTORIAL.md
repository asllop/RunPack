# RunPack Tutorial

To follow this tutorial, some programming skills are assumed. At least a basic level of Rust, and essential data structures like stacks, and hash maps.

This tutorial is designed to be read sequentially, but feel free to skip the parts you already now.

And one final note: it may take time to get used to a stack-based language, with its idiosyncratic [reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation). But once you do, you will absolutely love it.

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

The stack is the most important data structure in RunPack. It's used as an intermediate store to pass data from one word to another. In other language's terminology, we would say that it's used to pass arguments to functions, and to get returned values from them.

To push something to the stack we simply execute:

```
10
```

Now the stack contains one element, an integer with value 10. We can check that running the word `print_stack`, that will show the contents of the stack:

```
10 print_stack
```

The output will be:

```
Stack { stack: [Integer(10)], base: 0, nested: [] }
```

We learned a new thing, to execute a word (the equivalent of a function in other languages), we just need to name it.

Let's try to push more data of different types:

```
10 -5.5 true 'Hello' print_stack
```

Output:

```
Stack { stack: [Integer(10), Float(-5.5), Boolean(true), String("Hello")], base: 0, nested: [] }
```

All good. Now we want to pop data out from the stack, how can we do that?

```
10 20 + print_stack
```

Output:

```
Stack { stack: [Integer(30)], base: 0, nested: [] }
```

Interesting. We put two integers in the stack, then we called the word `+` and now the stack contains one integer with value 30. What happened here is that, the word `+` popped two integers from the stack, performed an addition, and finally pushed the resulting integer into the stack. And this is how RunPack works, the way we execute subroutines and pass arguments to them.

There are other basic operations we can perform on the stack. We can remove one data cell calling `drop`:

```
'hello' drop print_stack
```

Output:

```
Stack { stack: [], base: 0, nested: [] }
```

Duplicate it with `dup`:

```
123 dup print_stack
```

Output:

```
Stack { stack: [Integer(123), Integer(123)], base: 0, nested: [] }
```

Or `swap` positions:

```
'A' 'B' swap print_stack
```

Output:

```
Stack { stack: [String("B"), String("A")], base: 0, nested: [] }
```

### Stack transfers

Sometimes it can be hard to work with the stack. Our word needs the arguments in the stack in a certain way, but the current stack state left by the previous words is far from what we want. For these cases we have the stack transfer operator.

Let's imagine our current stack state is as follows:

```
0.5 'A string' 100.0 false
```

And we have to multiply the two floats, so we need them to be consecutive in the stack, right at the top of it. That's the perfect fit for the stack transfer operator:

```
0.5 'A string' 100.0 false
[ a, flt_a, b, flt_b : b, a, flt_b, flt_a ] * print
print_stack
```

Output:

```
50
Stack { stack: [Boolean(false), String("A string")], base: 0, nested: [] }
```

First of all, note that in RunPack, the comma is just a word separator, like the space. It has no other meaning and it's used to improve readability.

The stack transfer has the following format:

```
[ pop_1 pop_2 ... pop_N : push_1 push_2 ... push_N ]
```

The variables at the left of `:` are popped from the stack in the order they appear. The variables at the right of `:` are pushed into the stack in the order they appear. In the example of the two floats, we popped the following variables: `a` getting the value `false`, `flt_a` getting `100.0`, `b` getting `'A string'`, and `flt_b` getting `0.5`. Then we pushed them in the order we see: `b`(`'A string'`), `a`(`false`), `flt_b`(`0.5`) and `flt_a`(`100.0`). As a result, the `*` word will find the two floats in the stack to multiply them, and the other two data cells will remain untouched.

The variable in the left side can't be repeaded, but they can appear multiple times in the right side, or don't appear at all. For example, if we have 3 cells and want to remove the one in the middle, we could do:

```
1 2 3 [ a b c : c a ] print_stack
```

Output:

```
Stack { stack: [Integer(1), Integer(3)], base: 0, nested: [] }
```

Or maybe we want to triple a cell:

```
100 [ a : a a a ] print_stack
```

Output:

```
Stack { stack: [Integer(1), Integer(1), Integer(1)], base: 0, nested: [] }
```

### Nested stacks

TODO

## 2. Math and Logic

TODO