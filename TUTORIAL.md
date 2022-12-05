# RunPack Tutorial

In this tutorial we are going to learn the basics of RunPack, how the interpreter works and the core functionalities.

If you are not used to [reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) it can be shocking at first, take your time to understand and test the examples.

Some programming skills are assumed, at least a basic level of Rust, and understanding of the essential data structures like stacks, and hash maps.

## Index
  * [0. Setup](#0-setup)
  * [1. Ye Olde Stack](#1-ye-olde-stack)
  * [2. Arithmetic & Logic operations](#2-arithmetic--logic-operations)
  * [3. Words](#3-words)
  * [4. Control Flow](#4-control-flow)
    * [4.1. Conditional Execution](#41-conditional-execution)
    * [4.2. Loops](#42-loops)
  * [5. Lexicons](#5-lexicons)
  * [6. Word References](#6-word-references)
  * [7. Advanced Topics](#7-advanced-topics)
    * [7.1. Cells](#71-cells)
    * [7.2. The Concat(enation)](#72-the-concatenation)
    * [7.3. The Dictionary](#73-the-dictionary)
    * [7.4. The Return Stack](#74-the-return-stack)
    * [7.5. Custom Structs](#75-custom-structs)

## 0. Setup

All RunPack scripts in this tutorial have been executed using the REPL tool. To install it, do the following:

```
$ cargo install --git https://github.com/asllop/RunPack-REPL
$ runpack-cli
```

If your cargo bin folder is not in the path, put it or run:

```
$ ~/.cargo/bin/runpack-cli
```

Alternatively, you can create a Rust program and embed the scripts in there. Something like the following should work:

```rust
use runpack::{Pack, Cell, self};

fn main() {
    println!("RunPack Tutorial\n");

    // YOUR CODE GOES HERE
    let script = r#"
        'Hello, World!' print
    "#;

    // Create pack and register plugins
    let mut pack = Pack::new();
    pack.dictionary.native("print", print);
    pack.dictionary.native("show_stack", show_stack);

    // Add script code and run
    pack.code(script);
    pack.run().expect("Failed running the script");
}

fn print(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let Some(cell) = pack.stack.pop() {
        match cell {
            Cell::Integer(i) => println!("{}", i),
            Cell::Float(f) => println!("{}", f),
            Cell::Boolean(b) => println!("{}", b),
            Cell::String(s) => println!("{}", s),
            Cell::Word(w) => println!("{}", w),
            Cell::Block(b) => println!("{:?}", b),
            Cell::Struct(s) => println!("{:?}", s),
        }
        Ok(true)
    }
    else {
        Err(runpack::Error::new("print: couldn't get a cell from the stack".into()))
    }
}

fn show_stack(pack: &mut Pack) -> Result<bool, runpack::Error>  {
    println!("Stack:");
    for n in 0..pack.stack.size() {
        println!("\t{} : {:?}", n, pack.stack.get(n).unwrap());
    }
    Ok(true)
}
```

## 1. Ye Olde Stack

The stack is the most important data structure in RunPack. It's used as an intermediate store to pass data from one word to another. In other language's terminology, we would say that it's used to pass arguments to functions, and to get returned values from them.

To push something into the stack we simply do:

```
10
```

Now the stack contains one element, an integer with value 10. We can check that running the word `show_stack`, that will show the contents of the stack:

```
show_stack
```

The output will be:

```
Stack:
	0 : Integer(10)
```

We learned a new thing, to execute a word (the equivalent of a function in other languages), we just need to name it.

Let's try to push more data of different types:

```
-5.5 true 'Hello'
show_stack
```

Output:

```
Stack:
	0 : String("Hello")
	1 : Boolean(true)
	2 : Float(-5.5)
	3 : Integer(10)
```

All good. Now we want to pop data out from the stack. How can we do that?

First we are going to clean-up the stack:

```
wipe
```

And then:

```
10 20 +
show_stack
```

Output:

```
Stack:
	0 : Integer(30)
```

Interesting. We put two integers in the stack, then we called the word `+` and now the stack contains one integer with value 30. The word `+` popped two integers from the stack, performed an addition, and finally pushed the resulting integer into the stack. And this is how RunPack works, the way we execute subroutines and pass arguments to them.

There are other basic operations we can perform on the stack. We can remove one data cell calling `drop`:

```
drop
show_stack
```

Output:

```
Stack:
```

Duplicate it with `dup`:

```
123 dup
show_stack
```

Output:

```
Stack:
	1 : Integer(123)
	0 : Integer(123)
```

Or `swap` positions:

```
wipe
'A' 'B' show_stack
swap show_stack
```

Output:

```
Stack:
	0 : String("B")
	1 : String("A")
Stack:
	0 : String("A")
	1 : String("B")
```

### Stack Transfers

Sometimes it can be hard to work with the stack. Our word needs the arguments in the stack in a certain way, but the current stack state left by the previous words is far from what we want. For these cases we have the stack transfer operator.

Let's imagine our current stack state is as follows:

```
wipe

0.5 'A string' 100.0 false
```

And we have to multiply the two floats, so we need them to be consecutive in the stack, right at the top of it. That's the perfect fit for the stack transfer operator:

```
[ a, flt_a, b, flt_b | b, a, flt_b, flt_a ] * print
show_stack
```

Output:

```
50
Stack:
	0 : Boolean(false)
	1 : String("A string")
```

Look how we printed the multipliation of 0.5 and 100.0, and the rest of the srack remained the same.

**Beware of the spaces!** In RunPack, the space is the word delimiter. Writing "`[ a`" is a totally different thing than writing "`[a`". In the former case we have 2 words, `[` and `a`. In the latter, we have only one word, identified as `[a`.

Also, note that in RunPack, the comma is just a word separator, like the space. It has no other meaning, it's only used to improve readability, and is optional.

The stack transfer has the following format:

```
[ pop_1 pop_2 ... pop_N | push_1 push_2 ... push_N ]
```

The variables at the left of `|` are popped from the stack in the order they appear. The variables at the right of `|` are pushed into the stack in the order they appear. In the example of the two floats, we popped the following variables: `a` getting the value `false`, `flt_a` getting `100.0`, `b` getting `'A string'`, and `flt_b` getting `0.5`. Then we pushed them in the order we see: `b`(`'A string'`), `a`(`false`), `flt_b`(`0.5`) and `flt_a`(`100.0`). As a result, the `*` word will find the two floats in the stack to multiply them, and the other two data cells will remain untouched.

The variable in the left side can't be repeaded, but they can appear multiple times in the right side, or don't appear at all. For example, if we have 3 cells and want to remove the one in the middle, we could do:

```
wipe

1 2 3 [ a b c | c a ] show_stack
```

Output:

```
Stack:
	0 : Integer(3)
	1 : Integer(1)
```

Or maybe we want to triple a cell:

```
[ a | a a a ] show_stack
```

Output:

```
Stack:
	0 : Integer(3)
	1 : Integer(3)
	2 : Integer(3)
	3 : Integer(1)
```

### Nested Stacks

The way we have used the stack until now is linear, we push and pop data into the stack. But the RunPack stack is more powerful than that, and it's actually a stack of stacks.

We can create a new stack, nested into the current stack, and this one will become our new current stack. We do that with the words `(` and `)`:

```
wipe

10 ( 20 show_stack ) show_stack
```

Output:

```
Stack:
	0 : Integer(20)
Stack:
	0 : Integer(20)
	1 : Integer(10)
```

Explanation: The word `(` opens a new stack nested inside the current one. From now on, this new stack will be our current. When we push `20` we are pushing into the nested stack, and thus, this cell doesn't live in the same stack as the `10` we pushed before, because it's located in the previous stack. That's why in the first `show_stack` we only see the `20`. Then we run the word `)`, that closes a nested stack. When this happens, all data in the current stack goes to the parent stack, in our case, the `20`. that's why the second `show_stack` shows both, the `10` and the `20`.

Nested stacks are useful for operations that use all the data from the stack, because it allows us to demarcate the limits of these operations. For example, the word `wipe`, that removes all cells from the stack. We will see more usage examples in the following chapters.

## 2. Arithmetic & Logic operations

We have already seen some of them. Arithmetic operations can work either with integers or floats, but can't mix them. There are five: addition, subtraction, multiplication, division, and remainder of a division.

```
wipe

5 2 + print
5 2 - print
5 2 * print
5 2 / print
5 2 % print
```

Output:

```
7
3
10
2
1
```

If we need, we can convert between float and integer:

```
19 float 0.5 + print
5.5 int print
```

Output:

```
19.5
5
```

Sometimes an arithmetic operation that is big and complicated can look messy when implemented using these operators. Just look how we would implement `(1+2+3)*(4+5+6)*(7+8+9)`:

```
1 2 + 3 + 4 5 + 6 + 7 8 + 9 + * * print
```

Pretty ugly, uh?

For this kind of cases we have the sequence operations: `sum` and `prod`:

```
( ( 1 2 3 sum ) ( 4 5 6 sum ) ( 7 8 9 sum ) prod ) print
```

Pretty neat, uh?

This is the typical case where [nested stacks](#nested-stacks) are useful. The `sum` word gets all numbers in the stack and adds them. Same for `prod`, but with multiplication. Because we want to limit the data available for each operation, we use the words `(` and `)` to create a stack, where we put the data and finally call the operation we want.

Yet again, beware of the spaces!

Finally, the logic operators are pretty simple: `and`, `or`, and `not`. They operate on integers or booleans and the result is either an integer or a boolen:

```
-1 0 or print
-1 0 and print
-1 not print
true false or print
true false and print
true not print
```

Output:

```
-1
0
0
true
false
false
```

## 3. Words

Earlier in this tutorial we said that a word is akin to a function in other programming languages. That is partially true, because a word is more than a function. A word is also a variable, and a reference, and an object. A word is just a word and we will see now how to define and use them.

To define a word we use the word `def` followed by a name:

```
'Andreu' def name
555555 def phone
```

This syntax may look contradictory for a language that uses Reverse Polish Notation, it seems to violate the rules. The word `def` uses an argument that is in the stack, that's good, but then it uses another argument, the word name, that is not in the stack. For now let's leave it, we will understand what's going on once we get into the [Concat](#72-the-concatenation).

Now, what happens when we execute these words?

```
'Andreu' def name
555555 def phone

name
phone
show_stack
```

Output:

```
Stack:
	0 : Integer(555555)
	1 : String("Andreu")
```

The values we assigned to these words are now in the stack. These words act as a constant or a variable, executing `name` is equivalent to execute `'Andreu'`.

Before proceeding to the next step, we will talk about a new data type: the Block. To define a block we use the words `{` and `}` (I know I'm being a bit of a nagger, but please, beware of the spaces):

```
wipe

{ 'We are in a block' print }
show_stack
```

Output:

```
Stack:
	0 : Block(BlockRef { pos: 477, len: 3 })
```

_Note_: the exact pos value  may vary.

A block is a piece of code, but is also a data type and can be treated as data. It can be pushed into the stack, sent to other words, and returned by them. And of course, stored in a word definition:

```
def we_are

we_are
```

Output:

```
We are in a block
```

And here it is. When we store a block in a word, and then execute this word, we don't get the block in the stack, as it happened with the string and the integer in the previous example. In this case, RunPack executes the code in the block.

See another example, we will define a word to double a number:

```
{ 2 * } def 2*
120 2* print
```

Output:

```
240
```

There are other words used to define words, a very useful one is `var` used to define variables:

```
101 var my_num
```

Now we can just invoce `my_num` to get the value in the stack:

```
my_num print
```

Output:

```
101
```

But what makes it different from using `def`? The cool part of `var` is that it actually defines two words, a getter and a setter. We already saw the getter, is just `my_num`. The setter is used to change the value:

```
202 my_num!
```

Now we can print `my_num` again and see how the value changed:

```
my_num print
```

Output:

```
202
```

### Word Documentation

Because of the dynamic nature of RunPack and the use of the stack, there is no way to know the arguments a word takes and the results it produces without inspecting and understending the code. For this reason we have the stack effect comments, to describe in a fast and readable way how a word affects the stack. The format for this comments is: `a b -> x y`, where `a` and `b`, are the contents of the stack before executing the word, and that are used by it, and `x` and `y`, are the contents of the stack after executing the word.

```
? double2* 'a b -> x y' 'Double two numbers.'
{ 2* swap 2* swap } def double2*
```

_Note_: this word makes use of the `2*` we previously defined.

We used the word `?` to document how `double2*` works. It takes 3 arguments, the word name, the stack effects (a string) and the description (another string). Now we can use the word `help` to consult the documentation:

```
help double2*
```

Output:

```
Stack effect:	a b -> x y
Description:	Double two numbers.
```

The word `?` only does somthing in development mode (while in the REPL tool), when running a script in production mode, it will be ignored.

## 4. Control Flow

### 4.1 Conditional Execution

The `2*` word we implemented in the previous chapter will only work with integers, and that's a problem, because generic arithmetic operators should accept any number. How could we create a version of it that works with both types?

```
{ is_int? if 2 2.0 * } def 2*
5 2* print
5.2 2* print
```

Output:

```
10
10.4
```

We introduced multiple new concepts here. First, the `is_int?` word, checks if the type of the next cell in the stack is an integer, and puts a boolean with the result. Then we have the `if`. This words gets a boolean from the stack, if this boolean is true, the first word after the `if` is executed, and the second word is skipped. If the boolean is false, the first word is skipped and the second is executed.

If we don't care about one of the two cases, we can use the `_` word, that just does nothing:

```
{ 'Is greater' print } def greater
100 10 > if greater _
```

Here we introduced one more word, the `>`. This word is a comparator, it gets two cells, compares if the first is greater thant the second, and returns a boolean. There are 6 comparators: `=`, `!=`, `>`, `<`, `>=`, and `<=`.

### 4.2 Loops

There are two kinds of loops in RunPack: while-do and loop-again.

The first kind of loop is defined with the pair of words `while`/`do`. It has the format `while condition do action` where `condition` and `action` **must be words defined with a code block**. For example:

```
? continue? 'a -> a b' 'Check if "a" is greater than zero and put in the stack a boolean "b" accordingly.'
{ dup 0 > } def continue?

? decrement 'a -> b' 'Print number "a" and decrement it, leaving the resulting value "b" in the stack.'
{ dup print 1- } def decrement

? countdown 'a -> ' 'Print the countdown from "a" to 1.'
{ while continue? do decrement drop } def countdown

10 countdown
```

The condition word must return a boolean, that is used by `do` do decide whether to execute the action word or end the loop.

The second kind of loop is defined with the pair of words `loop`/`again`:

```
{ loop print size 0 > again } def print_all
( 1 2 3 4 print_all )
```

This word we just defined, `print_all`, gets every element in the stack and prints it. We do this with the `size` word, that returns the size of the current stack. Every time we print an element, the stack decreases, until it's 0 and the loop stops.

The word `loop` is very simple, it just "marks" its position. This position is then used by the word `again`. The word `again` does most of the work, it reads a boolean from the stack, if this boolean is true, it jumps back to the `loop`, if it's false, just continues the execution normally.

Sometimes we may need to break a loop before reaching the end condition. To accomplish it we have two words: `break` and `leave`.

```
{ loop 'Do it once' print break true again } def doit_once
doit_once
```

This example shows an infinite loop, because we provide a `true` to `again`, so it should loop forever. But it actually runs only once, because the word `break` ends the loop.

The second word, `leave`, does the same, but it takes an integer from the stack. This integer is the number of return stack levels it should leave to actually break the loop. It's used when we want to break a loop from within another word:

```
{ 2 leave } def actual_break
{ actual_break } def some_word
{ loop 'Do it once' print some_word true again } def doit_once
doit_once
```

Finally, we can use the word `loop`, `break` and `leave` to implement recursion:

```
 { loop 'Do it once' print break } def doit_once
 doit_once
```

The word `loop` when used without `again`, it produces a loop when the word reaches the `}`. In these cases, the only way to end the loop is with a breaking word.

## 5. Lexicons

In the previous section, [loops](#42-loops), we saw a simple usage example of `while`/`do`, the countdown. This code may look too verbose for someone comming from another programming language, where we are used to create loops with condition and action defined in the same code block. Having to separate each one of these parts in a word could sound strange, but it's actually very RunPack-style code.

Let's try to write the countdown in a different way, more like we could do in other programming languages. An **unexperienced** RunPack programmer could do something like:

```
{ loop dup print 1- dup 0 > again drop } def countdown
10 countdown
```

Explaination: This `countdown` word we just defined operates over an integer in the stack. That's why we use `dup` before comparing and printing, to avoid consuming the data, that must be used in the next loop iteration. And the final `drop` is to consume the 0 left there after leaving the loop.

If you find this code messy or hard to understand, don't worry, **it is**. It's actually pretty uggly, and that's because we are using an abstraction level that is not adequate to the job. To understand the code, we have to read each word, mentally calculate the stack effects it will produce, and move to the next word to do the same again. It's easy to miss something, and it's just a very straightforward definition, imagine a complex one! When we see the `dup 0 >`, we know we are comparing somthing with 0, but what? `dup` doesn't tell us much about it. Same for `dup print 1-`. These words are too low level. We need new words, with a higher abstraction level, that are specifically desgined to fit in our problem domain.

Knowing all this, let's try to create an alternative version of the countdown:

```
{ dup 0 > } def continue?
{ dup print } def plot
{ 1 - } def decrement
{ drop } def cleanup

{ loop plot decrement continue? again cleanup } def countdown

5 countdown
```

Wow, that was pretty verbose, wasn't it?

Yes, but look at the `countdown` definition, isn't it much more readable now? It's almost plain english. We defined four support words before `countdown`. Many, or maybe all of these words doesn't make sense outside the context of the countdown. They are closely related, all together form what Leo Brodie called a **lexicon**, in his influential book *"Thinking Forth"*. One could argue that we just took the parts of the old `countdown` definition and moved them to a different place. And that's true, but that makes the difference. First, we gave them meaningful names, so we can know what they do. Second, by splitting them apart, we can test them separately. And third, we can modify them without touching the main word.

That's the programming style we should use in RunPack. Is the way to create easy to write, read and maintain applications. We follow an iterative methodology where we atomize the program into small and simple words, we use these words to create other words with a higher abstraction level, and we group these words into lexicons.

RunPack offers a really basic but effective way to define lexicons, the pair of words `lex` and `\lex`. We can rewrite the countdown program to use them:

```
lex count
    { dup 0 > } def continue?
    { dup print } def plot
    { 1 - } def decrement
    { drop } def cleanup

    { loop count.plot count.decrement count.continue? again count.cleanup } def down
\lex

5 count.down
```

The word `lex` simply sets a prefix that will be added to every new word defined within the lexicon block. This way we can avoid name collisions, and also have the appearance of a hierarchical structure in the code.

One of the most valuable lessons you should learn from these examples is that a word definition is never too small. Even a word that only contains one word inside it (like `count.cleanup`), it's worth it if it clarifies the code.

This approach is also very flexible. Imagine that, after we finished the program, we decide that we want it to work with a variable, instead of an argument in the stack:

```
lex count
    0 var cnt
    { count.cnt! } def set
    { count.cnt 0 > } def continue?
    { count.cnt print } def plot
    { count.cnt 1- count.set } def decrement
    { 0 count.set } def cleanup

    { loop count.plot count.decrement count.continue? again count.cleanup } def down
\lex

5 count.set
count.down
```

Note that we didn't change the word `count.down` at all. We only adapted the definitions of the support words, but the main part, what contains the core logic of the module, remains the same.

Namespaces defined with `lex` can also be nested:

```
lex com
    10 var num
    lex domain
        20 var num
    \lex
\lex

com.num print
com.domain.num print
```

## 6. Word References

RunPack doesn't have a garbage collector, nor automatic reference counting, or any other built-in memory manager, it uses memory handling mechanisms provided by Rust. The reason why it's possible is simple: every time you send something from one place to another, you are cloning it. There are no pointers, memory references or shared buffers. You duplicate a string in the stack, RunPack clones the string. You drop it from the stack, RunPack deallocates it. Everything is passed by value in RunPack. So simple. But that doesn't sound too performant, right? What if I have a large piece of data stored in a variable, and I want to pass it to another word to operate with it? For these cases we have word references. You can create a word reference using the `@` word:

```
'This is a long string' def my_str
@ my_str
show_stack
```

Output:

```
Stack:
	0 : Word("my_str")
```

This code defines a variable with a string, and puts into the stack a reference to this variable.

We can even store the reference into another word:

```
def str_ref
str_ref
show_stack
```

Output:

```
Stack:
	0 : Word("my_str")
```

And execute this reference with the `exe` word:

```
exe
show_stack
```

Output:

```
Stack:
	0 : String("This is a long string")
```

Word references work with any word, not only variables:

```
@ print def print_ref
print_ref exe
```

Output:

```
This is a long string
```

Some words accept word references instead of values. For example, we know how to define a word using `def`, that takes the value from the stack and the word name from the concat. But this is just a convenient definition to make things easier, the actual primitive used to define words is `@def`, that takes both, word name and value, from the stack:

```
10 @ ten @def
ten print
```

Output: 

```
10
```

The word `def` internally uses `@def` to define words.

## 7. Advanced Topics

If you are reading this (and didn't cheat skipping chapters), it means you already know the basics of RunPack programming. Now it's time to understand the internals of the interpreter and how to interact with Rust to extend the language.

### 7.1 Cells

During this tutorial we have been reding the word "cell" here and there as a synonym of datum. Anything in the stack, an integer, float, string, whatever, is a cell. But cells are not only found in the stack, anything we execute is also a cell. Both, data and code, are composed of cells.

But what is it? Internally, a cell is a Rust enum that looks like:

```rust
pub enum Cell {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Word(String),
    Block(BlockRef),
    Struct(Struct),
}
```

Every data type supported by RunPack has its representation in this enum.

The stack is a vector of `Cell`s where we can push and pop elements. In Rust we can define a Cell and push it into the stack by doing:

```rust
use runpack::{Pack, Cell};

let mut pack = Pack::new();

// Put an integer into the stack
pack.stack.push(Cell::Integer(100));

// Put a string into the stack
pack.stack.push(Cell::String("This is a string".into()));
```

We can also pop a Cell from the stack:

```rust
if let Some(cell) = pack.stack.pop() {
    // do something with the cell
}
```

### 7.2 The Concat(enation)

After the stack, the Concat is the most important data structure in RunPack. The actual code that is executed must be stored somewhere, and this place is the Concat. Just like the stack, it is a vector of `Cell`s, but the way we use it is different. We have seen in the past that some words get arguments from the Concat, for example the word `def` here takes the word name `num` from the Concat:

```
10 def num
```

But how does it work?

When we add code to the `Pack`, using `pack.code(...)`, it is tokezined and converted into cells. These cells are appened to the Concat. The Concat contains an index to the current cell that is being executed. When the program starts, this index is 0. Then it runs a cell taken from the Concat at index position, increments the index, and starts the loop again. But a word can also get a cell from the Concat. When this happens, the index is incremented as if the cell was executed, and the execution will continue after it. Let's create a simple word that just gets a word from the concat and prints it:

```rust
use runpack::{self, Pack, Cell};

let script = r#"
    hello Andreu
"#;

let mut pack = Pack::new();
pack.dictionary.native("hello", hello_word);
pack.code(script);
pack.run().expect("Error running the script");

fn hello_word(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let Some(Cell::Word(w)) = pack.concat.next() {
        println!("Hello {}!", w);
        Ok(true)
    }
    else {
        Err(runpack::Error::new("Couldn't get a word from the concat".into()))
    }
}
```

We used the method `next()` to get the next cell from the concat. This will return a reference to the cell and increment the index. We also have the method `next_clone()` that returns a cloned cell instead of a reference.

We used Rust for this example, but we can also play with the Concat using RunPack code. We already know how to get a cell from the Concat:

```
@ my_word
```

The word `@` gets a cell from the Concat and pushes it into the stack. But what if we want to define a word that gets a cell from the concat? In this case we use the word `@@`. Let's try to recreate the `hello` word using RunPack:

```
{ 'Hello ' @@ string + '!' + print } def hello

hello Andreu
```

The word `@@` instead of getting the next cell in the concat, as `@` does, it gets the next cell in the concat of the word caller, that is `Andreu`.

With all we know now, we could even create our version of `def`:

```
{ @@ @def } def my_def

10 my_def num
```

And now comes the question, if we have two ways to pass arguments to a word (stack and concat), which one should we use?

In general, if the data needs to be dynamic, we should use the stack. And if it won't change and is defined in the moment we write the code, we can use the concat. For example, the `+` word uses two arguments in the stack, because we want to be able to add any number comming from any source. But `def` gets the word name from the concat, because it is something we want to set at the moment we write the program and it won't depend on execution results. This is a general rule, but how we use the stack and the concat should be driven by usuability and code readability criteria.

### 7.3 The Dictionary

Every time a word is found in the concat, the interpreter looks for it in the dictionary in order to execute it. Every time we define a word, a new entry is created in the dictionary.

Internally, the dictionary is a hash map where each key is a `String`, and each value is a `DictEntry` enum:

```rust
pub enum DictEntry {
    Native(fn(&mut Pack) -> Result<bool, Error>),
    Defined(BlockRef),
    Data(Cell),
}
```

From this enum we can infer the three kinds of words RunPack supports: native words (a Rust function), defined words (blocks of code) and data words (a Cell). Using RunPack we can only create two of them, defined and data words:

```
"This is a defined word"
{ 1 + } def plus_one

"These are data words"
10 def ten
'Andreu' def name
```

Native words are defined in Rust. We have already seen how to do it, in the previous chapter, when we created the word `hello`, we wrote:

```rust
pack.dictionary.native("hello", hello_word);
```

But we can also create data and defined words using Rust. Data words are easy:

```rust
pack.dictionary.data("my_num", Cell::Integer(101));
```

Defined words require a longer explanation, first we need to understand the `BlockRef` struct.

When we write a block in RunPack, like this:

```
{ 'Hello, World!' print }

show_stack
```

What we see in the stack is something like:

```
Stack:
	0 : Block(BlockRef { pos: 474, len: 3 })
```

What this is telling us is where the code block is located within the concat. The `pos` field contains the index of the first cell, in this case the string `'Hello, World!'`, and `len` is the size of the code block. It is 3 because `}` also counts, this word is like a "return" in other languages.

To create a defined word we need a `BlockRef`. To see a simple example, let's create a partial clone of `def` in Rust (it's partial because it doesn't support lexicons):

```rust
use runpack::{Pack, Cell, BlockRef, self};

let mut pack = Pack::new();
pack.dictionary.native("my_def", my_def);
pack.code(r#"
    { 1 + } my_def plus_one
    10 plus_one
"#);
pack.run().expect("Failed running the script");

if let Some(Cell::Integer(i)) = pack.stack.pop() {
    println!("Result = {}", i);
}

fn my_def(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let (Some(Cell::Block(blk)), Some(Cell::Word(w))) = (pack.stack.pop(), pack.concat.next()) {
        pack.dictionary.block(w, blk);
        Ok(true)
    }
    else {
        Err(runpack::Error::new("couldn't get arguments".into()))
    }
}
```

Our custom word `my_def` got two arguments, a block from the stack and a word from the concat. And it uses these arguments to create a new word.

### 7.4 The Return Stack

When a defined word is called, RunPack needs to know where to contnue the execution after it, and this is achieved using the return stack. For example:

```
{ 10 show_ret_stack } def just_ten
just_ten print
```

Output:

```
RetStack { stack: [486] }
10
```

As we can see, if we run `show_ret_stack` inside a word it shows the return address, the place in the concat we should return after finishing the execution of the current word.

At the moment `just_ten` is called, RunPack puts in the return stack the concat pointer of the next word, that is `print`. Then it gets the `BlockRef` of the word `just_ten` from the dictionary, and overwrites the concat pointer with the value of the `pos` field. Then the block is executed until it reaches the `}` word. This word just gets a value from the return stack, and puts it in the concat pointer, and the execution returns to the `print`.

The `RetStack` is a simple struct that only accepts two operations, pushing an address and popping an address:

```rust
pack.ret.push(address);
```

```rust
if let Some(address) = pack.ret.pop() {
    // ...
}
```

Manipulating the return stack is delicate, and must be done with care. In general, you shouldn't touch it, unless you have a very specific need that can't be achieved in any other way.

### 7.5 Custom Structs

In the chapter [7.1 Cells](#71-cells) we saw the structure of a Cell in RunPack, and in the previous and following chapters we alse have seen each one of the variants, except one, the `Struct`. The Struct variant is used to create custom data structures in Rust, and use them as normal cells in RunPack programs. For example, RunPack doesn't natively provide vectors or hash maps, but we can implement them using the `Cell::Struct` variant. To use a custom type as a `Cell::Struct` it must implement the `StructCell` trait. Let's see how we could create a simple hash map:

```rust
use std::collections::HashMap;
use runpack::{Pack, Cell, Struct, StructCell, ExtOption, self};

fn main() {
    let script = r#"
        "some code here..."
    "#;

    // Create pack
    let mut pack = Pack::new();

    // Add a custom struct to the stack
    pack.stack.push(MyMap::default().into());

    // Modify the custom struct MyMap by sending the command "set" with two arguments: key and value.
    if let Cell::Struct(s) = pack.stack.get_mut(0).unwrap() {
        s.object.doit_mut("set", Some(vec!["name".into(), "Andreu".into()]));
    }

    // Add script and run
    pack.code(script);
    pack.run().expect("Failed running the script");
}

#[derive(Default, Debug, Clone)]
struct MyMap {
    map: HashMap<Cell, Cell>,
}

impl From<MyMap> for Cell {
    fn from(val: MyMap) -> Self {
        Struct {
            name: "Map".into(),
            object: Box::new(val),
        }.into()
    }
}

impl StructCell for MyMap {
    fn object_clone(&self) -> Box<dyn StructCell> {
        Box::new(self.clone())
    }

    fn doit(&self, cmd: &str, args: Option<Vec<Cell>>) -> ExtOption {
        // Get an immutable reference to a value using a key.
        if cmd == "get" {
            if let Some(args) = args {
                if let Some(key) = args.get(0) {
                    return self.map.get(key).into();
                }
            }
        }
        ExtOption::Invalid
    }

    fn doit_mut(&mut self, cmd: &str, args: Option<Vec<Cell>>) -> ExtOption {
        match cmd {
            // Set a key-value pair.
            "set" => {
                if let Some(mut args) = args {
                    if let (Some(value), Some(key)) = (args.pop(), args.pop()) {
                        self.map.insert(key, value);
                        return ExtOption::None;
                    }
                }
            },
            // Remove a key-value pair, and returns the value.
            "rem" => {
                if let Some(args) = args {
                    if let Some(key) = args.get(0) {
                        return self.map.remove(key).into();
                    }
                }
            },
            _ => {},
        }
        ExtOption::Invalid
    }
}
```

The `StructCell` trait provides an interface for sending commands to our custom type, in two flavours, immutable (`doit`) and mutable (`doit_mut`). The possible results of a command are provided by the `ExtOption` enum:

```rust
pub enum ExtOption<'a> {
    None,
    Some(Cell),
    SomeRef(&'a Cell),
    SomeMutRef(&'a mut Cell),
    Invalid,
}
```

Each command decides the arguments it takes and what it will return. In our case, "get" requires one argument and returns a `SomeRef`, "set" two arguments and returns a `None`, and "rem" one argument and returns a `Some`. Anything else will return an `Invalid` value.

The trait interface also requieres the `object_clone()` function, that is used by custom types to clone themselves. The reason for using this instead of the standard `Clone` trait can be found in the [*object safety*](https://doc.rust-lang.org/reference/items/traits.html#object-safety) rules: a boxed dynamic trait must not require `Sized`, and `Clone` does.

With these tools we could define a set of words (a lexicon) to operate with `MyMap` instances, using the mechanisms shown in chapter [7.3 The Dictionary](#73-the-dictionary). For example, we could append this to the previous program:

```rust
// in main...
pack.dictionary.native("map.new", map_new);
pack.dictionary.native("map.set", map_set);

// at the end...
fn map_set(pack: &mut Pack) -> Result<bool, runpack::Error>  {
    if let (Some(val), Some(key), Some(Cell::Struct(mut s))) = (pack.stack.pop(), pack.stack.pop(), pack.stack.pop()) {
        if s.name == "Map" {
            if let ExtOption::None = s.object.doit_mut("set", Some(vec![key, val])) {
                pack.stack.push(s.into());
            }
        }
    }
    Ok(true)
}

fn map_new(pack: &mut Pack) -> Result<bool, runpack::Error>  {
    pack.stack.push(MyDict::default().into());
    Ok(true)
}
```

Now we can execute a RunPack program like the following:

```
map.new 'name' 'Andreu' map.set
```

And end up with something like this in the stack:

```
Stack:
	0 : Struct(Struct { name: "Map", object: MyMap { map: {String("name"): String("Andreu")} } })
```
