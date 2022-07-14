use runpack::{Script, Cell, Stack, Concat, Dictionary, RetStack};

fn main() {
    println!("Run Pack!\n");

    let program = r#"
        'This is a \'string\' with \ \\scape \stuff' print
        #hello_world print

        { 2 * inc } def twice_plus
        { 1 + } def inc
        '--------------' print
        66 inc print
        10 twice_plus print
        '--------------' print
        100 def num
        ( num inc ) def num
        num print
        '--------------' print
        { = ! } def !=
        10 1 != print
        10 10 != print
        10 1 > print
        10 10 > print
        10 1 - print
        print_stack
        '--------------' print
        2 def num
        num print
        { num 10 * def num } def set_num
        print_stack
        set_num
        num print
        '--------------' print
        10 def hola
        hola print
        { 'Déu vos guard, amic!' print } def hola
        hola
        '--------------' print
    "#;

    println!("Program = {}", program);

    let mut script = Script::new(program.bytes());
    script.dictionary.native("print", print);
    script.dictionary.native("print_stack", print_stack);
    script.run();

    script.exec("50 twice_plus".bytes());
    if let Some(Cell::Integer(num)) = script.stack.pop() {
        println!("Got value from exec script = {}", num);
    }
    else {
        println!("Couldn't get value");
    }
}

fn print(stack: &mut Stack, _: &mut Concat, _: &mut Dictionary, _: &mut RetStack) {
    if let Some(cell) = stack.pop() {
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

fn print_stack(stack: &mut Stack, _: &mut Concat, _: &mut Dictionary, _: &mut RetStack) {
    println!("{:?}", stack);
}
