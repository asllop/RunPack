pub const PRELUDE: &str = r#"
    "--- Word Definition ---"

    ? def 'a -> ' 'Define a word taken from the concat with the value taken from the stack.'
    { @@ @def } { @@ } exe def @def

    ? @ ' -> a' 'Get a cell from the concat and put it in the stack.'
    { @@ } def @

    ? setter 'a -> ' 'Create a setter for the word reference in the stack. The setter is named \'word!\''
    { dup string '!' + word, swap { def $ } block swap @def } def setter

    ? var 'a -> ' 'Define a variable and a setter with intial value taken from the stack and name taken from the concat.'
    { @@ [ word val | word val word ] @def setter } def var

    "--- Stack ---"

    ? drop 'a -> ' 'Extract one cell from the stack.'
    { [ a | ] } def drop

    ? dup 'a -> a a' 'Duplicate a cell in the stack.'
    { [ a | a a ] } def dup

    ? swap 'a b -> b a' 'Swap positions of 2 cells in the stack.'
    { [ a b | a b ] } def swap

    ? flush 'a b c ... N -> ' 'Remove all cells in the stack.'
    { { size 0 > } { drop } loop } def flush

    "--- Types ---"

    ? is_int? 'a -> bool' 'Check if cell in the stack is an integer.'
    { type 'integer' = } def is_int?

    ? is_float? 'a -> bool' 'Check if cell in the stack is a float.'
    { type 'float' = } def is_float?

    ? is_bool? 'a -> bool' 'Check if cell in the stack is a boolean.'
    { type 'boolean' = } def is_bool?

    ? is_str? 'a -> bool' 'Check if cell in the stack is a string.'
    { type 'string' = } def is_str?

    ? is_word? 'a -> bool' 'Check if cell in the stack is a word.'
    { type 'word' = } def is_word?

    ? is_block? 'a -> bool' 'Check if cell in the stack is a block.'
    { type 'block' = } def is_block?

    ? is_obj? 'a -> bool' 'Check if cell in the stack is an object.'
    { type 'object' = } def is_obj?
    
    "--- Math ---"

    ? ++ 'a -> b' 'Increment a number in the stack.'
    { is_int? { 1 } { 1.0 } either + } def ++

    ? -- 'a -> b' 'Decrement a number in the stack.'
    { is_int? { 1 } { 1.0 } either - } def --

    ? fract 'a -> b' 'Calculate the fractional part of a float number.'
    { dup int float - } def fract

    ? sum 'a b c .. N -> z' 'Calculate sumation of all numbers in the stack.'
    { { size 1 > } { + } loop } def sum

    ? sub 'a b c .. N -> z' 'Calculate substraction of all numbers in the stack.'
    { { size 1 > } { - } loop } def sub

    ? prod 'a b c .. N -> z' 'Calculate product of all numbers in the stack.'
    { { size 1 > } { * } loop } def prod

    ? div 'a b c .. N -> z' 'Calculate division of all numbers in the stack.'
    { { size 1 > } { / } loop } def div
"#;