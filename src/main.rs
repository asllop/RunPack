use runpack::{Script, Cell};

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
                rot guess.hidden =
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
        ---

        ---
        print_stack
    "#;

    println!("Program = {}", program);

    let mut script = Script::new(program);

    println!("Tokens =\n{:?}\n", script.concat);

    script.dictionary.native("print", print);
    script.dictionary.native("print_stack", print_stack);
    script.run();

    script.append("newline");
    script.append("50 twice_plus");
    script.append("dup print");
    script.run();
    if let Some(Cell::Integer(num)) = script.stack.pop() {
        println!("Got value from exec script = {}", num);
    }
    else {
        println!("Couldn't get value");
    }

    if let Some(Cell::Block(blk)) = script.stack.pop() {
        script.run_block(&blk);
    }
    else {
        println!("Couldn't get block");
    }

    script.exec("---");
    script.exec("hi");
    script.exec("num");
    script.exec("print");
    script.exec("---");
}

fn print(script: &mut Script) {
    if let Some(cell) = script.stack.pop() {
        match cell {
            Cell::Integer(i) => println!("{}", i),
            Cell::Float(f) => println!("{}", f),
            Cell::Boolean(b) => println!("{}", b),
            Cell::Symbol(s) => println!("{}", s),
            Cell::String(st) => println!("{}", st),
            Cell::Word(w) => println!("{}", w),
            _ => println!("<OTHER>")
        }
    }
    else {
        panic!("prints: couldn't get data from stack");
    }
}

fn print_stack(script: &mut Script) {
    println!("{:?}", script.stack);
}
