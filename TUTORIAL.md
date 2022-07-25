# RunPack Tutorial

This tutorial should be read sequentially. Some programming skills are assumed, at least a basic level of Rust, and understanding of the essential data structures like stacks, and hash maps.

If you are not used to [reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) it can be shocking at first. Take your time to understand and test the examples.

## 0. Setup

All RunPack scripts in this tutorial have been executed in the following program:

```rust
use runpack::{Pack, Cell, self};
use runpack_obj;

fn main() {
    println!("RunPack Tutorial\n");

    // YOUR CODE GOES HERE
    let script = r#"
        'Hello, World!' print
    "#;

    let mut pack = Pack::new_with_prelude(script);
    runpack_obj::register(&mut pack);
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

To push something into the stack we simply do:

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

All good. Now we want to pop data out from the stack. How can we do that?

```
10 20 + print_stack
```

Output:

```
Stack:
	0 : Integer(30)
```

Interesting. We put two integers in the stack, then we called the word `+` and now the stack contains one integer with value 30. The word `+` popped two integers from the stack, performed an addition, and finally pushed the resulting integer into the stack. And this is how RunPack works, the way we execute subroutines and pass arguments to them.

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

### Stack Transfers

Sometimes it can be hard to work with the stack. Our word needs the arguments in the stack in a certain way, but the current stack state left by the previous words is far from what we want. For these cases we have the stack transfer operator.

Let's imagine our current stack state is as follows:

```
0.5 'A string' 100.0 false
```

And we have to multiply the two floats, so we need them to be consecutive in the stack, right at the top of it. That's the perfect fit for the stack transfer operator:

```
0.5 'A string' 100.0 false
[ a, flt_a, b, flt_b | b, a, flt_b, flt_a ] * print
print_stack
```

Output:

```
50
Stack:
	1 : String("A string")
	0 : Boolean(false)
```

**Beware of the spaces!** In RunPack, the space is the word delimiter. Writing "`[ a`" is a totally different thing than writing "`[a`". In the former case we have 2 words, `[` and `a`. In the latter, we have only one word, identified as `[a`.

Also, note that in RunPack, the comma is just a word separator, like the space. It has no other meaning, it's only used to improve readability, and is optional.

The stack transfer has the following format:

```
[ pop_1 pop_2 ... pop_N | push_1 push_2 ... push_N ]
```

The variables at the left of `|` are popped from the stack in the order they appear. The variables at the right of `|` are pushed into the stack in the order they appear. In the example of the two floats, we popped the following variables: `a` getting the value `false`, `flt_a` getting `100.0`, `b` getting `'A string'`, and `flt_b` getting `0.5`. Then we pushed them in the order we see: `b`(`'A string'`), `a`(`false`), `flt_b`(`0.5`) and `flt_a`(`100.0`). As a result, the `*` word will find the two floats in the stack to multiply them, and the other two data cells will remain untouched.

The variable in the left side can't be repeaded, but they can appear multiple times in the right side, or don't appear at all. For example, if we have 3 cells and want to remove the one in the middle, we could do:

```
1 2 3 [ a b c | c a ] print_stack
```

Output:

```
Stack:
	1 : Integer(1)
	0 : Integer(3)
```

Or maybe we want to triple a cell:

```
100 [ a | a a a ] print_stack
```

Output:

```
Stack:
	2 : Integer(100)
	1 : Integer(100)
	0 : Integer(100)
```

### Nested Stacks

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

Explanation: The word `(` opens a new stack nested inside the current one. From now on, this new stack will be our current. When we push `20` we are pushing into the nested stack, and thus, this cell doesn't live in the same stack as the `10` we pushed before, because it's located in the previous stack. That's why in the first `print_stack` we only see the `20`. Then we run the word `)`, that closes a nested stack. When this happens, all data in the current stack goes to the parent stack, in our case, the `20`. that's why the second `print_stack` shows both, the `10` and the `20`.

Nested stacks are useful for operations that use all the data from the stack, because it allows us to demarcate the limits of these operations. For example, the word `flush`, that removes all cells from the stack. We will see more usage examples in the following chapters.

## 2. Arithmetic & Logic operations

We have already seen some of them. Arithmetic operations can work either with integers or floats, but can't mix them. There are five: addition, subtraction, multiplication, division, and remainder of a division.

```
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
1 2 + 3 + 4 5 + 6 + 7 8 + 9 + * *
```

Pretty ugly, uh?

For this kind of cases we have the sequence operations: `sum` and `prod`:

```
( ( 1 2 3 sum ) ( 4 5 6 sum ) ( 7 8 9 sum ) prod )
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

This syntax may look contradictory for a language that uses Reverse Polish Notation, it seems to violate the rules. The word `def` uses an argument that is in the stack, that's good, but then it uses another argument, the word name, that is not in the stack. For now let's leave it, we will understand what's going on once we get into the [Concat](#the-concat).

Now, what happens when we execute these words?

```
'Andreu' def name
555555 def phone

name
phone
print_stack
```

Output:

```
Stack:
	1 : String("Andreu")
	0 : Integer(555555)
```

The values we assigned to these words are now in the stack. These words act as a constant or a variable, executing `name` is equivalent to execute `'Andreu'`.

Before proceeding to the next step, we will talk about a new data type: the Block. To define a block we use the words `{` and `}` (I know I'm being a bit of a nagger, but please, beware of the spaces):

```
{ 'We are in a block' print }
print_stack
```

Output:

```
Stack:
	0 : Block(BlockRef { pos: 115, len: 3 })
```

A block is a piece of code, but is also a data type and can be treated as data. It can be pushed into the stack, sent to other words, and returned by them. And of course, stored in a word definition:

```
{ 'We are in a block' print } def we_are
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

### Stack Effect Comments

Because of the dynamic nature of RunPack and the use of the stack, there is no way to know the arguments a word takes and the results it produces without inspecting and understending the code. For this reason we have the stack effect comments, to describe in a fast and readable way how a word affects the stack. The format for this comments is:

```
"word_name a,b,c -> x,y,z"
{ . . . } def word_name
```

Where a, b, and c, are the contents of the stack before executing the word, and that are used by it, and x, y, and z, are the contents of the stack after executing the word, and that are produced by it. Optionally we can also include, underneath, a description of what the word does.

Let's take the `x2` word we previously defined:

```
"x2 a -> b"
" Description: Takes an integer and doubles it."
{ 2 * } def x2
```

This comment is telling us that this word takes an argument, called `a`, and consumes it. And as a results it pushes another element, `b`. We could also specify the data types:

```
"x2 a_i -> b_i"
" Description: Takes an integer and doubles it."
{ 2 * } def x2
```

The abreviations for the types are: i (integer), f (float), s (string), b (boolean), o (object), w (word), and k (block). If the word accepts multiple data types we can concatenate multiple type abreviations:

```
"my_word a_i_f -> b_i_f"
```

## 4. Control Flow

The `x2` word we implemented in the previous chapter will only work with integers, and that's a problem, because arithmetic operators should accept integers and floats. How could we create a version of it that works with both types?

```
{ is_int? { 2 } { 2.0 } ifelse * } def x2
5 x2 print
5.2 x2 print
```

Output:

```
10
10.4
```

We introduced multiple new things here. First, the `is_int?` word, checks if the type of the next cell in the stack is an integer, and puts a boolean with the result. Then we have the `ifelse`. This words gets from the stack a boolean, and two blocks, the first block will be executed if the boolean is `true`, and the second if it's `false`.

There is a simplified version that only has the true block:

```
{ 10 > { 'It\'s bigger than ten' print } if } def >10
100 >10
```

Output:

```
It's bigger than ten
```

Here we introduced one more word, the `>`. This word is a comparator, it gets two cells, compares if the first is bigger thant the second, and returns a boolean. There are 6 comparators: `=`, `!=`, `>`, `<`, `>=`, and `<=`.

And the last control flow word is `while`. It works pretty much like `if`, but the condition, instead of being a simple boolean, is a block that must return a boolean:

```
{ { dup 0 > } { dup print, -- } while drop } def countdown
5 countdown
```

Output:

```
5
4
3
2
1
```

This `countdown` word we just defined operates over an integer in the stack. That's why we use `dup` before comparing and printing, to avoid consuming the data, that must be used in the next loop iteration. And the final `drop` is to remove the 0 left there after finishing. If you find this code messy or hard to understand, don't worry, it is. In the next section we are going to talk more in depth about it.

## 5. Lexicons

This section is more about how to structure applications written un RunPack. We will start from the `countdown` example used in the previous chapter, and will create an alternative version of it:

```
{ dup 0 > } def continue_count?
{ dup print } def print_count
{ 1 - } def dec_count
{ drop } def clean_count
{ { continue_count? } { print_count dec_count } while clean_count } def countdown

5 countdown
```

Wow, that was pretty verbose, wasn't it?

Yes, but look at the `countdown` definition. Isn't it much more readable now? It's almost plain english. We defined four support words before `countdown`. Many, or maybe all, of these words doesn't make sense outside the context of the countdown. They are closely related, all together form what Leo Brodie called a *lexicon*, in his indispensable book *"Thinking Forth"*. 

That's the programming style we should use in RunPack. Is the way to create easy to write, read and maintain applications. We atomize the program into small and simple words, then use these words to create other words with a higher abstraction level, and finally group them into lexicons.

RunPack offers a really straightforward but effective way to define lexicons, the word `lex`. We can rewrite the countdown program to use it:

```
lex 'count.'
    { dup 0 > } def continue?
    { dup print } def print
    { 1 - } def dec
    { drop } def clean
    { { count.continue? } { count.print count.dec } while count.clean } def down
lex ''

5 count.down
```

The word `lex` simply sets a prefix that will be added to every word defined with `def`. This way we can avoid name collisions, and also have the appearance of a hierarchical structure in the code.

One of the most valuable lessons you should learn from these examples is that a word definition is never too small. Even a word that only contains one word inside it (like `count.clean`), it's worth it if it clarifies the code.

This approach is also very flexible. Imagine that, after we finished the program, we decide that we want it to work with a variable, instead of an argument in the stack:

```
lex 'count.'
    0 def var
    { count.var 0 > } def continue?
    { count.var print } def print
    { count.var 1 - def count.var } def dec
    { 0 def count.var } def clean
    { { count.continue? } { count.print count.dec } while count.clean } def down
lex ''

5 def count.var
count.down
```

Note that we didn't change the word `count.down` at all. We only adapted the definitions of the support words, but the main part, what contains the core logic of the module, remains the same.

## 6. Word References

RunPack doesn't have a garbage collector, nor automatic reference counting, or any other built-in memory manager. It uses the Rust memory handling mechanisms. The reason why it's possible is simple: every time you send something from one place to another, you are cloning it. There are no pointers, memory references or shared buffers. You duplicate a string in the stack, RunPack clones the string. You drop it from the stack, RunPack deallocates it. So simple. Everything is passed by value in RunPack. But that doesn't sound too performant, right? What if I have a large piece of data stored in a variable, and I want to pass it to a word to operate? For these kind of cases we have word references. You can create a word reference using the `@` word:

```
'This is a string' def my_str
@ my_str
print_stack
```

Output:

```
Stack:
	0 : Word("my_str")
```

This code defines a variable with a string, and puts into the stack a reference to this variable.

We can even store the reference into another word:

```
'This is a string' def my_str
@ my_str def str_ref
str_ref
print_stack
```

Output:

```
Stack:
	0 : Word("my_str")
```

And execute this reference with the `exe` word:

```
'This is a string' def my_str
@ my_str def str_ref
str_ref exe
print_stack
```

Output:

```
Stack:
	0 : String("This is a string")
```

Word references work with any word, not only variables:

```
@ print def print_ref
'Hello from ref' print_ref exe
```

Output:

```
Hello from ref
```

Some words accept word references instead of values. We will see some examples in the following chapter.

## 7. Objects

There is still one data type we haven't covered yet: the object. In RunPack an object is a set of key-value pairs, internally implemented with a hash map. Key and value can be of any type, integer, float, string, boolean, word, block, even another object.

To define an object, we use the `new` word:

```
( 'name' 'Joe'
  'phone' 5555555 new )
print_stack
```

Output:

```
Stack:
	0 : Object(Object { map: {String("phone"): Integer(5555555), String("name"): String("Joe")} })
```

As always, we can store it in a word using `def`:

```
( 'name' 'Andreu'
  'phone' 5555555 new ) def my_obj
```

We use a pair of words to `set` and `get` values from an object:

```
( 'name' 'Andreu'
  'phone' 5555555 new ) def my_obj

'name' @ my_obj get print
'name' 'Joe' @ my_obj set
'name' @ my_obj get print
```

Output:

```
Andreu
Joe
```

These operators use a [word reference](#6-word-references) to acces the object, to avoid cloning it in the stack over and over again.

We can check if a key exists in an object with the `key?` word:

```
( 'name' 'Andreu'
  'phone' 5555555 new ) def my_obj

'name' @ my_obj key? print
'xxxx' @ my_obj key? print
```

Output:

```
true
false
```

Vectors are just normal objects, with the particularity of having integer keys:

```
( 12.34, 'A string', 1000, true vec ) def my_vec
1 @ my_vec get print
@ my_vec len print
```

Output:

```
A string
4
```

There is also an operator to "run" keys, the `:` word:

```
(
    'name' 'Andreu'
    'hi' { 'Hello, World!' print }
    new
) def my_obj

@ my_obj : 'hi'
@ my_obj : 'name' print
```

Output:

```
Hello, World!
Andreu
```

And an operator to run keys as if they were methods, passing a reference to the object in the stack:

```
{ dup : val_a } def get_a
{ swap : val_b } def get_b
(
    @ +         { get_a get_b + }
    @ val_a     10
    @ val_b     20
    new
)
def my_obj

@ my_obj . + print
```

Output:

```
30
```

In this case we are using words as keys instead of strings. The key `+` contains a block. When executed using "`.`" it gets the object reference from the stack, obtains the values of `val_a` and `val_b`, and add them.

## 8. Advanced Topics

If you are reading this (and didn't cheat skipping chapters), it means you already know all the aspects of RunPack programming. Now it's time to understand the internals of the interpreter and how to interact with Rust to extend the language.

### The Cell

TODO

### The Dictionary

TODO

### The Concat

TODO

### The Return Stack

TODO