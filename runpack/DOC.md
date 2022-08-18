# Vocabulary

## !=

Stack Effects:

```
a b -> c
```
Description:

```
Compare two numbers, true if a is different from b: 2 1 !=
```

## %

Stack Effects:

```
a b -> c
```
Description:

```
Remainder of an integer division: 3 2 %
```

## (

Stack Effects:

```
 -> 
```
Description:

```
Open new nested stack: 1 2 ( 3 4 )
```

## )

Stack Effects:

```
 -> 
```
Description:

```
Close nested stack.
```

## *

Stack Effects:

```
a b -> c
```
Description:

```
Muliply two numbers: 1 2 *
```

## +

Stack Effects:

```
a b -> c
```
Description:

```
Add two numbers: 1 2 +
```

## ++

Stack Effects:

```
a -> b
```
Description:

```
Increment a number in the stack: 10 ++
```

## -

Stack Effects:

```
a b -> c
```
Description:

```
Subtract two numbers: 1 2 -
```

## --

Stack Effects:

```
a -> b
```
Description:

```
Decrement a number in the stack: 10 --
```

## /

Stack Effects:

```
a b -> c
```
Description:

```
Divide two numbers: 1 2 /
```

## <

Stack Effects:

```
a b -> c
```
Description:

```
Compare two numbers, true if a is smaller than b: 1 2 <
```

## <=

Stack Effects:

```
a b -> c
```
Description:

```
Compare two numbers, true if a is smaller or equal than b: 1 2 <=
```

## =

Stack Effects:

```
a b -> c
```
Description:

```
Compare two numbers, true if a is equal to b: 2 2 =
```

## >

Stack Effects:

```
a b -> c
```
Description:

```
Compare two numbers, true if a is bigger than b: 2 1 >
```

## >=

Stack Effects:

```
a b -> c
```
Description:

```
Compare two numbers, true if a is bigger or equal than b: 2 1 >=
```

## ?

Stack Effects:

```
 -> 
```
Description:

```
Get a word and two strings from the concat and generate help words: ? add 'a b -> c' 'Calculate addition of two operands and put results in stack.'
```

## @

Stack Effects:

```
 -> a
```
Description:

```
Get a cell from the concat and put it in the stack: @ my_word
```

## @@

Stack Effects:

```
 -> a
```
Description:

```
Get a cell from the concat of current block caller, and put it in the stack: { @@ } exe my_word
```

## @def

Stack Effects:

```
a b -> 
```
Description:

```
Define word b with value a: 10 @ my_num @def
```

## [

Stack Effects:

```
? -> ?
```
Description:

```
Create stack transfer: [ a b | a a ]
```

## and

Stack Effects:

```
a b -> c
```
Description:

```
Calculate logic "and" of two operands: -1 -1 and
```

## block

Stack Effects:

```
... a -> b
```
Description:

```
Get a block from the stack and create a new one. For each $ word in the block, it will get a cell from the stack and put in its place: 10 { 1 $ + } block exe
```

## def

Stack Effects:

```
a -> 
```
Description:

```
Define a word taken from the concat with the value taken from the stack: 10 def num
```

## div

Stack Effects:

```
a b c .. N -> z
```
Description:

```
Calculate division of all numbers in the stack: ( 3 6 2 div )
```

## drop

Stack Effects:

```
a -> 
```
Description:

```
Extract one cell from the stack.
```

## dup

Stack Effects:

```
a -> a a
```
Description:

```
Duplicate a cell in the stack.
```

## either

Stack Effects:

```
a b c -> 
```
Description:

```
Execute block b if a is true, or block c if a is false: 2 2 = { "true block" } { "false block" } either
```

## exe

Stack Effects:

```
a -> 
```
Description:

```
Execute a word referenced in the stack: @ a_word exe
```

## exist?

Stack Effects:

```
a -> a b
```
Description:

```
Check if word "a" exists and puts a boolean "b" in the stack: @ my_word exist?
```

## float

Stack Effects:

```
a -> b
```
Description:

```
Convert an integer into a float: 10 float
```

## fract

Stack Effects:

```
a -> b
```
Description:

```
Calculate the fractional part of a float number: 1.99 fract
```

## if

Stack Effects:

```
a b -> 
```
Description:

```
Execute block b if a is true: 2 2 = { "do something" } if
```

## int

Stack Effects:

```
a -> b
```
Description:

```
Convert a float into an integer: 10.9 int
```

## is_block?

Stack Effects:

```
a -> a bool
```
Description:

```
Check if cell in the stack is a block: { } is_block?
```

## is_bool?

Stack Effects:

```
a -> a bool
```
Description:

```
Check if cell in the stack is a boolean: false is_bool?
```

## is_float?

Stack Effects:

```
a -> a bool
```
Description:

```
Check if cell in the stack is a float: 10.0 is_float?
```

## is_int?

Stack Effects:

```
a -> a bool
```
Description:

```
Check if cell in the stack is an integer: 10 is_int?
```

## is_map?

Stack Effects:

```
a -> a bool
```
Description:

```
Check if cell in the stack is a map: ( new ) is_map?
```

## is_str?

Stack Effects:

```
a -> a bool
```
Description:

```
Check if cell in the stack is a string: 'hi' is_str?
```

## is_word?

Stack Effects:

```
a -> a bool
```
Description:

```
Check if cell in the stack is a word: @ hi is_word?
```

## lex

Stack Effects:

```
 -> 
```
Description:

```
Set prefix for word definition: lex 'mylex.' 10 def num lex ''
```

## lex#

Stack Effects:

```
 -> a
```
Description:

```
Put value of current lex prefix in the stack: lex#
```

## loop

Stack Effects:

```
a b -> 
```
Description:

```
Execute block b while result of block a is true: 10 var num { num 0 > } { num -- num! } loop
```

## not

Stack Effects:

```
a -> b
```
Description:

```
Calculate logic inversion of an operand: 0 not
```

## or

Stack Effects:

```
a b -> c
```
Description:

```
Calculate logic "or" of two operands: -1 0 or
```

## prod

Stack Effects:

```
a b c .. N -> z
```
Description:

```
Calculate product of all numbers in the stack: ( 1 2 3 prod )
```

## setter

Stack Effects:

```
a -> 
```
Description:

```
Create a setter for the word reference in the stack. The setter is named 'word!': 10 def num , @ num setter , 0 num!
```

## size

Stack Effects:

```
 -> a
```
Description:

```
Get size of current stack: size
```

## skip

Stack Effects:

```
a -> 
```
Description:

```
Skip "a" words from the concat: -2 skip
```

## string

Stack Effects:

```
a -> b
```
Description:

```
Convert a word into a string: @ my_word string
```

## sub

Stack Effects:

```
a b c .. N -> z
```
Description:

```
Calculate substraction of all numbers in the stack: ( 1 2 3 sub )
```

## sum

Stack Effects:

```
a b c .. N -> z
```
Description:

```
Calculate sumation of all numbers in the stack: ( 1 2 3 sum )
```

## swap

Stack Effects:

```
a b -> b a
```
Description:

```
Swap positions of 2 cells in the stack.
```

## type

Stack Effects:

```
a -> a b
```
Description:

```
Get type of data in the stack without consuming it: 20 type
```

## var

Stack Effects:

```
a -> 
```
Description:

```
Define a variable and a setter with intial value taken from the stack and name taken from the concat: 10 var num
```

## wipe

Stack Effects:

```
a b c ... N -> 
```
Description:

```
Remove all cells in the stack: ( 1 2 3 wipe )
```

## word

Stack Effects:

```
a -> b
```
Description:

```
Convert a string into a word: 'my_word' word
```

## {

Stack Effects:

```
 -> a
```
Description:

```
Create a block and put block cell in the stack: { 1 + }
```

## }

Stack Effects:

```
 -> 
```
Description:

```
Return from block, get concat position from the return stack.
```

