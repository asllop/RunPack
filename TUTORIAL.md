# RunPack Tutorial

To follow this tutorial, some programming skills are assumed. At least a basic level of Rust, and understanding of the essential data structures like stacks, and hash maps.

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
    println!("Stack:");
    for n in (0..pack.stack.size()).rev() {
        println!("\t{} : {:?}", n, pack.stack.get(n).unwrap());
    }
    Ok(true)
}
```

## 1. Ye Olde Stack

The stack is the most important data structure in RunPack. It's used as an intermediate store to pass data from one word to another. In other language's terminology, we would say that it's used to pass arguments to functions, and to get returned values from them.

To push something into the stack we simply execute:

```
10
```

Now the stack contains one element, an integer with value 10. We can check that running the word `print_stack`, that will show the contents of the stack:

```
10 print_stack
```

The output will be:

```
Stack:
	0 : Integer(10)
```

We learned a new thing, to execute a word (the equivalent of a function in other languages), we just need to name it.

Let's try to push more data of different types:

```
10 -5.5 true 'Hello' print_stack
```

Output:

```
Stack:
	3 : Integer(10)
	2 : Float(-5.5)
	1 : Boolean(true)
	0 : String("Hello")
```

All good. Now we want to pop data out from the stack, how can we do that?

```
10 20 + print_stack
```

Output:

```
Stack:
	0 : Integer(30)
```

Interesting. We put two integers in the stack, then we called the word `+` and now the stack contains one integer with value 30. What happened here is that, the word `+` popped two integers from the stack, performed an addition, and finally pushed the resulting integer into the stack. And this is how RunPack works, the way we execute subroutines and pass arguments to them.

There are other basic operations we can perform on the stack. We can remove one data cell calling `drop`:

```
'hello' drop print_stack
```

Output:

```
Stack:
```

Duplicate it with `dup`:

```
123 dup print_stack
```

Output:

```
Stack:
	1 : Integer(123)
	0 : Integer(123)
```

Or `swap` positions:

```
'A' 'B' print_stack swap print_stack
```

Output:

```
Stack:
	1 : String("A")
	0 : String("B")
Stack:
	1 : String("B")
	0 : String("A")
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
Stack:
	1 : String("A string")
	0 : Boolean(false)
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
Stack:
	1 : Integer(1)
	0 : Integer(3)
```

Or maybe we want to triple a cell:

```
100 [ a : a a a ] print_stack
```

Output:

```
Stack:
	2 : Integer(100)
	1 : Integer(100)
	0 : Integer(100)
```

### Nested stacks

The way we have used the stack until now is linear, we push and pop data into the stack. But the RunPack stack is more powerful than that, and it's actually a stack of stacks.

We can create a new stack, nested into the current stack, and this one will become our new current stack. We do that with the words `(` and `)`:

```
10 ( 20 print_stack ) print_stack
```

Output:

```
Stack:
	0 : Integer(20)
Stack:
	1 : Integer(10)
	0 : Integer(20)
```

Explanation: The word `(` opens a new stack nested inside the current one. From now on, this new stack will be our current. When we push `20` we are pushing into the nested stack, and thus, this cell doesn't live in the same stack as the `10` we pushed before, because it's located in the previous stack. That's why in the first `print_stack` we only see the `20`. Then we run the word `)`, that closes a nested stack. When this happens, all data in the current stack goes to the parent stack, in our case, the `20`. that's why the second `print_stack` shoes both, the `10` and the `20`.

Nested stacks are useful for operations that use all data from the stack, because it allows us to demarcate the limits of these operations. For example, the word `flush`, that removes all cells from the stack. We will see more usage examples in the following chapters.

## 2. Math and Logic

TODO