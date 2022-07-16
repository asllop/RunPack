#![no_std]

extern crate alloc;

use hashbrown::HashMap;
use alloc::{ vec::Vec, string::String, format };
use core::hash::Hash;

#[derive(Debug)]
/// Error type
pub struct Error {
    pub msg: String,
    pub code: u16
}

impl Error {
    pub fn new(msg: String, code: u16) -> Self {
        Self { msg, code }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Copy, Debug)]
/// Block reference type
pub struct BlockRef {
    pub pos: usize,
    pub len: usize,
}

#[derive(Eq, Clone, Debug)]
/// Custom object type
pub struct Object {
    pub map: HashMap<Cell, Cell>,
    pub kind: String,
}

impl Object {
    pub fn new(kind: &str) -> Self {
        Self { map: HashMap::new(), kind: kind.into()}
    }
}

impl Hash for Object {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.map.iter().for_each(|(k, v)| {
            k.hash(state);
            v.hash(state)
        });
        self.kind.hash(state);
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.map.len() == other.map.len()
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.map.len().partial_cmp(&other.map.len())
    }
}

/// Integer type alias
pub type IntegerType = i64;

/// Float type alias
pub type FloatType = i64;

#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Debug)]
/// Data primitive
pub enum Cell {
    Empty,
    Integer(IntegerType),
    Float(FloatType),
    Boolean(bool),
    Symbol(String),
    String(String),
    Word(String),
    Block(BlockRef),
    Object(Object),
}

impl Cell {
    fn number(token: &str) -> Option<Self> {
        if let Ok(int) = token.parse::<IntegerType>() {
            Some(Cell::Integer(int))
        }
        else if let Ok(flt) = token.parse::<FloatType>() {
            Some(Cell::Float(flt))
        }
        else {
            None
        }
    }

    fn symbol(token: &str) -> Option<Self> {
        if token.starts_with("#") {
            Some(Cell::Symbol(token.into()))
        }
        else {
            None
        }
    }

    fn boolean(token: &str) -> Option<Self> {
        if token == "true" {
            Some(Cell::Boolean(true))
        }
        else if token == "false" {
            Some(Cell::Boolean(false))
        }
        else {
            None
        }
    }
}

enum DictEntry {
    Native(fn(&mut Script) -> Result<bool, Error>),
    Defined(BlockRef),
    Data(Cell),
}

/// Dictionary of words
pub struct Dictionary {
    dict: HashMap<String, DictEntry>,
    pub lex: String,
}

impl Dictionary {
    fn new() -> Self {
        Self { dict: HashMap::new(), lex: String::default() }
    }

    /// Define a native word
    pub fn native(&mut self, word: &str, func: fn(&mut Script) -> Result<bool, Error>) {
        let lex = self.lex.clone();
        self.dict.insert(lex + word, DictEntry::Native(func));
    }

    /// Define block word
    pub fn block(&mut self, word: &str, block: BlockRef) {
        let lex = self.lex.clone();
        self.dict.insert(lex + word, DictEntry::Defined(block));
    }

    /// Define data word
    pub fn data(&mut self, word: &str, cell: Cell) {
        let lex = self.lex.clone();
        self.dict.insert(lex + word, DictEntry::Data(cell));
    }
}

#[derive(Debug)]
/// Return stack
pub struct RetStack {
    stack: Vec<usize>,
}

impl RetStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Push value to return stack
    pub fn push(&mut self, ret_pos: usize) {
        self.stack.push(ret_pos)
    }

    /// Pop value from return stack
    pub fn pop(&mut self) -> Option<usize> {
        self.stack.pop()
    }
}

#[derive(Debug)]
/// Concatenation, the array of words that conforms the program.
pub struct Concat {
    array: Vec<Cell>,
    pub pointer: usize,
}

impl Concat {
    fn new() -> Self {
        Self { array: Vec::new(), pointer: 0 }
    }

    /// Get next cell from the Concat
    pub fn next(&mut self) -> Option<&Cell> {
        if self.pointer < self.array.len() {
            let cell = &self.array[self.pointer];
            self.pointer += 1;
            Some(cell)
        }
        else {
            None
        }
    }
}

#[derive(Debug)]
/// Stack structure
pub struct Stack {
    stack: Vec<Cell>,
    base: usize,
    nested: Vec<usize>,
}

impl Stack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            base: 0,
            nested: Vec::new(),
        }
    }

    /// Starts a new nested stack
    pub fn start_stack(&mut self) {
        self.nested.push(self.base);
        self.base = self.stack.len();
    }

    /// Ends current stack
    pub fn end_stack(&mut self) -> Option<usize> {
        if let Some(base) = self.nested.pop() {
            self.base = base;
            Some(base)
        }
        else {
            None
        }
    }

    /// Push cell to current stack
    pub fn push(&mut self, cell: Cell) {
        self.stack.push(cell);
    }

    /// Pop cell from current stack
    pub fn pop(&mut self) -> Option<Cell> {
        if self.stack.len() > self.base {
            self.stack.pop()
        }
        else {
            None
        }
    }

    /// Size of current stack
    pub fn size(&self) -> usize {
        self.stack.len() - self.base
    }
}

/// RunPack script interpreter
pub struct Script {
    pub stack: Stack,
    pub dictionary: Dictionary,
    pub ret: RetStack,
    pub concat: Concat,
    reader: String,
    pos: usize,
}

impl Script {
    pub fn new(reader: &str) -> Self {
        let mut script = Self {
            stack: Stack::new(),
            dictionary: Dictionary::new(),
            ret: RetStack::new(),
            concat: Concat::new(),
            reader: reader.into(),
            pos: 0,
        };
        script.tokenize();
        script.def_natives(&[
            ("(", open_parenth), (")", close_parenth), ("{", open_curly), ("}", close_curly), ("def", def), ("+", plus), ("-", minus),
            ("*", star), ("/", slash), ("%", percent), (">", bigger), ("=", equal), ("&", and), ("|", or), ("!", not), ("if", if_word),
            ("ifelse", ifelse_word), ("while", while_word), ("lex", lex), ("[", open_bracket), ("new", new_obj), ("set", set_obj),
            ("get", get_obj), ("key?", key_obj),
        ]);
        script
    }

    fn next_cell(&mut self) -> Cell {
        let mut word_found = false;
        let mut in_string = false;
        let mut in_comment = false;
        let mut last_was_escape = false;
        let mut buff = Vec::new();
        let reader_bytes = self.reader.as_bytes();

        while self.pos < reader_bytes.len() {
            let b = reader_bytes[self.pos];
            self.pos += 1;
            if in_string {
                if b == 92 {    // backslash
                    if last_was_escape {
                        buff.push(b);
                    }
                    last_was_escape = !last_was_escape;
                }
                else if b == 39 {    // apostrophe
                    if last_was_escape {
                        last_was_escape = false;
                        buff.push(b);
                    }
                    else {
                        // End string
                        break;
                    }
                }
                else {
                    last_was_escape = false;
                    buff.push(b);
                }
            }
            else if in_comment {
                if b == 34 {    // quotation mark
                    in_comment = false;
                }
            }
            else {
                if b == 39 {    // apostrophe
                    in_string = true;
                    word_found = true;
                }
                else if b == 34 {    // quotation mark
                    in_comment = true;
                }
                else if b == 44 || b <= 32 {    // Found a word separator (comma, space or any control character)
                    if word_found {
                        break;
                    }
                }
                else {
                    word_found = true;
                    buff.push(b);
                }
            }
        }

        if in_string {
            if let Ok(token) = String::from_utf8(buff) {
                Cell::String(token)
            }
            else {
                //TODO: string parse error
                Cell::Empty
            }
        }
        else {
            if buff.len() > 0 {
                self.parse_token(buff)
            }
            else {
                Cell::Empty
            }
        }
    }

    fn parse_token(&mut self, token: Vec<u8>) -> Cell {
        if let Ok(token) = String::from_utf8(token) {
            if let Some(num_cell) = Cell::number(&token) {
                num_cell
            }
            else if let Some(sym_cell) = Cell::symbol(&token) {
                sym_cell
            }
            else if let Some(bool_cell) = Cell::boolean(&token) {
                bool_cell
            }
            else {
                Cell::Word(token)
            }
        }
        else {
            //TODO: string parse error
            Cell::Empty
        }
    }
    
    fn tokenize(&mut self) {
        loop {
            let cell = self.next_cell();
            if let Cell::Empty = cell {
                //TODO: add error handling, when the parser fails somewhere
                break;
            }
            else {
                self.concat.array.push(cell);
            }
        }
    }

    /// Define a batch of native functions
    pub fn def_natives(&mut self, list: &[(&str, fn(&mut Script) -> Result<bool, Error>)]) {
        list.iter().for_each(|(word_name, function)| {
            self.dictionary.native(word_name, *function);
        });
    }
    
    /// Execute a word from the dictionary
    pub fn exec(&mut self, word: &str) -> Result<bool, Error> {
        if let Some(dict_entry) = self.dictionary.dict.get(word) {
            match dict_entry {
                DictEntry::Native(func) => {
                    return func(self);
                },
                DictEntry::Defined(block_ref) => {
                    let block = *block_ref;
                    return self.run_block(&block);
                },
                DictEntry::Data(data_cell) => {
                    self.stack.push(data_cell.clone());
                },
            }
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    /// Append literal code to the end of the Concat.
    pub fn append(&mut self, code: &str) {
        self.reader = code.into();
        self.pos = 0;
        self.tokenize();
    }

    /// Run the script
    pub fn run(&mut self) -> Result<bool, Error> {
        loop {
            match self.one_step() {
                Ok(false) => return Ok(true),
                Err(e) => return Err(e),
                _ => {}
            }
        }
    }
    
    /// Run a block
    pub fn run_block(&mut self, block: &BlockRef) -> Result<bool, Error> {
        let init_pointer = self.concat.pointer;
        self.ret.push(self.concat.pointer);
        self.concat.pointer = block.pos;
        loop {
            match self.one_step() {
                Ok(true) => {
                    if self.concat.pointer == init_pointer {
                        return Ok(true);
                    }
                },
                Err(e) => return Err(e),
                _ => return Ok(false)
            }
        }
    }

    /// Run one cell of the Concat
    pub fn one_step(&mut self) -> Result<bool, Error> {
        if let Some(cell) = self.concat.next() {
            let cell = cell.clone();
            match cell {
                Cell::Integer(_) | Cell::Float(_) | Cell::Boolean(_) | Cell::Symbol(_) | Cell::String(_) => self.stack.push(cell),
                Cell::Word(w) => return self.exec(&w),
                _ => return Err(Error::new(format!("Found an invalid cell value in the Concat: {:?}", cell), 1))
            }
            Ok(true)
        }
        else {
            Ok(false)
        }
    }
}

// Primitives

fn open_parenth(script: &mut Script) -> Result<bool, Error> {
    script.stack.start_stack();
    Ok(true)
}

fn close_parenth(script: &mut Script) -> Result<bool, Error> {
    if let None = script.stack.end_stack() {
        Err(Error::new("close_parenth: Stack level undeflow".into(), 2))
    }
    else {
        Ok(true)
    }
}

fn open_curly(script: &mut Script) -> Result<bool, Error> {
    let pos = script.concat.pointer;
    let mut level = 1;
    loop {
        if let Some(cell) = script.concat.next() {
            if let Cell::Word(w) = cell {
                if w == "}" {
                    level -= 1;
                    if level == 0 {
                        let len = script.concat.pointer - pos;
                        script.stack.push(Cell::Block(BlockRef { pos, len }));
                        break;
                    }
                }
                else if w == "{" {
                    level += 1;
                }
            }
        }
        else {
            return Err(Error::new("open_curly: Reached the end and didn't find a closing block".into(), 3));
        }
    }
    Ok(true)
}

fn close_curly(script: &mut Script) -> Result<bool, Error> {
    if let Some(pos) = script.ret.pop() {
        script.concat.pointer = pos;
        Ok(true)
    }
    else {
        Err(Error::new("close_curly: Return stack underflow".into(), 4))
    }
}

fn def(script: &mut Script) -> Result<bool, Error> {
    let (data, word) = (script.stack.pop(), script.concat.next());
    if let Some(Cell::Word(word)) = word {
        if let Some(Cell::Block(block)) = data {
            script.dictionary.block(word, block);
        }
        else if let Some(cell) = data {
            script.dictionary.data(word, cell);
        }
        else {
            return Err(Error::new("def: Expecting a block or a cell".into(), 5));
        }
    }
    else {
        return Err(Error::new("def: Expecting a word in the Concat".into(), 6));
    }
    Ok(true)
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
        return Err(Error::new("two_num_op: Expecting two numbers of the same type".into(), 7));
    }
    Ok(true)
}

fn plus(script: &mut Script) -> Result<bool, Error> {
    two_num_op(&mut script.stack, |a, b| a + b, |a, b| a + b)
}

fn minus(script: &mut Script) -> Result<bool, Error> {
    two_num_op(&mut script.stack, |a, b| a - b, |a, b| a - b)
}

fn star(script: &mut Script) -> Result<bool, Error> {
    two_num_op(&mut script.stack, |a, b| a * b, |a, b| a * b)
}

fn slash(script: &mut Script) -> Result<bool, Error> {
    two_num_op(&mut script.stack, |a, b| a / b, |a, b| a / b)
}

fn percent(script: &mut Script) -> Result<bool, Error> {
    two_num_op(&mut script.stack, |a, b| a % b, |a, b| a % b)
}

fn bigger(script: &mut Script) -> Result<bool, Error> {
    let (cell_b, cell_a) = (script.stack.pop(), script.stack.pop());
    script.stack.push(Cell::Boolean(cell_a > cell_b));
    Ok(true)
}

fn equal(script: &mut Script) -> Result<bool, Error> {
    let (cell_b, cell_a) = (script.stack.pop(), script.stack.pop());
    script.stack.push(Cell::Boolean(cell_a == cell_b));
    Ok(true)
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
        return Err(Error::new("two_logic_op: Expecting two booleans or two integers".into(), 8));
    }
    Ok(true)
}

fn and(script: &mut Script) -> Result<bool, Error> {
    two_logic_op(&mut script.stack, |a, b| a & b, |a, b| a & b)
}

fn or(script: &mut Script) -> Result<bool, Error> {
    two_logic_op(&mut script.stack, |a, b| a | b, |a, b| a | b)
}

fn not(script: &mut Script) -> Result<bool, Error> {
    let cell = script.stack.pop();
    if let Some(Cell::Boolean(a)) = cell {
        script.stack.push(Cell::Boolean(!a));
    }
    else if let Some(Cell::Integer(a)) = cell {
        script.stack.push(Cell::Integer(!a));
    }
    else {
        return Err(Error::new("not: Expecting a boolean or an integer".into(), 9));
    }
    Ok(true)
}

fn if_word(script: &mut Script) -> Result<bool, Error> {
    if let (Some(Cell::Boolean(cond)), Some(Cell::Block(blk))) = (script.stack.pop(), script.stack.pop()) {
        if cond {
            script.run_block(&blk)?;
        }
    }
    else {
        return Err(Error::new("ifelse: couldn't find condition and 1 block".into(), 10));
    }
    Ok(true)
}

fn ifelse_word(script: &mut Script) -> Result<bool, Error> {
    if let (Some(Cell::Boolean(cond)), Some(Cell::Block(false_blk)), Some(Cell::Block(true_blk))) = (script.stack.pop(), script.stack.pop(), script.stack.pop()) {
        script.ret.push(script.concat.pointer);
        if cond {
            script.run_block(&true_blk)
        }
        else {
            script.run_block(&false_blk)
        }
    }
    else {
        Err(Error::new("ifelse: couldn't find condition and 2 blocks".into(), 11))
    }
}

fn while_word(script: &mut Script) -> Result<bool, Error> {
    if let (Some(Cell::Block(cond_blk)), Some(Cell::Block(loop_blk))) = (script.stack.pop(), script.stack.pop()) {
        loop {
            script.run_block(&cond_blk)?;
            if let Some(Cell::Boolean(cond)) = script.stack.pop() {
                if cond {
                    script.run_block(&loop_blk)?;
                }
                else {
                    break;
                }
            }
            else {
                return Err(Error::new("while: condition didn't produce a bool".into(), 12));
            }
        }
    }
    else {
        return Err(Error::new("while: couldn't find 2 blocks".into(), 13));
    }
    Ok(true)
}

fn lex(script: &mut Script) -> Result<bool, Error> {
    if let Some(Cell::String(lex_name)) = script.concat.next() {
        script.dictionary.lex = lex_name.clone();
        Ok(true)
    }
    else {
        return Err(Error::new("lex: couldn't find string".into(), 14));
    }
}

fn open_bracket(script: &mut Script) -> Result<bool, Error> {
    let mut vars: HashMap<String, Cell> = HashMap::with_capacity(16);
    while let Some(Cell::Word(w)) = script.concat.next() {
        if w == ":" {
            break;
        }
        else {
            if let Some(cell) = script.stack.pop() {
                vars.insert(w.clone(), cell);
            }
            else {
                return Err(Error::new("open_bracket: stack is empty".into(), 15));
            }
        }
    }
    while let Some(Cell::Word(w)) = script.concat.next() {
        if w == "]" {
            break;
        }
        else {
            if let Some(k) = vars.get(w) {
                script.stack.push(k.clone());
            }
            else {
                return Err(Error::new("open_bracket: Couldn't find variable name".into(), 16));
            }
        }
    }
    Ok(true)
}

fn new_obj(script: &mut Script) -> Result<bool, Error> {
    if script.stack.size() % 2 == 0 && script.stack.size() > 0 {
        let mut obj = Object::new("obj");
        while let (Some(val), Some(key)) = (script.stack.pop(), script.stack.pop()) {
            obj.map.insert(key, val);
        }
        script.stack.push(Cell::Object(obj));
    }
    else {
        return Err(Error::new("new: Stack must contain key-value pairs".into(), 17));
    }
    Ok(true)
}

fn set_obj(script: &mut Script) -> Result<bool, Error> {
    if let (Some(val), Some(key), Some(Cell::Word(w))) = (script.stack.pop(), script.stack.pop(), script.concat.next()) {
        if let Some(DictEntry::Data(Cell::Object(obj))) = script.dictionary.dict.get_mut(w) {
            obj.map.insert(key, val);
        }
        else {
            return Err(Error::new(format!("set: dictionary doesn't contain an Object for word '{}'", w), 18));
        }
    }
    else {
        return Err(Error::new("set: Couldn't get a key-value pair and a word".into(), 19));
    }
    Ok(true)
}

fn get_obj(script: &mut Script) -> Result<bool, Error> {
    if let (Some(key), Some(Cell::Word(w))) = (script.stack.pop(), script.concat.next()) {
        if let Some(DictEntry::Data(Cell::Object(obj))) = script.dictionary.dict.get(w) {
            if let Some(val) = obj.map.get(&key) {
                script.stack.push(val.clone());
            }
            else {
                return Err(Error::new("get: key doesn't exist in object".into(), 20));
            }
        }
        else {
            return Err(Error::new(format!("get: dictionary doesn't contain an Object for word '{}'", w), 21));
        }
    }
    else {
        return Err(Error::new("get: Couldn't get a value and a word".into(), 22));
    }
    Ok(true)
}

fn key_obj(script: &mut Script) -> Result<bool, Error> {
    if let (Some(key), Some(Cell::Word(w))) = (script.stack.pop(), script.concat.next()) {
        if let Some(DictEntry::Data(Cell::Object(obj))) = script.dictionary.dict.get(w) {
            script.stack.push(Cell::Boolean(obj.map.contains_key(&key)));
        }
        else {
            return Err(Error::new(format!("key: dictionary doesn't contain an Object for word '{}'", w), 23));
        }
    }
    else {
        return Err(Error::new("key: Couldn't get a value and a word".into(), 24));
    }
    Ok(true)
}
