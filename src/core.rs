use hashbrown::HashMap;
use alloc::{boxed::Box, vec::Vec, string::String, format, str};
use core::hash::Hash;
use super::primitives::register_primitives;
use super::prelude::PRELUDE;
use super::run_future::RunFuture;

#[derive(Debug)]
/// Error type
pub struct Error {
    pub msg: String,
}

impl Error {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Copy, Debug)]
/// Block reference type
pub struct BlockRef {
    pub pos: usize,
    pub len: usize,
}

/// Extended Option.
pub enum ExtOption<'a> {
    /// Nothing.
    None,
    /// Cell value.
    Some(Cell),
    /// Cell reference.
    SomeRef(&'a Cell),
    /// Mutable Cell reference.
    SomeMutRef(&'a mut Cell),
    /// Invalid command or arguments.
    Invalid,
}

impl<'a> From<Option<Cell>> for ExtOption<'a> {
    fn from(val: Option<Cell>) -> Self {
        if let Some(cell) = val {
            ExtOption::Some(cell)
        }
        else {
            ExtOption::None
        }
    }
}

impl<'a> From<Option<&'a Cell>> for ExtOption<'a> {
    fn from(val: Option<&'a Cell>) -> Self {
        if let Some(cell) = val {
            ExtOption::SomeRef(cell)
        }
        else {
            ExtOption::None
        }
    }
}

impl<'a> From<Option<&'a mut Cell>> for ExtOption<'a> {
    fn from(val: Option<&'a mut Cell>) -> Self {
        if let Some(cell) = val {
            ExtOption::SomeMutRef(cell)
        }
        else {
            ExtOption::None
        }
    }
}

/// Trait for generic structs.
pub trait StructCell: core::fmt::Debug {
    /// Clone wrapper.
    fn object_clone(&self) -> Box<dyn StructCell>;
    /// Execute a command.
    fn doit(&self, cmd: &str, args: Option<Vec<Cell>>) -> ExtOption;
    /// Execute a command in a mutable instance.
    fn doit_mut(&mut self, cmd: &str, args: Option<Vec<Cell>>) -> ExtOption;
}

#[derive(Debug)]
/// Struct Cell.
pub struct Struct {
    /// Struct name.
    pub name: String,
    /// Struct data.
    pub object: Box<dyn StructCell>,
}

impl PartialEq for Struct {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Struct {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Clone for Struct {
    fn clone(&self) -> Self {
        Self { name: self.name.clone(), object: self.object.object_clone() }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
/// Data primitive
pub enum Cell {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Word(String),
    Block(BlockRef),
    Struct(Struct),
}

impl Cell {
    fn number(token: &str) -> Option<Self> {
        //TODO: support hex and binary integers (https://doc.rust-lang.org/std/primitive.i64.html#method.from_str_radix)
        if let Ok(int) = token.parse::<i64>() {
            Some(int.into())
        }
        else if let Ok(flt) = token.parse::<f64>() {
            Some(flt.into())
        }
        else {
            None
        }
    }

    fn boolean(token: &str) -> Option<Self> {
        if token == "true" {
            Some(true.into())
        }
        else if token == "false" {
            Some(false.into())
        }
        else {
            None
        }
    }
}

impl Hash for Cell {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Eq for Cell {}

impl From<String> for Cell {
    fn from(val: String) -> Self {
        Cell::String(val)
    }
}

impl From<&str> for Cell {
    fn from(val: &str) -> Self {
        Cell::String(val.into())
    }
}

impl From<i64> for Cell {
    fn from(val: i64) -> Self {
        Cell::Integer(val)
    } 
}

impl From<f64> for Cell {
    fn from(val: f64) -> Self {
        Cell::Float(val)
    } 
}

impl From<bool> for Cell {
    fn from(val: bool) -> Self {
        Cell::Boolean(val)
    } 
}

impl From<BlockRef> for Cell {
    fn from(val: BlockRef) -> Self {
        Cell::Block(val)
    }
}

impl From<Struct> for Cell {
    fn from(val: Struct) -> Self {
        Cell::Struct(val)
    }
}

//TODO: Make native words async, or how to interact with async functions

#[derive(Clone)]
/// Dictionary entry
pub enum DictEntry {
    Native(fn(&mut Pack) -> Result<bool, Error>),
    Defined(BlockRef),
    Data(Cell),
}

#[derive(Default, Clone)]
/// Dictionary of words
pub struct Dictionary {
    pub dict: HashMap<String, DictEntry>,
    pub lex: String,
}

impl Dictionary {
    /// Define a native word
    pub fn native(&mut self, word: &str, func: fn(&mut Pack) -> Result<bool, Error>) {
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

#[derive(Default, Debug, Clone)]
/// Return stack
pub struct RetStack {
    stack: Vec<usize>,
}

impl RetStack {
    /// Push value to return stack
    pub fn push(&mut self, ret_pos: usize) {
        self.stack.push(ret_pos)
    }

    /// Pop value from return stack
    pub fn pop(&mut self) -> Option<usize> {
        self.stack.pop()
    }

    /// Stack size
    pub fn size(&self) -> usize {
        self.stack.len()
    }
}

#[derive(Default, Debug, Clone)]
/// Concatenation, the array of words that conforms the program.
pub struct Concat {
    pub array: Vec<Cell>,
    pub pointer: usize,
}

impl Concat {
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

    /// Get next cell from the Concat, cloning it
    pub fn next_clone(&mut self) -> Option<Cell> {
        if self.pointer < self.array.len() {
            let cell = &self.array[self.pointer];
            self.pointer += 1;
            Some(cell.clone())
        }
        else {
            None
        }
    }
}

#[derive(Debug, Default, Clone)]
/// Stack structure
pub struct Stack {
    stack: Vec<Cell>,
    base: usize,
    nested: Vec<usize>,
}

impl Stack {
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

    /// Get cell. Position is referenced to the top.
    pub fn get(&self, pos: usize) -> Option<&Cell> {
        if pos < self.size() {
            self.stack.get(self.stack.len() - 1 - pos)
        }
        else {
            None
        }
    }

    /// Get mutable cell. Position is referenced to the top.
    pub fn get_mut(&mut self, pos: usize) -> Option<&mut Cell> {
        if pos < self.size() {
            let len = self.stack.len();
            self.stack.get_mut(len - 1 - pos)
        }
        else {
            None
        }
    }

    /// Size of current stack
    pub fn size(&self) -> usize {
        self.stack.len() - self.base
    }

    /// Clear current stack
    pub fn clear(&mut self) {
        self.stack.drain(self.base..);
    }
}

#[derive(Default, Clone)]
/// Pack of structures that form the RunPack interpreter
pub struct Pack {
    pub stack: Stack,
    pub dictionary: Dictionary,
    pub ret: RetStack,
    pub concat: Concat,
}

impl Pack {
    /// Create a new Pack with registered primitives and prelude.
    pub fn new() -> Self {
        let mut pack = Pack::default();
        register_primitives(&mut pack);
        pack.code(PRELUDE);
        pack
    }

    fn next_cell(&mut self, code: &str, mut pos: usize) -> (Option<Cell>, usize) {
        let mut word_found = false;
        let mut in_string = false;
        let mut in_comment = false;
        let mut last_was_escape = false;
        let mut buff = Vec::new();
        let code_bytes = code.as_bytes();

        while pos < code_bytes.len() {
            let b = code_bytes[pos];
            pos += 1;
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
                (Some(token.into()), pos)
            }
            else {
                //TODO: string parse error
                (None, pos)
            }
        }
        else {
            if buff.len() > 0 {
                (self.parse_token(buff), pos)
            }
            else {
                (None, pos)
            }
        }
    }

    fn parse_token(&mut self, token: Vec<u8>) -> Option<Cell> {
        if let Ok(token) = String::from_utf8(token) {
            if let Some(num_cell) = Cell::number(&token) {
                Some(num_cell)
            }
            else if let Some(bool_cell) = Cell::boolean(&token) {
                Some(bool_cell)
            }
            else {
                Some(Cell::Word(token))
            }
        }
        else {
            //TODO: string parse error
            None
        }
    }

    /// Define a batch of native functions
    pub(crate) fn def_natives(&mut self, list: &[(&str, fn(&mut Pack) -> Result<bool, Error>)]) {
        list.iter().for_each(|(word_name, function)| {
            self.dictionary.native(word_name, *function);
        });
    }
    
    /// Execute a word from the dictionary.
    /// 
    /// Note: If word is Defined, we must call `run` afterward.
    pub fn exec(&mut self, word: &str) -> Result<bool, Error> {
        if let Some(dict_entry) = self.dictionary.dict.get(word) {
            // Cloning the DictEntry is necessary because a Data entry will have to be put into the stack,
            // and in the other two variants, Native and Defined, a clone has very little performance impact.
            let dict_entry = dict_entry.clone();
            self.exec_dict_entry(dict_entry)
        }
        else {
            Err(Error::new(format!("Word '{}' doesn't exist in dictionary", word)))
        }
    }

    /// Execute a dictionary entry
    pub fn exec_dict_entry(&mut self, dict_entry: DictEntry) -> Result<bool, Error> {
        match dict_entry {
            DictEntry::Native(func) => {
                func(self)
            },
            DictEntry::Defined(block_ref) => {
                self.ret.push(self.concat.pointer);
                self.concat.pointer = block_ref.pos;
                Ok(true)
            },
            DictEntry::Data(data_cell) => {
                self.stack.push(data_cell);
                Ok(true)
            },
        }
    }

    /// Append code to the end of the Concat.
    pub fn code(&mut self, code: &str) {
        let mut pos = 0;
        // tokenize and push cells into the Concat
        loop {
            let (cell, tmp_pos) = self.next_cell(code, pos);
            pos = tmp_pos;
            if let Some(cell) = cell {
                self.concat.array.push(cell);
            }
            else {
                //TODO: add error handling, when the parser fails somewhere
                break;
            }
        }
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

    //TODO: create "run_word", a public version of "exec" that in case of a Defined word,
    // it automatically calls run and make the current "exec" pub(crate).

    /// Run one cell from the Concat
    pub fn one_step(&mut self) -> Result<bool, Error> {
        if let Some(cell) = self.concat.next() {
            let cell = cell.clone();
            match cell {
                Cell::Word(w) => return self.exec(&w),
                _ => self.stack.push(cell),
            }
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    //TODO: create an async version of "run_word" to partner with "async_run".

    /// Async version of run().
    pub fn async_run(&mut self) -> RunFuture {
        RunFuture::new(self)
    }
}
