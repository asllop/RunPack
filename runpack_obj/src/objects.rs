use runpack::{Pack, Cell, Object, DictEntry, IntegerType, Error};
use crate::prelude::PRELUDE;
use alloc::format;

//TODO: traverse object:
//  @ my_obj { print print '------' print } for
//  Execute the block for each key-val of my_obj, passing the key and val in the stack.

//TODO: remove key from object

//TODO: push and pop cells into a vector

/// Register words and prelude.
pub fn register(pack: &mut Pack) {
    pack.code(PRELUDE);
    pack.def_natives(&[
        ("new", new_word), ("vec", vec_word), ("set", set_word), ("get", get_word), ("key?", key_word), ("len", len_word),
    ]);
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
        return Err(Error::new("new: Stack must contain key-value pairs".into()));
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
            return Err(Error::new(format!("set: dictionary doesn't contain an Object for word '{}'", w)));
        }
    }
    else {
        return Err(Error::new("set: Couldn't get a key-value pair and a word".into()));
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
                return Err(Error::new("get: key doesn't exist in object".into()));
            }
        }
        else {
            return Err(Error::new(format!("get: dictionary doesn't contain an Object for word '{}'", w)));
        }
    }
    else {
        return Err(Error::new("get: Couldn't get a value and a word".into()));
    }
    Ok(true)
}

fn key_word(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Word(w)), Some(key)) = (pack.stack.pop(), pack.stack.pop()) {
        if let Some(DictEntry::Data(Cell::Object(obj))) = pack.dictionary.dict.get(&w) {
            pack.stack.push(Cell::Boolean(obj.map.contains_key(&key)));
        }
        else {
            return Err(Error::new(format!("key: dictionary doesn't contain an Object for word '{}'", w)));
        }
    }
    else {
        return Err(Error::new("key: Couldn't get a value and a word".into()));
    }
    Ok(true)
}

fn len_word(pack: &mut Pack) -> Result<bool, Error> {
    if let Some(Cell::Word(w)) = pack.stack.pop() {
        if let Some(DictEntry::Data(Cell::Object(obj))) = pack.dictionary.dict.get(&w) {
            pack.stack.push(Cell::Integer(obj.map.len() as IntegerType));
        }
        else {
            return Err(Error::new(format!("key: dictionary doesn't contain an Object for word '{}'", w)));
        }
    }
    else {
        return Err(Error::new("key: Couldn't get a value and a word".into()));
    }
    Ok(true)
}