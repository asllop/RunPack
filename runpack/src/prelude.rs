pub const PRELUDE: &str = r#"
    "--- Word Definition ---"

    ? def 'a -> ' 'Define a word taken from the concat with the value taken from the stack: 10 def num'
    { @@ @def } { @@ } exe def @def

    ? @ ' -> a' 'Get a cell from the concat and put it in the stack: @ my_word'
    { @@ } def @

    ? setter 'a -> ' 'Create a setter for the word reference in the stack. The setter is named \'word!\': 10 def num , @ num setter , 0 num!'
    { dup string '!' + word, swap { def $ } block swap @def } def setter

    ? var 'a -> ' 'Define a variable and a setter with intial value taken from the stack and name taken from the concat: 10 var num'
    { @@ [ word val | word val word ] @def setter } def var

    "--- Stack ---"

    ? drop 'a -> ' 'Extract one cell from the stack.'
    { [ a | ] } def drop

    ? dup 'a -> a a' 'Duplicate a cell in the stack.'
    { [ a | a a ] } def dup

    ? swap 'a b -> b a' 'Swap positions of 2 cells in the stack.'
    { [ a b | a b ] } def swap

    ? wipe 'a b c ... N -> ' 'Remove all cells in the stack: ( 1 2 3 wipe )'
    { { size 0 > } { drop } loop } def wipe

    "--- Types ---"

    ? is_int? 'a -> a bool' 'Check if cell in the stack is an integer: 10 is_int?'
    { type 'integer' = } def is_int?

    ? is_float? 'a -> a bool' 'Check if cell in the stack is a float: 10.0 is_float?'
    { type 'float' = } def is_float?

    ? is_bool? 'a -> a bool' 'Check if cell in the stack is a boolean: false is_bool?'
    { type 'boolean' = } def is_bool?

    ? is_str? 'a -> a bool' 'Check if cell in the stack is a string: \'hi\' is_str?'
    { type 'string' = } def is_str?

    ? is_word? 'a -> a bool' 'Check if cell in the stack is a word: @ hi is_word?'
    { type 'word' = } def is_word?

    ? is_block? 'a -> a bool' 'Check if cell in the stack is a block: { } is_block?'
    { type 'block' = } def is_block?

    ? is_obj? 'a -> a bool' 'Check if cell in the stack is an object: ( new ) is_obj?'
    { type 'object' = } def is_obj?
    
    "--- Math ---"

    ? ++ 'a -> b' 'Increment a number in the stack: 10 ++'
    { is_int? { 1 } { 1.0 } either + } def ++

    ? -- 'a -> b' 'Decrement a number in the stack: 10 --'
    { is_int? { 1 } { 1.0 } either - } def --

    ? fract 'a -> b' 'Calculate the fractional part of a float number: 1.99 fract'
    { dup int float - } def fract

    ? sum 'a b c .. N -> z' 'Calculate sumation of all numbers in the stack: ( 1 2 3 sum )'
    { { size 1 > } { + } loop } def sum

    ? sub 'a b c .. N -> z' 'Calculate substraction of all numbers in the stack: ( 1 2 3 sub )'
    { { size 1 > } { - } loop } def sub

    ? prod 'a b c .. N -> z' 'Calculate product of all numbers in the stack: ( 1 2 3 prod )'
    { { size 1 > } { * } loop } def prod

    ? div 'a b c .. N -> z' 'Calculate division of all numbers in the stack: ( 3 6 2 div )'
    { { size 1 > } { / } loop } def div
"#;