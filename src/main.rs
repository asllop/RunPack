use runpack;

fn main() {
    println!("Run Pack!");

    let program = r#"
        10 hello #symbol 99.11
        'This is a \'string\' \\my \\ \ \friend'
        ( false true ( 1,2,3,4 ) 'Hi!' )
        { 1 + } def inc
        #twice { 2 * } def#
    "#;

    println!("Program = {}", program);

    let mut script = runpack::Script::new(program.bytes());

    script.tokenize();
}