extern crate alloc;

use super::core::{Pack, Cell, Object, DictEntry, BlockRef, Stack, IntegerType, FloatType, Error};
use hashbrown::HashMap;
use alloc::{string::String, format};

enum PrimitiveErr {
    StackUnderflow = 100,
    NoClosingBlock,
    NoArgsStack,
    NoArgsConcat,
    NoArgsStackConcat,
    WrongType,
    NotFound,
}

impl Into<u16> for PrimitiveErr {
    fn into(self) -> u16 {
        self as u16
    }
}

pub fn register_primitives(pack: &mut Pack) {
    pack.def_natives(&[
        ("(", open_parenth), (")", close_parenth), ("{", open_curly), ("}", close_curly), ("lex", lex), ("def", def), ("@", at),
        ("+", plus), ("-", minus), ("*", star), ("/", slash), ("%", percent), (">", bigger), ("<", smaller), ("=", equal),
        ("!=", not_equal), (">=", big_equal), ("<=", small_equal), ("and", and), ("or", or), ("not", not), ("if", if_word),
        ("ifelse", ifelse_word), ("while", while_word), ("[", open_bracket), ("new", new_word), ("vec", vec_word), ("set", set_word), ("get", get_word),
        ("key?", key_word), ("len", len_word), (":", colon), (".", period), ("exe", exe), ("int", int), ("float", float), ("type", type_word), ("size", size),
    ]);
}

fn open_parenth(pack: &mut Pack) -> Result<bool, Error> {
    pack.stack.start_stack();
    Ok(true)
}

fn close_parenth(pack: &mut Pack) -> Result<bool, Error> {
    if let None = pack.stack.end_stack() {
        Err(Error::new("close_parenth: Stack level undeflow".into(), PrimitiveErr::StackUnderflow.into()))
    }
    else {
        Ok(true)
    }
}

fn open_curly(pack: &mut Pack) -> Result<bool, Error> {
    let pos = pack.concat.pointer;
    let mut level = 1;
    loop {
        if let Some(cell) = pack.concat.next() {
            if let Cell::Word(w) = cell {
                if w == "}" {
                    level -= 1;
                    if level == 0 {
                        let len = pack.concat.pointer - pos;
                        pack.stack.push(Cell::Block(BlockRef { pos, len }));
                        break;
                    }
                }
                else if w == "{" {
                    level += 1;
                }
            }
        }
        else {
            return Err(Error::new("open_curly: Reached the end and didn't find a closing block".into(), PrimitiveErr::NoClosingBlock.into()));
        }
    }
    Ok(true)
}

fn close_curly(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(pos) = pack.ret.pop() {
        pack.concat.pointer = pos;
        Ok(true)
    }
    else {
        Err(Error::new("close_curly: Return stack underflow".into(), PrimitiveErr::StackUnderflow.into()))
    }
}

fn lex(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::String(lex_name)) = pack.concat.next() {
        pack.dictionary.lex = lex_name.clone();
        Ok(true)
    }
    else {
        return Err(Error::new("lex: couldn't find string".into(), PrimitiveErr::NoArgsConcat.into()));
    }
}

fn def(pack: &mut Pack) -> Result<bool, Error> {
    let (data, word) = (pack.stack.pop(), pack.concat.next());
    if let Some(Cell::Word(word)) = word {
        if let Some(Cell::Block(block)) = data {
            pack.dictionary.block(word, block);
        }
        else if let Some(cell) = data {
            pack.dictionary.data(word, cell);
        }
        else {
            return Err(Error::new("def: Expecting a block or a cell".into(), PrimitiveErr::WrongType.into()));
        }
    }
    else {
        return Err(Error::new("def: Expecting a word in the Concat".into(), PrimitiveErr::NoArgsStackConcat.into()));
    }
    Ok(true)
}

fn at(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Word(w)) = pack.concat.next() {
        pack.stack.push(Cell::Word(w.clone()));
        Ok(true)
    }
    else {
        return Err(Error::new("at: Expecting a word in the Concat".into(), PrimitiveErr::NoArgsConcat.into()));
    }
}

fn two_num_op(stack: &mut Stack, int_op: fn(IntegerType, IntegerType) -> IntegerType, flt_op: fn(FloatType, FloatType) -> FloatType) -> Result<bool, Error> {
    let (cell_b, cell_a) = (stack.pop(), stack.pop());
    if let (Some(Cell::Integer(int_a)), Some(Cell::Integer(int_b))) = (&cell_a, &cell_b) {
        stack.push(Cell::Integer(int_op(*int_a, *int_b)));
    }
    else if let (Some(Cell::Float(flt_a)), Some(Cell::Float(flt_b))) = (&cell_a, &cell_b) {
        stack.push(Cell::Float(flt_op(*flt_a, *flt_b)));
    }
    else {
        return Err(Error::new("two_num_op: Expecting two numbers of the same type".into(), PrimitiveErr::NoArgsStack.into()));
    }
    Ok(true)
}

fn two_num_or_str_op(stack: &mut Stack, int_op: fn(IntegerType, IntegerType) -> IntegerType, flt_op: fn(FloatType, FloatType) -> FloatType, str_op: fn(&String, &String) -> String) -> Result<bool, Error> {
    let (cell_b, cell_a) = (stack.pop(), stack.pop());
    if let (Some(Cell::Integer(int_a)), Some(Cell::Integer(int_b))) = (&cell_a, &cell_b) {
        stack.push(Cell::Integer(int_op(*int_a, *int_b)));
    }
    else if let (Some(Cell::Float(flt_a)), Some(Cell::Float(flt_b))) = (&cell_a, &cell_b) {
        stack.push(Cell::Float(flt_op(*flt_a, *flt_b)));
    }
    else if let (Some(Cell::String(str_a)), Some(Cell::String(str_b))) = (&cell_a, &cell_b) {
        stack.push(Cell::String(str_op(str_a, str_b)));
    }
    else {
        return Err(Error::new("two_num_op: Expecting two numbers of the same type".into(), PrimitiveErr::NoArgsStack.into()));
    }
    Ok(true)
}

fn plus(pack: &mut Pack) -> Result<bool, Error> {
    two_num_or_str_op(&mut pack.stack, |a, b| a + b, |a, b| a + b, |a, b| a.clone() + &b)
}

fn minus(pack: &mut Pack) -> Result<bool, Error> {
    two_num_op(&mut pack.stack, |a, b| a - b, |a, b| a - b)
}

fn star(pack: &mut Pack) -> Result<bool, Error> {
    two_num_op(&mut pack.stack, |a, b| a * b, |a, b| a * b)
}

fn slash(pack: &mut Pack) -> Result<bool, Error> {
    two_num_op(&mut pack.stack, |a, b| a / b, |a, b| a / b)
}

fn percent(pack: &mut Pack) -> Result<bool, Error> {
    two_num_op(&mut pack.stack, |a, b| a % b, |a, b| a % b)
}

fn two_cell_cmp(pack: &mut Pack, op: fn(Cell, Cell) -> bool) -> Result<bool, Error> {
    if let (Some(cell_b), Some(cell_a)) = (pack.stack.pop(), pack.stack.pop()) {
        pack.stack.push(Cell::Boolean(op(cell_a, cell_b)));
        Ok(true)
    }
    else {
        Err(Error::new("two_cell_cmp: Couldn't get two cells".into(), PrimitiveErr::NoArgsStack.into()))
    }
}

fn bigger(pack: &mut Pack) -> Result<bool, Error> {
    two_cell_cmp(pack, |a,b| a > b)
}

fn smaller(pack: &mut Pack) -> Result<bool, Error> {
    two_cell_cmp(pack, |a,b| a < b)
}

fn equal(pack: &mut Pack) -> Result<bool, Error> {
    two_cell_cmp(pack, |a,b| a == b)
}

fn not_equal(pack: &mut Pack) -> Result<bool, Error> {
    two_cell_cmp(pack, |a,b| a != b)
}

fn big_equal(pack: &mut Pack) -> Result<bool, Error> {
    two_cell_cmp(pack, |a,b| a >= b)
}

fn small_equal(pack: &mut Pack) -> Result<bool, Error> {
    two_cell_cmp(pack, |a,b| a <= b)
}

fn two_logic_op(stack: &mut Stack, op_bool: fn(bool, bool) -> bool, op_int: fn(IntegerType, IntegerType) -> IntegerType) -> Result<bool, Error> {
    let (cell_b, cell_a) = (stack.pop(), stack.pop());
    if let (Some(Cell::Boolean(bool_a)), Some(Cell::Boolean(bool_b))) = (&cell_a, &cell_b) {
        stack.push(Cell::Boolean(op_bool(*bool_a, *bool_b)));
    }
    else if let (Some(Cell::Integer(int_a)), Some(Cell::Integer(int_b))) = (&cell_a, &cell_b) {
        stack.push(Cell::Integer(op_int(*int_a, *int_b)));
    }
    else {
        return Err(Error::new("two_logic_op: Expecting two booleans or two integers".into(), PrimitiveErr::NoArgsStack.into()));
    }
    Ok(true)
}

fn and(pack: &mut Pack) -> Result<bool, Error> {
    two_logic_op(&mut pack.stack, |a, b| a & b, |a, b| a & b)
}

fn or(pack: &mut Pack) -> Result<bool, Error> {
    two_logic_op(&mut pack.stack, |a, b| a | b, |a, b| a | b)
}

fn not(pack: &mut Pack) -> Result<bool, Error> {
    let cell = pack.stack.pop();
    if let Some(Cell::Boolean(a)) = cell {
        pack.stack.push(Cell::Boolean(!a));
    }
    else if let Some(Cell::Integer(a)) = cell {
        pack.stack.push(Cell::Integer(!a));
    }
    else {
        return Err(Error::new("not: Expecting a boolean or an integer".into(), PrimitiveErr::NoArgsStack.into()));
    }
    Ok(true)
}

fn if_word(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Block(blk)), Some(Cell::Boolean(cond))) = (pack.stack.pop(), pack.stack.pop()) {
        if cond {
            pack.run_block(&blk)?;
        }
    }
    else {
        return Err(Error::new("ifelse: couldn't find condition and 1 block".into(), PrimitiveErr::NoArgsStack.into()));
    }
    Ok(true)
}

fn ifelse_word(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Block(false_blk)), Some(Cell::Block(true_blk)), Some(Cell::Boolean(cond))) = (pack.stack.pop(), pack.stack.pop(), pack.stack.pop()) {
        if cond {
            pack.run_block(&true_blk)
        }
        else {
            pack.run_block(&false_blk)
        }
    }
    else {
        Err(Error::new("ifelse: couldn't find condition and 2 blocks".into(), PrimitiveErr::NoArgsStack.into()))
    }
}

fn while_word(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Block(loop_blk)), Some(Cell::Block(cond_blk))) = (pack.stack.pop(), pack.stack.pop()) {
        loop {
            pack.run_block(&cond_blk)?;
            if let Some(Cell::Boolean(cond)) = pack.stack.pop() {
                if cond {
                    pack.run_block(&loop_blk)?;
                }
                else {
                    break;
                }
            }
            else {
                return Err(Error::new("while: condition didn't produce a bool".into(), PrimitiveErr::WrongType.into()));
            }
        }
    }
    else {
        return Err(Error::new("while: couldn't find 2 blocks".into(), PrimitiveErr::NoArgsStack.into()));
    }
    Ok(true)
}

fn open_bracket(pack: &mut Pack) -> Result<bool, Error> {
    let mut vars: HashMap<String, Cell> = HashMap::with_capacity(16);
    while let Some(Cell::Word(w)) = pack.concat.next() {
        if w == "|" {
            break;
        }
        else {
            if let Some(cell) = pack.stack.pop() {
                vars.insert(w.clone(), cell);
            }
            else {
                return Err(Error::new("open_bracket: stack is empty".into(), PrimitiveErr::NoArgsStack.into()));
            }
        }
    }
    while let Some(Cell::Word(w)) = pack.concat.next() {
        if w == "]" {
            break;
        }
        else {
            if let Some(k) = vars.get(w) {
                pack.stack.push(k.clone());
            }
            else {
                return Err(Error::new("open_bracket: Couldn't find variable name".into(), PrimitiveErr::NoArgsStack.into()));
            }
        }
    }
    Ok(true)
}

fn new_word(pack: &mut Pack) -> Result<bool, Error> {
    if pack.stack.size() % 2 == 0 {
        let mut obj = Object::default();
        while let (Some(val), Some(key)) = (pack.stack.pop(), pack.stack.pop()) {
            obj.map.insert(key, val);
        }
        pack.stack.push(Cell::Object(obj));
    }
    else {
        return Err(Error::new("new: Stack must contain key-value pairs".into(), PrimitiveErr::NoArgsStack.into()));
    }
    Ok(true)
}

fn vec_word(pack: &mut Pack) -> Result<bool, Error> {
    let mut obj = Object::default();
    let mut size = pack.stack.size();
    while let Some(val) = pack.stack.pop() {
        obj.map.insert(Cell::Integer((size - 1) as IntegerType), val);
        size -= 1;
    }
    pack.stack.push(Cell::Object(obj));
    Ok(true)
}

fn set_word(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Word(w)), Some(val), Some(key)) = (pack.stack.pop(), pack.stack.pop(), pack.stack.pop()) {
        if let Some(DictEntry::Data(Cell::Object(obj))) = pack.dictionary.dict.get_mut(&w) {
            obj.map.insert(key, val);
        }
        else {
            return Err(Error::new(format!("set: dictionary doesn't contain an Object for word '{}'", w), PrimitiveErr::WrongType.into()));
        }
    }
    else {
        return Err(Error::new("set: Couldn't get a key-value pair and a word".into(), PrimitiveErr::NoArgsStack.into()));
    }
    Ok(true)
}

fn get_word(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Word(w)), Some(key)) = (pack.stack.pop(), pack.stack.pop()) {
        if let Some(DictEntry::Data(Cell::Object(obj))) = pack.dictionary.dict.get(&w) {
            if let Some(val) = obj.map.get(&key) {
                pack.stack.push(val.clone());
            }
            else {
                return Err(Error::new("get: key doesn't exist in object".into(), PrimitiveErr::NotFound.into()));
            }
        }
        else {
            return Err(Error::new(format!("get: dictionary doesn't contain an Object for word '{}'", w), PrimitiveErr::WrongType.into()));
        }
    }
    else {
        return Err(Error::new("get: Couldn't get a value and a word".into(), PrimitiveErr::NoArgsStack.into()));
    }
    Ok(true)
}

fn key_word(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Word(w)), Some(key)) = (pack.stack.pop(), pack.stack.pop()) {
        if let Some(DictEntry::Data(Cell::Object(obj))) = pack.dictionary.dict.get(&w) {
            pack.stack.push(Cell::Boolean(obj.map.contains_key(&key)));
        }
        else {
            return Err(Error::new(format!("key: dictionary doesn't contain an Object for word '{}'", w), PrimitiveErr::WrongType.into()));
        }
    }
    else {
        return Err(Error::new("key: Couldn't get a value and a word".into(), PrimitiveErr::NoArgsStack.into()));
    }
    Ok(true)
}

fn len_word(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Word(w)) = pack.stack.pop() {
        if let Some(DictEntry::Data(Cell::Object(obj))) = pack.dictionary.dict.get(&w) {
            pack.stack.push(Cell::Integer(obj.map.len() as IntegerType));
        }
        else {
            return Err(Error::new(format!("key: dictionary doesn't contain an Object for word '{}'", w), PrimitiveErr::WrongType.into()));
        }
    }
    else {
        return Err(Error::new("key: Couldn't get a value and a word".into(), PrimitiveErr::NoArgsStack.into()));
    }
    Ok(true)
}

fn colon(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Word(w)), Some(key)) = (pack.stack.pop(), pack.concat.next()) {
        if let Some(DictEntry::Data(Cell::Object(obj))) = pack.dictionary.dict.get(&w) {
            if let Some(val) = obj.map.get(&key) {
                match val {
                    Cell::Integer(_) | Cell::Float(_) | Cell::Boolean(_) | Cell::String(_) | Cell::Object(_) => pack.stack.push(val.clone()),
                    Cell::Word(word) => {
                        let word = word.clone();
                        return pack.exec(&word)
                    },
                    Cell::Block(blk) => {
                        let blk = blk.clone();
                        return pack.run_block(&blk);
                    },
                    Cell::Empty => return Err(Error::new("period: cell is empty".into(), PrimitiveErr::WrongType.into())),
                }
            }
            else {
                return Err(Error::new("period: key doesn't exist in object".into(), PrimitiveErr::NotFound.into()));
            }
        }
        else {
            return Err(Error::new(format!("period: dictionary doesn't contain an Object for word '{}'", w), PrimitiveErr::WrongType.into()));
        }
    }
    else {
        return Err(Error::new("period: Couldn't get an object and a key".into(), PrimitiveErr::NoArgsStackConcat.into()));
    }
    Ok(true)
}

fn period(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(cell) = pack.stack.pop() {
        pack.stack.push(cell.clone());
        pack.stack.push(cell);
        colon(pack)
    }
    else {
        return Err(Error::new("period: Couldn't get a cell".into(), PrimitiveErr::NoArgsStack.into()));
    }
}

/*
PROPOSAL: Create a word (&) to exec a key from an object in the stack, popping it before exec, and then pushing it again after exec:
    (
        @ + { & val_a, & val_b, drop + }
        @ val_a 10
        @ val_b 20
        new
    ) def my_obj

    @ my_obj . + print
Will this really simplify things?
 */

//TODO: map: traverse object key by key

fn exe(pack: &mut Pack) -> Result<bool, Error> {
    match pack.stack.pop() {
        Some(Cell::Block(blk)) => pack.run_block(&blk),
        Some(Cell::Word(w)) => pack.exec(&w),
        _ => Err(Error::new("exe: Couldn't get a word".into(), PrimitiveErr::NoArgsStack.into())),
    }
}

fn int(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Float(f)) = pack.stack.pop() {
        pack.stack.push(Cell::Integer(f as IntegerType));
        Ok(true)
    }
    else {
        Err(Error::new("int: Coulnd't get a float".into(), PrimitiveErr::NoArgsStack.into()))
    }
}

fn float(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Integer(i)) = pack.stack.pop() {
        pack.stack.push(Cell::Float(i as FloatType));
        Ok(true)
    }
    else {
        Err(Error::new("int: Coulnd't get an int".into(), PrimitiveErr::NoArgsStack.into()))
    }
}

fn type_word(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(cell) = pack.stack.get(0) {
        let type_str = match cell {
            Cell::Empty => "empty",
            Cell::Integer(_) => "integer",
            Cell::Float(_) => "float",
            Cell::Boolean(_) => "boolean",
            Cell::String(_) => "string",
            Cell::Word(_) => "word",
            Cell::Block(_) => "block",
            Cell::Object(_) => "object",
        };
        pack.stack.push(Cell::String(type_str.into()));
        Ok(true)
    }
    else {
        Err(Error::new("type: Stack is empty".into(), PrimitiveErr::NoArgsStack.into()))
    }
}

fn size(pack: &mut Pack) -> Result<bool, Error> {
    pack.stack.push(Cell::Integer(pack.stack.size() as IntegerType));
    Ok(true)
}