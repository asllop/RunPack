use super::core::{Pack, Cell, BlockRef, Stack, Error};
use hashbrown::HashMap;
use alloc::string::String;

pub fn register_primitives(pack: &mut Pack) {
    pack.def_natives(&[
        ("(", open_parenth), (")", close_parenth), ("size", size), ("{", open_curly), ("}", close_curly), ("lex", lex),
        ("+", plus), ("-", minus), ("*", star), ("/", slash), ("%", percent), (">", bigger), ("<", smaller), ("=", equal),
        ("!=", not_equal), (">=", big_equal), ("<=", small_equal), ("and", and), ("or", or), ("not", not), ("if", if_word),
        ("either", either), ("loop", loop_word), ("[", open_bracket), ("exe", exe), ("int", int), ("float", float),
        ("string", string), ("word", word), ("type", type_word), ("?", question), ("@@", atat), ("@def", atdef), ("lex#", lex_val),
        ("skip", skip), ("block", block), ("exist?", exist_question), ("_", underscore),
    ]);
}

fn open_parenth(pack: &mut Pack) -> Result<bool, Error> {
    pack.stack.start_stack();
    Ok(true)
}

fn close_parenth(pack: &mut Pack) -> Result<bool, Error> {
    if let None = pack.stack.end_stack() {
        Err(Error::new("close_parenth: Stack level undeflow".into()))
    }
    else {
        Ok(true)
    }
}

fn size(pack: &mut Pack) -> Result<bool, Error> {
    pack.stack.push((pack.stack.size() as i64).into());
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
                        pack.stack.push(BlockRef { pos, len }.into());
                        break;
                    }
                }
                else if w == "{" {
                    level += 1;
                }
            }
        }
        else {
            return Err(Error::new("open_curly: Reached the end and didn't find a closing block".into()));
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
        Err(Error::new("close_curly: Return stack underflow".into()))
    }
}

/* TODO:
    - Instead of using a string as an argument, use a word and RunPack will automatically append the dot.
    - Create a word "unlex" or "endlex" or similar, to clear the lexicon mark, instead of the current unelegant approach os usign an empty string.
*/

fn lex(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::String(lex_name)) = pack.concat.next() {
        pack.dictionary.lex = lex_name.clone();
        Ok(true)
    }
    else {
        return Err(Error::new("lex: couldn't find string".into()));
    }
}

fn two_num_op(stack: &mut Stack, int_op: fn(i64, i64) -> i64, flt_op: fn(f64, f64) -> f64) -> Result<bool, Error> {
    let (cell_b, cell_a) = (stack.pop(), stack.pop());
    if let (Some(Cell::Integer(int_a)), Some(Cell::Integer(int_b))) = (&cell_a, &cell_b) {
        stack.push(Cell::Integer(int_op(*int_a, *int_b)));
    }
    else if let (Some(Cell::Float(flt_a)), Some(Cell::Float(flt_b))) = (&cell_a, &cell_b) {
        stack.push(flt_op(*flt_a, *flt_b).into());
    }
    else {
        return Err(Error::new("two_num_op: Expecting two numbers of the same type".into()));
    }
    Ok(true)
}

fn two_num_or_str_op(stack: &mut Stack, int_op: fn(i64, i64) -> i64, flt_op: fn(f64, f64) -> f64, str_op: fn(&String, &String) -> String) -> Result<bool, Error> {
    let (cell_b, cell_a) = (stack.pop(), stack.pop());
    if let (Some(Cell::Integer(int_a)), Some(Cell::Integer(int_b))) = (&cell_a, &cell_b) {
        stack.push(int_op(*int_a, *int_b).into());
    }
    else if let (Some(Cell::Float(flt_a)), Some(Cell::Float(flt_b))) = (&cell_a, &cell_b) {
        stack.push(flt_op(*flt_a, *flt_b).into());
    }
    else if let (Some(Cell::String(str_a)), Some(Cell::String(str_b))) = (&cell_a, &cell_b) {
        stack.push(str_op(str_a, str_b).into());
    }
    else {
        return Err(Error::new("two_num_or_str_op: Expecting two cells of the same type".into()));
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
    two_num_op(&mut pack.stack, |a, b| if b != 0 { a / b } else { 0 }, |a, b| if b != 0.0 { a / b } else { 0.0 })
}

fn percent(pack: &mut Pack) -> Result<bool, Error> {
    two_num_op(&mut pack.stack, |a, b| if b != 0 { a % b } else { 0 }, |a, b| if b != 0.0 { a % b } else { 0.0 })
}

fn two_cell_cmp(pack: &mut Pack, op: fn(Cell, Cell) -> bool) -> Result<bool, Error> {
    if let (Some(cell_b), Some(cell_a)) = (pack.stack.pop(), pack.stack.pop()) {
        pack.stack.push(op(cell_a, cell_b).into());
        Ok(true)
    }
    else {
        Err(Error::new("two_cell_cmp: Couldn't get two cells".into()))
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

fn two_logic_op(stack: &mut Stack, op_bool: fn(bool, bool) -> bool, op_int: fn(i64, i64) -> i64) -> Result<bool, Error> {
    let (cell_b, cell_a) = (stack.pop(), stack.pop());
    if let (Some(Cell::Boolean(bool_a)), Some(Cell::Boolean(bool_b))) = (&cell_a, &cell_b) {
        stack.push(op_bool(*bool_a, *bool_b).into());
    }
    else if let (Some(Cell::Integer(int_a)), Some(Cell::Integer(int_b))) = (&cell_a, &cell_b) {
        stack.push(op_int(*int_a, *int_b).into());
    }
    else {
        return Err(Error::new("two_logic_op: Expecting two booleans or two integers".into()));
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
        pack.stack.push((!a).into());
    }
    else if let Some(Cell::Integer(a)) = cell {
        pack.stack.push((!a).into());
    }
    else {
        return Err(Error::new("not: Expecting a boolean or an integer".into()));
    }
    Ok(true)
}

//TODO: rework if to be a fast version of either. It takes 1 condition from the stack and 2 words from the concat:
//      10 2 > if do_true do_false
// In case one of the two conditions must be ignored, just put a _ that is like a NOP:
//      10 2 > if do_true _
fn if_word(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Block(blk)), Some(Cell::Boolean(cond))) = (pack.stack.pop(), pack.stack.pop()) {
        if cond {
            pack.ret.push(pack.concat.pointer);
            pack.concat.pointer = blk.pos;
        }
    }
    else {
        return Err(Error::new("if: couldn't find condition and 1 block".into()));
    }
    Ok(true)
}

fn either(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Block(false_blk)), Some(Cell::Block(true_blk)), Some(Cell::Boolean(cond))) = (pack.stack.pop(), pack.stack.pop(), pack.stack.pop()) {
        if cond {
            pack.ret.push(pack.concat.pointer);
            pack.concat.pointer = true_blk.pos;
        }
        else {
            pack.ret.push(pack.concat.pointer);
            pack.concat.pointer = false_blk.pos;
        }
        Ok(true)
    }
    else {
        Err(Error::new("either: couldn't find condition and 2 blocks".into()))
    }
}

fn loop_word(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Block(loop_blk)), Some(Cell::Block(cond_blk))) = (pack.stack.pop(), pack.stack.pop()) {
        let concat_pointer = pack.concat.pointer;
        loop {
            pack.run_block(&cond_blk)?;
            pack.ret.pop();
            if let Some(Cell::Boolean(cond)) = pack.stack.pop() {
                if cond {
                    pack.run_block(&loop_blk)?;
                    pack.ret.pop();
                }
                else {
                    pack.concat.pointer = concat_pointer;
                    break;
                }
            }
            else {
                return Err(Error::new("loop: condition didn't produce a bool".into()));
            }
        }
    }
    else {
        return Err(Error::new("loop: couldn't find 2 blocks".into()));
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
                return Err(Error::new("open_bracket: stack is empty".into()));
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
                return Err(Error::new("open_bracket: Couldn't find variable name".into()));
            }
        }
    }
    Ok(true)
}

fn exe(pack: &mut Pack) -> Result<bool, Error> {
    match pack.stack.pop() {
        Some(Cell::Block(blk)) => {
            pack.ret.push(pack.concat.pointer);
            pack.concat.pointer = blk.pos;
            Ok(true)
        },
        Some(Cell::Word(w)) => pack.exec(&w),
        Some(cell) => {
            pack.stack.push(cell);
            Ok(true)
        },
        _ => Err(Error::new("exe: Couldn't get a valid cell from the stack".into()))
    }
}

fn int(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Float(f)) = pack.stack.pop() {
        pack.stack.push((f as i64).into());
        Ok(true)
    }
    else {
        Err(Error::new("int: Coulnd't get a float".into()))
    }
}

fn float(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Integer(i)) = pack.stack.pop() {
        pack.stack.push((i as f64).into());
        Ok(true)
    }
    else {
        Err(Error::new("int: Coulnd't get an int".into()))
    }
}

fn string(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Word(w)) = pack.stack.pop() {
        pack.stack.push(w.into());
        Ok(true)
    }
    else {
        Err(Error::new("string: Coulnd't get a word".into()))
    }
}

fn word(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::String(s)) = pack.stack.pop() {
        pack.stack.push(Cell::Word(s));
        Ok(true)
    }
    else {
        Err(Error::new("word: Coulnd't get a string".into()))
    }
}

fn type_word(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(cell) = pack.stack.get(0) {
        let type_str = match cell {
            Cell::Integer(_) => "integer",
            Cell::Float(_) => "float",
            Cell::Boolean(_) => "boolean",
            Cell::String(_) => "string",
            Cell::Word(_) => "word",
            Cell::Block(_) => "block",
            Cell::Struct(_) => "struct",
        };
        pack.stack.push(type_str.into());
        Ok(true)
    }
    else {
        Err(Error::new("type: Stack is empty".into()))
    }
}

fn question(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Word(word)), Some(Cell::String(stack_effect)), Some(Cell::String(description))) = (pack.concat.next_clone(), pack.concat.next_clone(), pack.concat.next_clone()) {
        if pack.dictionary.dict.contains_key("?__") {
            let stack_help_word = format!("?_{word}_stack_");
            let desc_help_word = format!("?_{word}_desc_");
            pack.dictionary.data(&stack_help_word, stack_effect.into());
            pack.dictionary.data(&desc_help_word, description.into());
        }
        Ok(true)
    }
    else {
        Err(Error::new("question: No correct arguments in the concat".into()))
    }
}

fn atat(pack: &mut Pack) -> Result<bool, Error>  {
    if let Some(parent_concat_pos) = pack.ret.pop() {
        if let Some(cell) = pack.concat.array.get(parent_concat_pos) {
            pack.ret.push(parent_concat_pos + 1);
            pack.stack.push(cell.clone());
            Ok(true)
        }
        else {
            Err(Error::new("atat: couldn't get a cell from the concat".into()))
        }
    }
    else {
        Err(Error::new("atat: couldn't get ret pos".into()))
    }
}

fn atdef(pack: &mut Pack) -> Result<bool, Error> {
    let (word, data) = (pack.stack.pop(), pack.stack.pop());
    if let Some(Cell::Word(word)) = word {
        if let Some(Cell::Block(block)) = data {
            pack.dictionary.block(&word, block);
        }
        else if let Some(cell) = data {
            pack.dictionary.data(&word, cell);
        }
        else {
            return Err(Error::new("atdef: Expecting a block or a cell".into()));
        }
    }
    else {
        return Err(Error::new("atdef: Expecting a word in the stack".into()));
    }
    Ok(true)
}

fn skip(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Integer(offset)) = pack.stack.pop() {
        pack.concat.pointer = pack.concat.pointer.wrapping_add(offset as usize);
        Ok(true)
    }
    else {
        Err(Error::new("skip: Expecting a integer in the stack".into()))
    }
}

fn block(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Block(block)) = pack.stack.pop() {
        // New block will start at the end of current concat + 3 ("N skip {").
        let new_block_pos = pack.concat.array.len() + 3;
        let new_block_len = block.len;
        // Add skip and {
        pack.concat.array.push((new_block_len as i64 + 1).into());
        pack.concat.array.push(Cell::Word("skip".into()));
        pack.concat.array.push(Cell::Word("{".into()));
        // Copy the block to the end of the concat
        for n in block.pos..(block.pos + block.len) {
            // Substitute any $ word with the cell in the stack
            if let Cell::Word(w) = &pack.concat.array[n] {
                if w == "$" {
                    if let Some(cell) = pack.stack.pop() {
                        pack.concat.array.push(cell);
                        continue;
                    }
                    else {
                        return Err(Error::new("block: Couldn't get cell from stack".into()))
                    }
                }
            }
            pack.concat.array.push(pack.concat.array[n].clone());
        }
        // Return the new block in the stack
        pack.stack.push(BlockRef { pos: new_block_pos, len: new_block_len }.into());
        Ok(true)
    }
    else {
        Err(Error::new("block: Couldn't get block from stack".into()))
    }
}

fn lex_val(pack: &mut Pack) -> Result<bool, Error> {
    pack.stack.push(pack.dictionary.lex.clone().into());
    Ok(true)
}

fn exist_question(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Word(w)) = pack.stack.pop() {
        let b = pack.dictionary.dict.contains_key(&w);
        pack.stack.push(Cell::Word(w));
        pack.stack.push(b.into());
        Ok(true)
    }
    else {
        Err(Error::new("exist_question: Couldn't get word ref from stack".into()))
    }
}

fn underscore(_: &mut Pack) -> Result<bool, Error> {
    Ok(true)
}