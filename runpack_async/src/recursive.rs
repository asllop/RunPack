use runpack::{Pack, Cell, Error};

/// Register words and prelude.
pub fn register(pack: &mut Pack) {
    //pack.code(PRELUDE);
    pack.def_natives(&[
        ("break", break_word), ("reenter", reenter),
    ]);
}

fn reenter(pack: &mut Pack) -> Result<bool, runpack::Error> {
    pack.ret.push(pack.concat.pointer - 1);
    Ok(true)
}

//TODO: create word "end" that is like break with 1 level

fn break_word(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let Some(Cell::Integer(level)) = pack.stack.pop() {
        // Discard n positions of the return stack + reenter return stack
        for _ in 0..level + 1 {
            pack.ret.pop();
        }
        if let Some(pos) = pack.ret.pop() {
            pack.concat.pointer = pos;
            Ok(true)
        }
        else {
            Err(Error::new("break_word: Return stack underflow".into()))
        }
    }
    else {
        Err(Error::new("break_word: Couln't find integer in stack".into()))
    }
}