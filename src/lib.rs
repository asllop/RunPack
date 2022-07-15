#![no_std]

extern crate alloc;

use hashbrown::HashMap;
use alloc::{
    vec::Vec,
    string::String,
};

// TODO: Custom objects, add a new variant of Cell and create a Trait

/// Integer type alias
pub type IntegerType = i64;

/// Float type alias
pub type FloatType = f64;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
/// Data primitive
pub enum Cell {
    Empty,  // TODO: once we implement error handling, we won't need an empty variant
    Integer(IntegerType),
    Float(FloatType),
    Boolean(bool),
    Symbol(String),
    String(String),
    Word(String),
    Block(BlockRef),
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

#[derive(PartialEq, PartialOrd, Clone, Debug)]
/// Block reference
pub struct BlockRef {
    pub pos: usize,
    pub len: usize,
}

enum DictEntry {
    Native(fn(&mut Script)),
    Defined(BlockRef),
    Data(Cell),
}

/// Dictionary of words
pub struct Dictionary {
    dict: HashMap<String, DictEntry>,
    lex: String,
}

impl Dictionary {
    fn new() -> Self {
        Self { dict: HashMap::new(), lex: String::default() }
    }

    /// Define a native word
    pub fn native(&mut self, word: &str, func: fn(&mut Script)) {
        self.dict.insert(word.into(), DictEntry::Native(func));
    }

    /// Define block word
    pub fn block(&mut self, word: &str, block: BlockRef) {
        self.dict.insert(word.into(), DictEntry::Defined(block));
    }

    /// Define data word
    pub fn data(&mut self, word: &str, cell: Cell) {
        self.dict.insert(word.into(), DictEntry::Data(cell));
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
            ("ifelse", ifelse_word), ("while", while_word), ("lex", lex), ("[", open_bracket),
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
    pub fn def_natives(&mut self, list: &[(&str, fn(&mut Script))]) {
        list.iter().for_each(|(word_name, function)| {
            self.dictionary.native(word_name, *function);
        });
    }
    
    /// Execute a word from the dictionary
    pub fn exec(&mut self, word: &str) -> bool {
        if let Some(dict_entry) = self.dictionary.dict.get(word) {
            match dict_entry {
                DictEntry::Native(func) => {
                    func(self);
                },
                DictEntry::Defined(block_ref) => {
                    let block = block_ref.clone();
                    self.run_block(&block);
                },
                DictEntry::Data(data_cell) => {
                    self.stack.push(data_cell.clone());
                },
            }
            true
        }
        else {
            false
        }
    }

    /// Append literal code to the end of the Concat.
    pub fn append(&mut self, code: &str) {
        self.reader = code.into();
        self.pos = 0;
        self.tokenize();
    }

    /// Run the script
    pub fn run(&mut self) {
        while self.one_step() {}
    }
    
    /// Run a block
    pub fn run_block(&mut self, block: &BlockRef) {
        let init_pointer = self.concat.pointer;
        self.ret.push(self.concat.pointer);
        self.concat.pointer = block.pos;
        while self.one_step() {
            if self.concat.pointer == init_pointer {
                break;
            }
        }
    }

    /// Run one cell of the Concat
    pub fn one_step(&mut self) -> bool {
        if let Some(cell) = self.concat.next() {
            let cell = cell.clone();
            match cell {
                Cell::Integer(_) | Cell::Float(_) | Cell::Boolean(_) | Cell::Symbol(_) | Cell::String(_) => self.stack.push(cell),
                Cell::Word(w) => {
                    if let Some(dict_entry) = self.dictionary.dict.get(&w) {
                        match dict_entry {
                            DictEntry::Native(func) => {
                                func(self);
                            },
                            DictEntry::Defined(block_ref) => {
                                self.ret.push(self.concat.pointer);
                                self.concat.pointer = block_ref.pos;
                            },
                            DictEntry::Data(data_cell) => {
                                self.stack.push(data_cell.clone());
                            },
                        }
                    }
                    else {
                        panic!("{}: word not found in the dictionary", w);
                    }
                },
                _ => panic!("Found an invalid cell value in the Concat: {:?}", cell)
            }
            true
        }
        else {
            false
        }
    }
}

// Primitives

fn open_parenth(script: &mut Script) {
    script.stack.start_stack();
}

fn close_parenth(script: &mut Script) {
    if let None = script.stack.end_stack() {
        panic!("close_parenth: Stack level undeflow");
    }
}

fn open_curly(script: &mut Script) {
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
            panic!("open_curly: Reached the end and didn't find a closing block");
        }
    }
}

fn close_curly(script: &mut Script) {
    if let Some(pos) = script.ret.pop() {
        script.concat.pointer = pos;
    }
    else {
        panic!("close_curly: Return stack underflow");
    }
}

fn def(script: &mut Script) {
    let (data, word) = (script.stack.pop(), script.concat.next());
    if let Some(Cell::Word(word)) = word {
        if let Some(Cell::Block(block)) = data {
            let lex = script.dictionary.lex.clone();
            script.dictionary.block(&(lex + word), block);
        }
        else if let Some(cell) = data {
            let lex = script.dictionary.lex.clone();
            script.dictionary.data(&(lex + word), cell);
        }
        else {
            panic!("def: Expecting a block or a cell");
        }
    }
    else {
        panic!("def: Expecting a word in the Concat");
    }
}

fn two_num_op(stack: &mut Stack, int_op: fn(IntegerType, IntegerType) -> IntegerType, flt_op: fn(FloatType, FloatType) -> FloatType) {
    let (cell_b, cell_a) = (stack.pop(), stack.pop());
    if let (Some(Cell::Integer(int_a)), Some(Cell::Integer(int_b))) = (&cell_a, &cell_b) {
        stack.push(Cell::Integer(int_op(*int_a, *int_b)));
    }
    else if let (Some(Cell::Float(flt_a)), Some(Cell::Float(flt_b))) = (&cell_a, &cell_b) {
        stack.push(Cell::Float(flt_op(*flt_a, *flt_b)));
    }
    else {
        panic!("two_num_op: Expecting two numbers of the same type");
    }
}

fn plus(script: &mut Script) {
    two_num_op(&mut script.stack, |a, b| a + b, |a, b| a + b);
}

fn minus(script: &mut Script) {
    two_num_op(&mut script.stack, |a, b| a - b, |a, b| a - b);
}

fn star(script: &mut Script) {
    two_num_op(&mut script.stack, |a, b| a * b, |a, b| a * b);
}

fn slash(script: &mut Script) {
    two_num_op(&mut script.stack, |a, b| a / b, |a, b| a / b);
}

fn percent(script: &mut Script) {
    two_num_op(&mut script.stack, |a, b| a % b, |a, b| a % b);
}

fn bigger(script: &mut Script) {
    let (cell_b, cell_a) = (script.stack.pop(), script.stack.pop());
    script.stack.push(Cell::Boolean(cell_a > cell_b));
}

fn equal(script: &mut Script) {
    let (cell_b, cell_a) = (script.stack.pop(), script.stack.pop());
    script.stack.push(Cell::Boolean(cell_a == cell_b));
}

fn two_logic_op(stack: &mut Stack, op_bool: fn(bool, bool) -> bool, op_int: fn(IntegerType, IntegerType) -> IntegerType) {
    let (cell_b, cell_a) = (stack.pop(), stack.pop());
    if let (Some(Cell::Boolean(bool_a)), Some(Cell::Boolean(bool_b))) = (&cell_a, &cell_b) {
        stack.push(Cell::Boolean(op_bool(*bool_a, *bool_b)));
    }
    else if let (Some(Cell::Integer(int_a)), Some(Cell::Integer(int_b))) = (&cell_a, &cell_b) {
        stack.push(Cell::Integer(op_int(*int_a, *int_b)));
    }
    else {
        panic!("two_logic_op: Expecting two booleans or two integers");
    }
}

fn and(script: &mut Script) {
    two_logic_op(&mut script.stack, |a, b| a & b, |a, b| a & b);
}

fn or(script: &mut Script) {
    two_logic_op(&mut script.stack, |a, b| a | b, |a, b| a | b);
}

fn not(script: &mut Script) {
    let cell = script.stack.pop();
    if let Some(Cell::Boolean(a)) = cell {
        script.stack.push(Cell::Boolean(!a));
    }
    else if let Some(Cell::Integer(a)) = cell {
        script.stack.push(Cell::Integer(!a));
    }
    else {
        panic!("not: Expecting a boolean or an integer");
    }
}

fn if_word(script: &mut Script) {
    if let (Some(Cell::Boolean(cond)), Some(Cell::Block(blk))) = (script.stack.pop(), script.stack.pop()) {
        if cond {
            script.run_block(&blk);
        }
    }
    else {
        panic!("ifelse: couldn't find condition and 1 block");
    }
}

fn ifelse_word(script: &mut Script) {
    if let (Some(Cell::Boolean(cond)), Some(Cell::Block(false_blk)), Some(Cell::Block(true_blk))) = (script.stack.pop(), script.stack.pop(), script.stack.pop()) {
        script.ret.push(script.concat.pointer);
        if cond {
            script.run_block(&true_blk);
        }
        else {
            script.run_block(&false_blk);
        }
    }
    else {
        panic!("ifelse: couldn't find condition and 2 blocks");
    }
}

fn while_word(script: &mut Script) {
    if let (Some(Cell::Block(cond_blk)), Some(Cell::Block(loop_blk))) = (script.stack.pop(), script.stack.pop()) {
        loop {
            script.run_block(&cond_blk);
            if let Some(Cell::Boolean(cond)) = script.stack.pop() {
                if cond {
                    script.run_block(&loop_blk);
                }
                else {
                    break;
                }
            }
            else {
                panic!("while: condition didn't produce a bool");
            }
        }
    }
    else {
        panic!("while: couldn't find 2 blocks");
    }
}

fn lex(script: &mut Script) {
    if let Some(Cell::String(lex_name)) = script.concat.next() {
        script.dictionary.lex = lex_name.clone();
    }
    else {
        panic!("lex: couldn't find string");
    }
}

fn open_bracket(script: &mut Script) {
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
                panic!("open_bracket: stack is empty");
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
                panic!("open_bracket: Couldn't find variable name");
            }
        }
    }
}
