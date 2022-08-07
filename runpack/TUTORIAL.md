# RunPack Tutorial

In this tutorial we are going to learn the basics of RunPack, how the interpreter works and the core functionalities.

If you are not used to [reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) it can be shocking at first, take your time to understand and test the examples.

Some programming skills are assumed, at least a basic level of Rust, and understanding of the essential data structures like stacks, and hash maps.

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
use runpack_obj;

fn main() {
    println!("RunPack Tutorial\n");

    // YOUR CODE GOES HERE
    let script = r#"
        'Hello, World!' print
    "#;

    // Create pack and register plugins
    let mut pack = Pack::new();
    runpack_obj::register(&mut pack);
    pack.dictionary.native("print", print);
    pack.dictionary.native("show_stack", show_stack);

    // Add script code and run
    pack.code(script);
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
{ 2 * } def x2
120 x2 print
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
? double2x 'a b -> x y' 'Double two numbers.'
{ x2 swap x2 swap } def double2x
```

_Note_: this word makes use of the `x2` we previously defined.

We used the word `?` to document how `double2x` works. It takes 3 arguments, the word name, the stack effects (a string) and the description (another string). Now we can use the word `help` to consult the documentation:

```
help double2x
```

Output:

```
Stack effect:	a b -> x y
Description:	Double two numbers.
```

The word `?` only does somthing in development mode (while in the REPL tool), when running a script in production mode, it will just be ignored.

## 4. Control Flow

The `x2` word we implemented in the previous chapter will only work with integers, and that's a problem, because arithmetic operators should accept integers and floats. How could we create a version of it that works with both types?

```
{ is_int? { 2 } { 2.0 } either * } def x2
5 x2 print
5.2 x2 print
```

Output:

```
10
10.4
```

We introduced multiple new things here. First, the `is_int?` word, checks if the type of the next cell in the stack is an integer, and puts a boolean with the result. Then we have the `either`. This words gets from the stack a boolean, and two blocks, the first block will be executed if the boolean is `true`, and the second if it's `false`.

If we only need a true block, we can use the `if` word:

```
{ 10 > { 'It\'s bigger than ten' print } if } def >10
100 >10
```

Output:

```
It's bigger than ten
```

Here we introduced one more word, the `>`. This word is a comparator, it gets two cells, compares if the first is bigger thant the second, and returns a boolean. There are 6 comparators: `=`, `!=`, `>`, `<`, `>=`, and `<=`.

And finally, the last control flow word is `loop`. It gets two blocks, executes the first and it the resutl is a `true` in the stack, executes the second block, and loops again until the condition block returns a `false`.

```
{ { size 0 > } { 2 * print } loop } def dobl
( 1 2 3 4 dobl )
```

Output:

```
8
6
4
2
```

This word we just defined, `dobl`, gets every integer in the stack, doubles it, and prints. We do this with the `size` word, that returns the size of the current stack. Every time we calculate a multiplication, the stack decreases, until it's 0 and the loop stops.

Now let's try to write a word to count down. We will pass an integer to it and will print the countdown until it reaches zero. 

An **unexperienced** RunPack programmer could do something like:

```
{ { dup 0 > } { dup print, -- } loop drop } def countdown
5 countdown
```

Explaination: This `countdown` word we just defined operates over an integer in the stack. That's why we use `dup` before comparing and printing, to avoid consuming the data, that must be used in the next loop iteration. And the final `drop` is to remove the 0 left there after finishing.

If you find this code messy or hard to understand, don't worry, **it is**. In the next section we are going to talk more in depth about it.

## 5. Lexicons

This section is more about how to structure applications written un RunPack.

In the previous chapter we crated a `countdown` word, but we are not very happy with the results. The resulting code is actually pretty uggly, and that's because we are using an abstraction level that is not adequate to the job. To understand the code, we have to read each word, mentally calculate the stack effects it will produce, and move to the next word to do the same again. It's easy to miss something, and it's just a very straightforward definition, imagine a complex one! When we see the `dup 0 >`, we know we are comparing somthing with 0, but what? `dup` doesn't tell us much about it. Same for `dup print, --`, and the final `drop`. These words are too low level. We need new words, with a higher abstraction level, that are specifically desgined to fit in our problem domain.

Knowing all this, let's try to create an alternative version of the countdown:

```
{ dup 0 > } def continue_count?
{ dup print } def print_count
{ 1 - } def dec_count
{ drop } def clean_count

{ { continue_count? } { print_count dec_count } loop clean_count } def countdown

5 countdown
```

Wow, that was pretty verbose, wasn't it?

Yes, but look at the `countdown` definition. Isn't it much more readable now? It's almost plain english. We defined four support words before `countdown`. Many, or maybe all of these words doesn't make sense outside the context of the countdown. They are closely related, all together form what Leo Brodie called a **lexicon**, in his indispensable book *"Thinking Forth"*. One could argue that we just took the parts of the old `countdown` definition and moved them to a different place. And that's true, but that makes the difference. First, we gave them meaningful names, so we can know what they do. Second, by splitting them apart, we can test them separately. And third, we can modify them without touching the main word.

That's the programming style we should use in RunPack. Is the way to create easy to write, read and maintain applications. We follow an iterative methodology where we atomize the program into small and simple words, we use these words to create other words with a higher abstraction level, and we group these words into lexicons.

RunPack offers a really basic but effective way to define lexicons, the word `lex`. We can rewrite the countdown program to use it:

```
lex 'count.'
    { dup 0 > } def continue?
    { dup print } def print
    { 1 - } def dec
    { drop } def clean

    { { count.continue? } { count.print count.dec } loop count.clean } def down
lex ''

5 count.down
```

The word `lex` simply sets a prefix that will be added to every word defined with `def`. This way we can avoid name collisions, and also have the appearance of a hierarchical structure in the code.

One of the most valuable lessons you should learn from these examples is that a word definition is never too small. Even a word that only contains one word inside it (like `count.clean`), it's worth it if it clarifies the code.

This approach is also very flexible. Imagine that, after we finished the program, we decide that we want it to work with a variable, instead of an argument in the stack:

```
lex 'count.'
    0 var cnt
    { count.cnt! } def set
    { count.cnt 0 > } def continue?
    { count.cnt print } def print
    { count.cnt -- count.set } def dec
    { 0 count.set } def clean

    { { count.continue? } { count.print count.dec } loop count.clean } def down
lex ''

5 count.set
count.down
```

Note that we didn't change the word `count.down` at all. We only adapted the definitions of the support words, but the main part, what contains the core logic of the module, remains the same.

## 6. Word References

RunPack doesn't have a garbage collector, nor automatic reference counting, or any other built-in memory manager. It uses the Rust memory handling mechanisms. The reason why it's possible is simple: every time you send something from one place to another, you are cloning it. There are no pointers, memory references or shared buffers. You duplicate a string in the stack, RunPack clones the string. You drop it from the stack, RunPack deallocates it. So simple. Everything is passed by value in RunPack. But that doesn't sound too performant, right? What if I have a large piece of data stored in a variable, and I want to pass it to a word to operate? For these kind of cases we have word references. You can create a word reference using the `@` word:

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
    Empty,
    Integer(IntegerType),
    Float(FloatType),
    Boolean(bool),
    String(String),
    Word(String),
    Block(BlockRef),
    Object(Object),
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

When we add code to the `Pack`, using `pack.code(...)`, it is tokezined and converted into cells. These cells are appened to the Concat. The Concat contains a pointer to the current cell that is being executed. When the program starts, this pointer is 0. Then it runs a cell taken from the Concat at pointer position, increments the pointer, and starts the loop again. But a word can also get a cell from the Concat. When this happens, the pointer is incremented as if the cell was executed, and the execution will continue after it. Let's create a simple word that just gets a word from the concat and prints it:

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
        Err(runpack::Error::new("Couldn't get a word from the concat".into(), 1000))
    }
}
```

We used the method `next()` to get the next cell from the concat. This will return a reference to the cell and increment the pointer. We also have the method `next_clone()` that returns a cloned cell instead of a reference.

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

In general, if the data needs to be dynamic, we should use the stack. And if it won't change and is defined in the moment we write the code, we can use the concat. For example, the `+` word uses two arguments in the stack, because we want to be able to add any number comming from any source. But `def` gets the word name from the concat, because it is something we want to set at the moment we write the program and it won't depend on execution results. This is a general rule, but how we use the stack and the concat must be driven by usuability and code readability criteria.

### 7.3 The Dictionary

TODO

### 7.4 The Return Stack

TODO