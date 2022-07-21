pub const PRELUDE: &str = r#"
    "--- Stack ---"
    "drop a -> "
    { [ a : ] } def drop
    "dup a -> a,a"
    { [ a : a a ] } def dup
    "swap a,b -> b,a"
    { [ a b : a b ] } def swap
    "flush a,b,c,..,N -> "
    { { size 0 > } { drop } while } def flush

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
    "++ a -> a+1"
    { is_int? { 1 } { 1.0 } ifelse + } def ++
    "-- a -> a-1"
    { is_int? { 1 } { 1.0 } ifelse - } def --
    "fract a.x -> 0.x"
    { dup int float - } def fract
    "sum a,b,c,..,N -> a+b+c.."
    { { size 1 > } { + } while } def sum
    "sub a,b,c,..,N -> a-b-c.."
    { { size 1 > } { - } while } def sub
    "prod a,b,c,..,N -> a*b*c.."
    { { size 1 > } { * } while } def prod
    "div a,b,c,..,N -> a/b/c.."
    { { size 1 > } { / } while } def div

    "TODO: Vectors"
"#;