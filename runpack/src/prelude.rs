pub const PRELUDE: &str = r#"
    "--- Stack ---"
    "drop a -> "
    { [ a | ] } def drop
    "dup a -> a,a"
    { [ a | a a ] } def dup
    "swap a,b -> b,a"
    { [ a b | a b ] } def swap
    "flush a,b,c,..,N -> "
    { { size 0 > } { drop } loop } def flush

    "--- Types ---"
    "is_int? a -> bool"
    { type 'integer' = } def is_int?
    "is_float? a -> bool"
    { type 'float' = } def is_float?
    "is_bool? a -> bool"
    { type 'boolean' = } def is_bool?
    "is_str? a -> bool"
    { type 'string' = } def is_str?
    "is_word? a -> bool"
    { type 'word' = } def is_word?
    "is_block? a -> bool"
    { type 'block' = } def is_block?
    "is_obj? a -> bool"
    { type 'object' = } def is_obj?
    
    "--- Math ---"
    "++ a_i_f -> b_i_f"
    { is_int? { 1 } { 1.0 } either + } def ++
    "-- a_i_f -> b_i_f"
    { is_int? { 1 } { 1.0 } either - } def --
    "fract a_f -> b_f"
    { dup int float - } def fract
    "sum a,b,c,..,N -> z"
    { { size 1 > } { + } loop } def sum
    "sub a,b,c,..,N -> z"
    { { size 1 > } { - } loop } def sub
    "prod a,b,c,..,N -> z"
    { { size 1 > } { * } loop } def prod
    "div a,b,c,..,N -> z"
    { { size 1 > } { / } loop } def div
"#;