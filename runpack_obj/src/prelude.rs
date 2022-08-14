pub const PRELUDE: &str = r#"
    ? new 'k0 v0 k1 v1 ... N -> obj' 'Create object with all pairs of key-value taken from the stack: ( \'name\' \'Andreu\' \'age\' 38 new ) def my_obj'
    ? get 'key word -> val' 'Get value from "key" in the object stored at "word": \'name\' @ my_obj get'
    ? set 'key value word -> ' 'Set value for key in the object stored at "word": \'age\' 39 @ my_obj set'
    ? vec 'v0 v1 v2 ... N -> obj' 'Create vector with values taken from the stack: ( 10 20 30 vec ) def nums , 0 @ nums get print'
    ? key? 'key word -> bool' 'Check if key exist in object: \'age\' @ my_obj key?'
    ? len 'word -> n' 'Get size of an object: @ my_obj len'
    ? foreach 'word block -> ?' 'Traverse object at "word" and execute "block" passing each key and value in the stack: @ my_obj { print print } foreach'
    ? rem 'key word -> ' 'Remove key from object: \'my_key\' @ my_obj rem'

    ? : 'a -> ' 'Get word "a" from the stack and a word "w" from the concat and run "w" from "a": @ my_obj : my_word'
    { @@ swap get exe } def :

    ? . 'a -> ' 'Get word "a" from the stack and a word "w" from the concat and run "w" from "a", passing the object in the stack: @ my_obj . my_word'
    { dup @@ swap get exe } def .

    ? fn 'key word -> ?' 'Run block or word identified as key from an object at "word": @ foo @ my_obj fn'
    { get exe } def fn

    ? md 'key word -> ?' 'Run block or word identified as key from an object at "word", passing the object ref in the stack: @ foo @ my_obj md'
    { [ obj key | obj key obj ] get exe } def md
"#;