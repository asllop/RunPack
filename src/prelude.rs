pub const PRELUDE: &str = r#"
    "Stack"
    { [ a : ] } def drop
    { [ a : a a ] } def dup
    { [ a b : a b ] } def swap
    { { drop } { size 0 > } while } def flush

    "Types"
    { type 'integer' = } def is_int?
    
    "Math"
    { is_int? { 1 + } { 1.0 + } ifelse } def ++
    { is_int? { 1 - } { 1.0 - } ifelse } def --
    { dup int float - } def fract
    { { size 1 > } { + } while } def sum
    { { size 1 > } { - } while } def sub
    { { size 1 > } { * } while } def prod
    { { size 1 > } { / } while } def div
    "TODO: vectors"
"#;