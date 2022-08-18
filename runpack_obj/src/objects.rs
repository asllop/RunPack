use runpack::{Pack, Cell, Map, DictEntry, IntegerType, Error, Vector};
use crate::prelude::PRELUDE;
use alloc::{format, vec::Vec};

/// Register words and prelude.
pub fn register(pack: &mut Pack) {
    pack.code(PRELUDE);
    pack.def_natives(&[
        ("map", map_word), ("set", set_word), ("get", get_word), ("key?", key_word), ("len", len_word),
        ("foreach", foreach), ("rem", rem), ("vec", vec_word), 
    ]);
}

fn map_word(pack: &mut Pack) -> Result<bool, Error> {
    if pack.stack.size() % 2 == 0 {
        let mut obj = Map::default();
        while let (Some(val), Some(key)) = (pack.stack.pop(), pack.stack.pop()) {
            obj.map.insert(key, val);
        }
        pack.stack.push(Cell::Map(obj));
    }
    else {
        return Err(Error::new("map: Stack must contain key-value pairs".into()));
    }
    Ok(true)
}

fn set_word(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Word(w)), Some(val), Some(key)) = (pack.stack.pop(), pack.stack.pop(), pack.stack.pop()) {
        if let Some(DictEntry::Data(Cell::Map(obj))) = pack.dictionary.dict.get_mut(&w) {
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
        if let Some(DictEntry::Data(Cell::Map(obj))) = pack.dictionary.dict.get(&w) {
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
        if let Some(DictEntry::Data(Cell::Map(obj))) = pack.dictionary.dict.get(&w) {
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
        if let Some(DictEntry::Data(Cell::Map(obj))) = pack.dictionary.dict.get(&w) {
            pack.stack.push(Cell::Integer(obj.map.len() as IntegerType));
        }
        else {
            return Err(Error::new(format!("len: dictionary doesn't contain an Object for word '{}'", w)));
        }
    }
    else {
        return Err(Error::new("len: Couldn't get a value and a word".into()));
    }
    Ok(true)
}

fn foreach(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Block(blk)), Some(Cell::Word(w))) = (pack.stack.pop(), pack.stack.pop()) {
        if let Some(DictEntry::Data(Cell::Map(obj))) = pack.dictionary.dict.get(&w) {
            let obj = obj.clone();
            for (key, val) in obj.map.into_iter() {
                pack.stack.push(key);
                pack.stack.push(val);
                pack.run_block(&blk)?;
            }
            Ok(true)
        }
        else {
            Err(Error::new(format!("foreach: dictionary doesn't contain an Object for word '{}'", w)))
        }
    }
    else {
        Err(Error::new("foreach: Couldn't get a block and a word".into()))
    }
}

//TODO: put removed key and value into the stack
fn rem(pack: &mut Pack) -> Result<bool, Error> {
    if let (Some(Cell::Word(w)), Some(cell)) = (pack.stack.pop(), pack.stack.pop()) {
        if let Some(DictEntry::Data(Cell::Map(obj))) = pack.dictionary.dict.get_mut(&w) {
            obj.map.remove(&cell);
            Ok(true)
        }
        else {
            Err(Error::new(format!("rem: dictionary doesn't contain an Object for word '{}'", w)))
        }
    }
    else {
        Err(Error::new("rem: Couldn't get a key and a word".into()))
    }
}

fn vec_word(pack: &mut Pack) -> Result<bool, Error> {
    let mut vector = Vec::with_capacity(pack.stack.size());
    unsafe { vector.set_len(pack.stack.size()); }
    let mut v = Vector { vector };
    while let Some(val) = pack.stack.pop() {
        v.vector[pack.stack.size()] = val;
    }
    pack.stack.push(Cell::Vector(v));
    Ok(true)
}

//TODO: push, pop, and insert cells on a vector

//TODO: overload get, set, rem, foreach and len to support vectors

//TODO: create words to work with objects in the stack without cloning it