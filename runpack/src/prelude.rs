pub const PRELUDE: &str = r#"
    "--- Primitives doc ---"

    ? ( ' -> ' 'Open new nested stack: 1 2 ( 3 4 )'
    ? ) ' -> ' 'Close nested stack.'
    ? { ' -> a' 'Create a block and put block cell in the stack: { 1 + }'
    ? } ' -> ' 'Return from block, get concat position from the return stack.'
    ? [ '? -> ?' 'Create stack transfer: [ a b | a a ]'
    ? skip 'a -> ' 'Skip "a" words from the concat: -2 skip'
    ? size ' -> a' 'Get size of current stack: size'
    ? lex ' -> ' 'Set prefix for word definition: lex \'mylex.\' 10 def num lex \'\''
    ? + 'a b -> c' 'Add two numbers: 1 2 +'
    ? - 'a b -> c' 'Subtract two numbers: 1 2 -'
    ? * 'a b -> c' 'Muliply two numbers: 1 2 *'
    ? / 'a b -> c' 'Divide two numbers: 1 2 /'
    ? % 'a b -> c' 'Remainder of an integer division: 3 2 %'
    ? > 'a b -> c' 'Compare two numbers, true if a is bigger than b: 2 1 >'
    ? < 'a b -> c' 'Compare two numbers, true if a is smaller than b: 1 2 <'
    ? >= 'a b -> c' 'Compare two numbers, true if a is bigger or equal than b: 2 1 >='
    ? <= 'a b -> c' 'Compare two numbers, true if a is smaller or equal than b: 1 2 <='
    ? = 'a b -> c' 'Compare two numbers, true if a is equal to b: 2 2 ='
    ? != 'a b -> c' 'Compare two numbers, true if a is different from b: 2 1 !='
    ? and 'a b -> c' 'Calculate logic "and" of two operands: -1 -1 and'
    ? or 'a b -> c' 'Calculate logic "or" of two operands: -1 0 or'
    ? not 'a -> b' 'Calculate logic inversion of an operand: 0 not'
    ? if 'a b -> ' 'Execute block b if a is true: 2 2 = { "do something" } if'
    ? either 'a b c -> ' 'Execute block b if a is true, or block c if a is false: 2 2 = { "true block" } { "false block" } either'
    ? loop 'a b -> ' 'Execute block b while result of block a is true: 10 var num { num 0 > } { num -- num! } loop'
    ? exe 'a -> ' 'Execute a word referenced in the stack: @ a_word exe'
    ? int 'a -> b' 'Convert a float into an integer: 10.9 int'
    ? float 'a -> b' 'Convert an integer into a float: 10 float'
    ? string 'a -> b' 'Convert a word into a string: @ my_word string'
    ? word 'a -> b' 'Convert a string into a word: \'my_word\' word'
    ? type 'a -> a b' 'Get type of data in the stack without consuming it: 20 type'
    ? @@ ' -> a' 'Get a cell from the concat of current block caller, and put it in the stack: { @@ } exe my_word'
    ? @def 'a b -> ' 'Define word b with value a: 10 @ my_num @def'
    ? lex# ' -> a' 'Put value of current lex prefix in the stack: lex#'
    ? block '... a -> b' 'Get a block from the stack and create a new one. For each $ word in the block, it will get a cell from the stack and put in its place: 10 { 1 $ + } block exe'
    ? exist? 'a -> a b' 'Check if word "a" exists and puts a boolean "b" in the stack: @ my_word exist?'
    ? ? ' -> ' 'Get a word and two strings from the concat and generate help words: ? add \'a b -> c\' \'Calculate addition of two operands and put results in stack.\''

    "--- Word Definition ---"

    ? def 'a -> ' 'Define a word taken from the concat with the value taken from the stack: 10 def num'
    { @@ @def } { @@ } exe def @def

    ? @ ' -> a' 'Get a cell from the concat and put it in the stack: @ my_word'
    { @@ } def @

    ? setter 'a -> ' 'Create a setter for the word reference in the stack. The setter is named \'word!\': 10 def num , @ num setter , 0 num!'
    { dup string '!' + word, swap string lex# swap + word { def $ } block swap @def } def setter

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

    ? is_map? 'a -> a bool' 'Check if cell in the stack is an map: ( new ) is_map?'
    { type 'map' = } def is_map?

    ? is_vec? 'a -> a bool' 'Check if cell in the stack is a vector: ( vec ) is_vec?'
    { type 'vector' = } def is_vec?
    
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