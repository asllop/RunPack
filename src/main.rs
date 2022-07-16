use runpack::{Script, Cell, Error as RPError};

fn main() {
    println!("Run Pack!\n");

    let program = r#"
        { '' print } def newline
        { '--------------' print } def ---
        
        'This is a \'string\' with \ \\scape \stuff' print
        #hello_world print

        { 2 * inc } def twice_plus
        { 1 + } def inc
        { 1 - } def dec
        { [ a : ] } def drop
        { [ a : a a ] } def dup
        { [ a b : a b ] } def swap
        ---
        66 inc print
        10 twice_plus print
        ---
        100 def num
        ( num inc ) def num
        num print
        ---
        { = ! } def !=
        10 1 != print
        10 10 != print
        10 1 > print
        10 10 > print
        10 1 - print
        print_stack
        ---
        2 def num
        num print
        { num 10 * def num } def set_num
        print_stack
        set_num
        num print
        ---
        10 def hola
        hola print
        { 'Déu vos guard, amic!' print } def hola
        hola
        ---
        "---------------------------------------------------------------------
            This is a comment and won't be executed and is actually ignored
         ---------------------------------------------------------------------"
        'After comment' print
        print_stack
        ---
        { 'Is true!' print } { 'Is false!' print } ( 10 100 > ) ifelse
        { 'Is true!' print } { 'Is false!' print } ( 10 0 > ) ifelse
        { 'Is true!' print } ( 10 0 > ) if
        { 'Is true!' print } ( 10 1000 > ) if
        ---
        'The Guess Game' print
        lex 'guess.'
            10 def hidden

            "is_correct? ( a:Int b c -- b c d:Bool )"
            {
                [ c b a : b c a ] guess.hidden =
            }
            def is_correct?

            "try ( a:Int -- )"
            {
                { 'You found it!' print } { 'Nope :(' print } guess.is_correct? ifelse
            }
            def try
        lex ''

        5 guess.try
        10 guess.try
        guess.hidden print
        ---
        lex 'math.'
            { 1 + } def inc
            { 1 - } def dec
        lex ''
        { 'Hi!' print } def hi

        10 math.inc print
        10 math.dec print
        hi
        ---
        "Puts a block in the stack"
        { 'Hello from block' print }
        'This is a test' print
        ---
        10 def num
        { num print, ( num dec ) def num } { num -1 > } while
        print_stack
        ---
        10 20 30 40 [ a b c d : b d ]
        print_stack
        ---
        [ a b : ] "2 drops"
        print_stack
        22 dup
        print_stack
        1 2 swap
        print_stack
        [ a b c d : ] "4 drops"
        print_stack
        ---
        ( 'name' 'Andreu','age' 39 new ) def my_obj
        'name' get my_obj print
        'name','Joe' set my_obj
        'name' get my_obj print
        'name' key? my_obj print
        'anything' key? my_obj print
        ---
    "#;

    println!("Program = {}", program);

    let mut script = Script::new(program);

    println!("Tokens =\n{:?}\n", script.concat);

    script.dictionary.native("print", print);
    script.dictionary.native("print_stack", print_stack);
    script.run().expect("Failed run");

    script.append("newline");
    script.append("50 twice_plus");
    script.run().expect("Failed run");
    if let Some(Cell::Integer(num)) = script.stack.pop() {
        println!("Got value from exec script = {}", num);
    }
    else {
        println!("Couldn't get value");
    }

    if let Some(Cell::Block(blk)) = script.stack.pop() {
        script.run_block(&blk).expect("Failed run block");
    }
    else {
        println!("Couldn't get block");
    }

    script.exec("---").expect("Failed exec");
    script.exec("hi").expect("Failed exec");
    script.exec("num").expect("Failed exec");
    script.exec("print").expect("Failed exec");
    script.exec("---").expect("Failed exec");

    script.append("{ } print_stack");
    script.run().expect("Failed run");
}

fn print(script: &mut Script) -> Result<bool, RPError> {
    if let Some(cell) = script.stack.pop() {
        match cell {
            Cell::Integer(i) => println!("{}", i),
            Cell::Float(f) => println!("{}", f),
            Cell::Boolean(b) => println!("{}", b),
            Cell::Symbol(s) => println!("{}", s),
            Cell::String(st) => println!("{}", st),
            Cell::Word(w) => println!("{}", w),
            Cell::Empty => println!("<EMPTY>"),
            Cell::Block(b) => println!("{:?}", b),
            Cell::Object(o) => println!("{:?}", o),
        }
    }
    else {
        return Err(RPError::new("prints: couldn't get data from stack".into(), 1000));
    }
    Ok(true)
}

fn print_stack(script: &mut Script) -> Result<bool, RPError>  {
    println!("{:?}", script.stack);
    Ok(true)
}
