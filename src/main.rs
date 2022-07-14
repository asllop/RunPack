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
        print_stack
    "#;

    println!("Program = {}", program);

    let mut script = Script::new(program.bytes());
    script.dictionary().native("print", print);
    script.dictionary().native("print_stack", print_stack);
    script.run();
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
