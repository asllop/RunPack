pub const PRELUDE: &str = r#"
    "fn key obj_ref -> ?"
    " Description: Run block or word identified as key from an object."
    { get exe } def fn
    "md key obj_ref -> ?"
    " Description: Run block or word identified as key from an object, passing the object ref in the stack."
    { [ obj key | obj key obj ] get exe } def md
"#;