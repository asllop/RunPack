use runpack::{Script, Cell, Stack, Concat, Dictionary};

fn main() {
    println!("Run Pack!");

    let program = r#"
        'Andreu' hola #symbol 10 99.11
        'This is a \'string\' \\my \\ \ \friend'
        ( false true ( 1,2,3,4 ) 'Hi!' )
        { 1 + } def inc
        #twice { 2 * } def#
    "#;


    let program = r#"
        #symbol 10 99.11 'This is a \'string\' \\my \\ \ \friend'
        print_stack
        print
        '--------------' print
        ( 10 -0.444 ( 'hola amic' print_stack ) true )
        '--------------' print
        print_stack
        { 1 + } def inc
        '--------------' print
        print_stack
        66 inc
        10 20 +
        11.1 0.9 +
        print_stack
    "#;

    println!("Program = {}", program);

    let mut script = Script::new(program.bytes());
    script.dictionary().define("print", print);
    script.dictionary().define("print_stack", print_stack);
    script.run();
}

fn print(stack: &mut Stack, _: &mut Concat, _: &mut Dictionary) {
    if let Some(Cell::String(data)) = stack.pop() {
        println!("{}", data);
    }
    else {
        panic!("prints: couldn't get a string from stack");
    }
}

fn print_stack(stack: &mut Stack, _: &mut Concat, _: &mut Dictionary) {
    println!("{:?}", stack);
}
