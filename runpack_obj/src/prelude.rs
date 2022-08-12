pub const PRELUDE: &str = r#"
    ? new 'k v k v ... N -> obj' 'Create object with all pairs of key-value taken from the stack: ( \'name\' \'Andreu\' \'age\' 38 new ) def my_obj'
    ? get '' ''
    ? set '' ''
    ? vec '' ''
    ? key? '' ''
    ? len '' ''

    ? : 'a -> ' 'Get word "a" from the stack and a word "w" from the concat and run "w" from "a": @ my_obj : my_word'
    { @@ swap get exe } def :

    ? . 'a -> ' 'Get word "a" from the stack and a word "w" from the concat and run "w" from "a", passing the object in the stack: @ my_obj . my_word'
    { dup @@ swap get exe } def .

    ? fn 'key obj_ref -> ?' 'Run block or word identified as key from an object'
    { get exe } def fn

    ? md 'key obj_ref -> ?' 'Run block or word identified as key from an object, passing the object ref in the stack.'
    { [ obj key | obj key obj ] get exe } def md
"#;