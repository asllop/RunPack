use runpack::{self, Cell};

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
        print_stack
    "#;

    println!("Program = {}", program);

    let mut script = runpack::Script::new(program.bytes());
    script.dictionary.implement("hola", | stack | {
        if let Some(Cell::String(name)) = stack.pop() {
            println!("Hola {}!", name);
        }
        else {
            panic!("hola: couldn't get a string from stack");
        }
    });
    script.dictionary.implement("print_stack", | stack | {
        println!("{:?}", stack);
    });
    script.run();
}