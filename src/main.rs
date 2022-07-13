use runpack::{
    Concat
};

fn main() {
    println!("Run Pack!");

    let program = r#"
        10 hello #symbol 99.11
        'This is a \'string\' my friend'
        false
    "#;

    println!("Program = {}", program);

    let mut concat = Concat::new(program.bytes());

    concat.tokenize();
}