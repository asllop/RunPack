use runpack::{Pack, Cell, self};

fn main() {
    println!("RunPack Tutorial\n");

    // YOUR CODE GOES HERE
    let script = r#"
    10 ( 20 print_stack ) print_stack
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

// use runpack::{Pack, Cell, self};

// fn main() {
//     println!("Run Pack!\n");

//     let program = r#"
//         { '' print } def newline
//         { '--------------' print } def ---
        
//         'This is a \'string\' with \ \\scape \stuff' print

//         { 2 * ++ } def twice_plus
//         ---
//         66 ++ print
//         10 twice_plus print
//         ---
//         100 def num
//         ( num ++ ) def num
//         num print
//         ---
//         10 1 != print
//         10 10 != print
//         10 1 > print
//         10 10 > print
//         10 10 >= print
//         10 1 - print
//         print_stack
//         ---
//         2 def num
//         num print
//         { num 10 * def num } def set_num
//         print_stack
//         set_num
//         num print
//         ---
//         10 def hola
//         hola print
//         { 'Déu vos guard, amic!' print } def hola
//         hola
//         ---
//         "---------------------------------------------------------------------
//             This is a comment and won't be executed and is actually ignored
//          ---------------------------------------------------------------------"
//         'After comment' print
//         print_stack
//         ---
//         { 'Is true!' print } { 'Is false!' print } ( 10 100 > ) ifelse
//         { 'Is true!' print } { 'Is false!' print } ( 10 0 > ) ifelse
//         { 'Is true!' print } ( 10 0 > ) if
//         { 'Is true!' print } ( 10 1000 > ) if
//         ---
//         'The Guess Game' print
//         lex 'guess.'
//             10 def hidden

//             "is_correct? ( a:Int b c -- b c d:Bool )"
//             {
//                 [ c b a : b c a ] guess.hidden =
//             }
//             def is_correct?

//             "try ( a:Int -- )"
//             {
//                 { 'You found it!' print } { 'Nope :(' print } guess.is_correct? ifelse
//             }
//             def try
//         lex ''

//         5 guess.try
//         10 guess.try
//         guess.hidden print
//         ---
//         lex 'math.'
//             { 1 + } def inc
//             { 1 - } def dec
//         lex ''
//         { 'Hi!' print } def hi

//         10 math.inc print
//         10 math.dec print
//         hi
//         ---
//         10 def num
//         { num print, ( num -- ) def num } { num -1 > } while
//         print_stack
//         ---
//         10 20 30 40 [ a b c d : b d ]
//         print_stack
//         ---
//         [ a b : ] "2 drops"
//         print_stack
//         22 dup
//         print_stack
//         1 2 swap
//         print_stack
//         [ a b c d : ] "4 drops"
//         print_stack
//         ---
//         ( 'name' 'Andreu','age' 39 new ) def my_obj
//         'name' @ my_obj get print
//         'name','Joe' @ my_obj set
//         'name' @ my_obj get print
//         'name' @ my_obj key? print
//         'anything' @ my_obj key? print
//         print_stack
//         '--- Arrays ---' print
//         ( 0 'Zero', 1 555, 2 123.5 3 ( 'name' 'Andreu' new ) new ) def arr
//         0 @ arr get print
//         1 @ arr get print
//         2 @ arr get print
//         3 @ arr get print
//         print_stack
//         '--- Object with executable blocks ---' print
//         ( 'hola' { 'Hola!!' print } , 'adeu' { 'Adéu!!' print } new ) def foo_obj
//         ( 'hola' @ foo_obj get ) exe
//         ( 'adeu' @ foo_obj get ) exe
//         'Final' @ print exe
//         print_stack
//         ---
//         "Get fractional part of a float"
//         13.5 fract type
//         print_stack [ a b : ]
//         ---
//         'Are the two strings equal?' print
//         'hola' 'hola' = print
//         'Hello ' 'World!' + print
//         ---
//         55 ( 1 2 3 4 flush ) print_stack
//         ---
//     "#;

//     println!("Program = {}", program);

//     let mut pack = Pack::new_with_prelude(program);

//     println!("Tokens =\n{:?}\n", pack.concat);

//     pack.dictionary.native("print", print);
//     pack.dictionary.native("print_stack", print_stack);

//     pack.run().expect("Failed run");
// }

// fn print(pack: &mut Pack) -> Result<bool, runpack::Error> {
//     if let Some(cell) = pack.stack.pop() {
//         match cell {
//             Cell::Empty => println!("<EMPTY>"),
//             Cell::Integer(i) => println!("{}", i),
//             Cell::Float(f) => println!("{}", f),
//             Cell::Boolean(b) => println!("{}", b),
//             Cell::String(st) => println!("{}", st),
//             Cell::Word(w) => println!("{:?}", w),
//             Cell::Block(b) => println!("{:?}", b),
//             Cell::Object(o) => println!("{:?}", o),
//         }
//     }
//     else {
//         return Err(runpack::Error::new("prints: couldn't get data from stack".into(), 1000));
//     }
//     Ok(true)
// }

// fn print_stack(pack: &mut Pack) -> Result<bool, runpack::Error>  {
//     println!("{:?}", pack.stack);
//     Ok(true)
// }
