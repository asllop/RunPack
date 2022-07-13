use runpack::{
    Concat
};

fn main() {
    println!("Run Pack!");

    let mut concat = Concat::new(r#"
        10 hello #symbol 99.11
        'This is a \'string\' my friend'
        false
    "#.bytes());

    concat.tokenize();
}