use runpack::{Script, Cell, Stack};

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
        'Andreu' hola #symbol 10 99.11
        'This is a \'string\' \\my \\ \ \friend'
        print_stack
        hola
        ( 10 -0.444 ( 'hola amic' print_stack ) true )
        print_stack
    "#;

    println!("Program = {}", program);

    let mut script = Script::new(program.bytes());
    script.dictionary().define("hola", hola);
    script.dictionary().define("print_stack", print_stack);
    script.run();
}

fn hola(stack: &mut Stack) {
    if let Some(Cell::String(name)) = stack.pop() {
        println!("Hola {}!", name);
    }
    else {
        panic!("hola: couldn't get a string from stack");
    }
}

fn print_stack(stack: &mut Stack) {
    println!("{:?}", stack);
}
