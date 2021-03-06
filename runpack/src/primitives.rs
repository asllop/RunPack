extern crate alloc;

use super::core::{Pack, Cell, BlockRef, Stack, IntegerType, FloatType, Error, ErrCode};
use hashbrown::HashMap;
use alloc::string::String;

pub fn register_primitives(pack: &mut Pack) {
    pack.def_natives(&[
        ("(", open_parenth), (")", close_parenth), ("size", size), ("{", open_curly), ("}", close_curly), ("lex", lex), ("def", def),
        ("@", at), ("+", plus), ("-", minus), ("*", star), ("/", slash), ("%", percent), (">", bigger), ("<", smaller), ("=", equal),
        ("!=", not_equal), (">=", big_equal), ("<=", small_equal), ("and", and), ("or", or), ("not", not), ("if", if_word),
        ("either", either_word), ("loop", loop_word), ("[", open_bracket), ("exe", exe), ("int", int), ("float", float),
        ("type", type_word),
    ]);
}

//TODO: get cell from concat and put into stack, but not from immediate concat, from defined word concat:
//  { @@ ++ } inc_post
//  inc_post 100 print
//The word @@ doesn't get the next cell in the concat, that is "++", it uses concat position of word caller.

//TODO: remove word from dictionary

fn open_parenth(pack: &mut Pack) -> Result<bool, Error> {
    pack.stack.start_stack();
    Ok(true)
}

fn close_parenth(pack: &mut Pack) -> Result<bool, Error> {
    if let None = pack.stack.end_stack() {
        Err(Error::new("close_parenth: Stack level undeflow".into(), ErrCode::StackUnderflow.into()))
    }
    else {
        Ok(true)
    }
}

fn size(pack: &mut Pack) -> Result<bool, Error> {
    pack.stack.push(Cell::Integer(pack.stack.size() as IntegerType));
    Ok(true)
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
            return Err(Error::new("open_curly: Reached the end and didn't find a closing block".into(), ErrCode::NoClosingBlock.into()));
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
        Err(Error::new("close_curly: Return stack underflow".into(), ErrCode::StackUnderflow.into()))
    }
}

fn lex(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::String(lex_name)) = pack.concat.next() {
        pack.dictionary.lex = lex_name.clone();
        Ok(true)
    }
    else {
        return Err(Error::new("lex: couldn't find string".into(), ErrCode::NoArgsConcat.into()));
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
            return Err(Error::new("def: Expecting a block or a cell".into(), ErrCode::WrongType.into()));
        }
    }
    else {
        return Err(Error::new("def: Expecting a word in the Concat".into(), ErrCode::NoArgsStackConcat.into()));
    }
    Ok(true)
}

fn at(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(cell) = pack.concat.next() {
        pack.stack.push(cell.clone());
        Ok(true)
    }
    else {
        return Err(Error::new("at: Expecting a cell in the Concat".into(), ErrCode::NoArgsConcat.into()));
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
        return Err(Error::new("two_num_op: Expecting two numbers of the same type".into(), ErrCode::NoArgsStack.into()));
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
        return Err(Error::new("two_num_op: Expecting two numbers of the same type".into(), ErrCode::NoArgsStack.into()));
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
        Err(Error::new("two_cell_cmp: Couldn't get two cells".into(), ErrCode::NoArgsStack.into()))
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
        return Err(Error::new("two_logic_op: Expecting two booleans or two integers".into(), ErrCode::NoArgsStack.into()));
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
        return Err(Error::new("not: Expecting a boolean or an integer".into(), ErrCode::NoArgsStack.into()));
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
        return Err(Error::new("if: couldn't find condition and 1 block".into(), ErrCode::NoArgsStack.into()));
    }
    Ok(true)
}

fn either_word(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Block(false_blk)), Some(Cell::Block(true_blk)), Some(Cell::Boolean(cond))) = (pack.stack.pop(), pack.stack.pop(), pack.stack.pop()) {
        if cond {
            pack.run_block(&true_blk)
        }
        else {
            pack.run_block(&false_blk)
        }
    }
    else {
        Err(Error::new("either: couldn't find condition and 2 blocks".into(), ErrCode::NoArgsStack.into()))
    }
}

fn loop_word(pack: &mut Pack) -> Result<bool, Error> {
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
                return Err(Error::new("loop: condition didn't produce a bool".into(), ErrCode::WrongType.into()));
            }
        }
    }
    else {
        return Err(Error::new("loop: couldn't find 2 blocks".into(), ErrCode::NoArgsStack.into()));
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
                return Err(Error::new("open_bracket: stack is empty".into(), ErrCode::NoArgsStack.into()));
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
                return Err(Error::new("open_bracket: Couldn't find variable name".into(), ErrCode::NoArgsStack.into()));
            }
        }
    }
    Ok(true)
}

fn exe(pack: &mut Pack) -> Result<bool, Error> {
    match pack.stack.pop() {
        Some(Cell::Block(blk)) => pack.run_block(&blk),
        Some(Cell::Word(w)) => pack.exec(&w),
        _ => Err(Error::new("exe: Couldn't get a word".into(), ErrCode::NoArgsStack.into())),
    }
}

fn int(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Float(f)) = pack.stack.pop() {
        pack.stack.push(Cell::Integer(f as IntegerType));
        Ok(true)
    }
    else {
        Err(Error::new("int: Coulnd't get a float".into(), ErrCode::NoArgsStack.into()))
    }
}

fn float(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Integer(i)) = pack.stack.pop() {
        pack.stack.push(Cell::Float(i as FloatType));
        Ok(true)
    }
    else {
        Err(Error::new("int: Coulnd't get an int".into(), ErrCode::NoArgsStack.into()))
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
        Err(Error::new("type: Stack is empty".into(), ErrCode::NoArgsStack.into()))
    }
}