pub const PRELUDE: &str = r#"
    "Stack"
    { [ a : ] } def drop
    { [ a : a a ] } def dup
    { [ a b : a b ] } def swap
    { { drop } { size 0 > } while } def flush
    "Math"
    { 1 + } def ++
    { 1 - } def --
    { dup int float - } def fract
    { { + } { size 1 > } while } def sum
    { { - } { size 1 > } while } def sub
    { { * } { size 1 > } while } def prod
    { { / } { size 1 > } while } def div
"#;