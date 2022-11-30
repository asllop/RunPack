/*
TODO:

Modifications to make it fully async
    - Integrate the runpack_async into the core.
    - Remove the loop word, all loops must be done using recursion.
    - Rework "reenter", and convert it into "loop{", that works like "{" + "reenter".

Modifications to support any struct in the stack/concat:
    - Remove Map and Vector, and use the generic Struct instead.
 */

use hashbrown::HashMap;
use alloc::{boxed::Box, vec::Vec, string::String, format, str};
use core::hash::Hash;
use super::primitives::register_primitives;
use super::prelude::PRELUDE;

// TODO:
//       - add error location info: concat pos, ret stack (backtrace), and word that caused the crash.

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

#[derive(Default, Eq, Clone, Debug)]
/// Map type
pub struct Map {
    pub map: HashMap<Cell, Cell>,
}

impl Hash for Map {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.map.iter().for_each(|(k, v)| {
            k.hash(state);
            v.hash(state)
        });
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.map.len() == other.map.len()
    }
}

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.map.len().partial_cmp(&other.map.len())
    }
}

#[derive(Default, Eq, PartialEq, PartialOrd, Clone, Debug)]
/// Custom vector type
pub struct Vector {
    pub vector: Vec<Cell>,
}

/// Trait for generic structs in a Cell.
pub trait StructCell: core::fmt::Debug {
    /// Clone wrapper.
    fn custom_clone(&self) -> Box<dyn StructCell>;
    /// Set value.
    fn set(&mut self, key: Cell, value: Cell);
    /// Remove value.
    fn rem(&mut self, key: &Cell) -> Option<Cell>;
    /// Get value.
    fn get(&self, key: &Cell) -> Option<&Cell>;
    /// Execute a command.
    fn doit(&self, cmd: &str, args: Option<Vec<Cell>>) -> Option<Cell>;
    /// Execute a command in a mutable instance.
    fn doit_mut(&mut self, cmd: &str, args: Option<Vec<Cell>>) -> Option<Cell>;
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
        Self { name: self.name.clone(), object: self.object.custom_clone() }
    }
}

/// Integer type alias
pub type IntegerType = i64;

/// Float type alias
pub type FloatType = f64;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
/// Data primitive
pub enum Cell {
    Empty,
    Integer(IntegerType),
    Float(FloatType),
    Boolean(bool),
    String(String),
    Word(String),
    Block(BlockRef),
    Map(Map),
    Vector(Vector),
    Struct(Struct),
}

impl Cell {
    fn number(token: &str) -> Option<Self> {
        //TODO: support hex and binary integers (https://doc.rust-lang.org/std/primitive.i64.html#method.from_str_radix)
        if let Ok(int) = token.parse::<IntegerType>() {
            Some(int.into())
        }
        else if let Ok(flt) = token.parse::<FloatType>() {
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

impl From<IntegerType> for Cell {
    fn from(val: IntegerType) -> Self {
        Cell::Integer(val)
    } 
}

impl From<FloatType> for Cell {
    fn from(val: FloatType) -> Self {
        Cell::Float(val)
    } 
}

impl From<bool> for Cell {
    fn from(val: bool) -> Self {
        Cell::Boolean(val)
    } 
}

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
}

#[derive(Default, Clone)]
/// Pack of structures that form the RunPack interpreter
pub struct Pack {
    pub stack: Stack,
    pub dictionary: Dictionary,
    pub ret: RetStack,
    pub concat: Concat,
    reader: String,
    pos: usize,
}

impl Pack {
    /// Create a new Pack with registered primitives and prelude.
    pub fn new() -> Self {
        let mut pack = Pack::default();
        register_primitives(&mut pack);
        pack.code(PRELUDE);
        pack
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
                token.into()
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
    pub fn def_natives(&mut self, list: &[(&str, fn(&mut Pack) -> Result<bool, Error>)]) {
        list.iter().for_each(|(word_name, function)| {
            self.dictionary.native(word_name, *function);
        });
    }
    
    /// Execute a word from the dictionary
    pub fn exec(&mut self, word: &str) -> Result<bool, Error> {
        if let Some(dict_entry) = self.dictionary.dict.get(word) {
            match dict_entry {
                DictEntry::Native(func) => {
                    func(self)
                },
                DictEntry::Defined(block_ref) => {
                    let block = block_ref.clone();
                    self.run_block(&block)
                },
                DictEntry::Data(data_cell) => {
                    self.stack.push(data_cell.clone());
                    Ok(true)
                },
            }
        }
        else {
            Err(Error::new(format!("Word '{}' doesn't exist in dictionary", word)))
        }
    }

    /// Append code to the end of the Concat.
    pub fn code(&mut self, code: &str) {
        self.reader = code.into();
        self.tokenize();
        self.reader = String::new();
        self.pos = 0;
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
    
    /// Run a block and return when finished.
    pub fn run_block(&mut self, block: &BlockRef) -> Result<bool, Error> {
        self.ret.push(self.concat.pointer);
        self.concat.pointer = block.pos;
        loop {
            match self.one_step() {
                Ok(true) => {
                    if self.concat.pointer == block.pos + block.len - 1 {
                        return Ok(true);
                    }
                },
                Err(e) => return Err(e),
                _ => return Ok(false)
            }
        }
    }

    /// Run one cell from the Concat
    pub fn one_step(&mut self) -> Result<bool, Error> {
        if let Some(cell) = self.concat.next() {
            let cell = cell.clone();
            match cell {
                Cell::Word(w) => return self.exec(&w),
                Cell::Empty => return Err(Error::new(format!("Found an invalid cell value in the Concat: {:?}", cell))),
                _ => self.stack.push(cell),
            }
            Ok(true)
        }
        else {
            Ok(false)
        }
    }
}
