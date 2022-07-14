//TODO: use no_std, hashbrown::HashMap and alloc::vec::Vec
use std::collections::HashMap;

#[derive(Clone, Debug)]
/// Block reference
pub struct BlockRef {
    pub pos: usize,
    pub len: usize,
}

enum DictEntry {
    Native(fn(&mut Stack, &mut Concat, &mut Dictionary, &mut RetStack)),
    Defined(BlockRef),
    Data(Cell),
}

/// Dictionary of words
pub struct Dictionary {
    dict: HashMap<String, DictEntry>,
}

impl Dictionary {
    fn new() -> Self {
        Self { dict: HashMap::new() }
    }

    /// Define a native word
    pub fn native(&mut self, word: &str, func: fn(&mut Stack, &mut Concat, &mut Dictionary, &mut RetStack)) {
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
/// TODO: Custom object
pub struct Object {

}

/// Integer type alias
pub type IntegerType = i64;

/// Float type alias
pub type FloatType = f64;

#[derive(Clone, Debug)]
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
    //Object(Box<Object>),
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

/// Concatenation, the array of words that conforms the program.
pub struct Concat {
    array: Vec<Cell>,
    pointer: usize,
}

impl Concat {
    fn new() -> Self {
        Self { array: Vec::new(), pointer: 0 }
    }

    /// Set pointer to position
    pub fn go_to(&mut self, pos: usize) {
        self.pointer = pos;
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

    /// Get current pointer position
    pub fn pointer(&self) -> usize {
        self.pointer
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
    /// Create new stack
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

/// RunPack interpreter
pub struct Script<T: Iterator<Item=u8> + Sized> {
    stack: Stack,
    dictionary: Dictionary,
    ret: RetStack,
    concat: Concat,
    reader: T,
}

impl<T: Iterator<Item=u8> + Sized> Script<T> {
    pub fn new(reader: T) -> Self {
        let mut script = Self {
            stack: Stack::new(),
            dictionary: Dictionary::new(),
            ret: RetStack::new(),
            concat: Concat::new(),
            reader,
        };
        script.tokenize();
        script.def_natives(&[
            ("(", open_parenth), (")", close_parenth), ("{", open_curly), ("}", close_curly), ("def", def), ("+", plus), ("-", minus), ("*", star), ("/", slash), ("%", percent),
        ]);
        script
    }

    fn next_cell(&mut self) -> Cell {
        let mut word_found = false;
        let mut in_string = false;
        let mut last_was_escape = false;
        let mut buff = Vec::new();

        //TODO: comments
        while let Some(b) = self.reader.next() {
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
            else {
                if b == 39 {    // apostrophe
                    in_string = true;
                    word_found = true;
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

        if buff.len() > 0 {
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
                self.parse_token(buff)
            }
        }
        else {
            Cell::Empty
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
    pub fn def_natives(&mut self, list: &[(&str, fn(&mut Stack, &mut Concat, &mut Dictionary, &mut RetStack))]) {
        list.iter().for_each(|(word_name, function)| {
            self.dictionary.native(word_name, *function);
        });
    }

    //TODO: append: parse another piece of program into the same script space. Is appened at the end of the Concat.
    //TODO: run a piece a code directly: appends and runs from the pos of the new code

    /// Run the script
    pub fn run(&mut self) {
        while let Some(cell) = self.concat.next() {
            let cell = cell.clone();
            match cell {
                Cell::Integer(_) | Cell::Float(_) | Cell::Boolean(_) | Cell::Symbol(_) | Cell::String(_) => self.stack.push(cell),
                Cell::Word(w) => {
                    if let Some(dict_entry) = self.dictionary.dict.get(&w) {
                        match dict_entry {
                            DictEntry::Native(func) => {
                                func(&mut self.stack, &mut self.concat, &mut self.dictionary, &mut self.ret);
                            },
                            DictEntry::Defined(block_ref) => {
                                self.ret.push(self.concat.pointer());
                                self.concat.go_to(block_ref.pos);
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
                _ => { panic!("Found an invalid cell value in the Concat: {:?}", cell) },
            }
        }
    }

    /// Get mutable ref to ductionary
    pub fn dictionary(&mut self) -> &mut Dictionary {
        &mut self.dictionary
    }
}

// Primitives

fn open_parenth(stack: &mut Stack, _: &mut Concat, _: &mut Dictionary, _: &mut RetStack) {
    stack.start_stack();
}

fn close_parenth(stack: &mut Stack, _: &mut Concat, _: &mut Dictionary, _: &mut RetStack) {
    if let None = stack.end_stack() {
        panic!("close_parenth: Stack level undeflow");
    }
}

fn open_curly(stack: &mut Stack, concat: &mut Concat, _: &mut Dictionary, _: &mut RetStack) {
    let pos = concat.pointer();
    loop {
        if let Some(cell) = concat.next() {
            if let Cell::Word(w) = cell {
                if w == "}" {
                    let len = concat.pointer() - pos;
                    stack.push(Cell::Block(BlockRef { pos, len }));
                    break;
                }
            }
        }
        else {
            panic!("open_curly: Reached the end and didn't find a closing block");
        }
    }
}

fn close_curly(_: &mut Stack, concat: &mut Concat, _: &mut Dictionary, ret: &mut RetStack) {
    if let Some(pos) = ret.pop() {
        concat.go_to(pos);
    }
    else {
        panic!("close_curly: Return stack underflow");
    }
}

//TODO: Podem definir paraules sense emprar el Concat: #twice { 2 * } def#

fn def(stack: &mut Stack, concat: &mut Concat, dict: &mut Dictionary, _: &mut RetStack) {
    let (data, word) = (stack.pop(), concat.next());
    if let Some(Cell::Word(word)) = word {
        if let Some(Cell::Block(block)) = data{
            dict.block(word, block);
        }
        else if let Some(cell) = data {
            dict.data(word, cell);
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
    let (cell_a, cell_b) = (stack.pop(), stack.pop());
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

fn plus(stack: &mut Stack, _: &mut Concat, _: &mut Dictionary, _: &mut RetStack) {
    two_num_op(stack, |a, b| a + b, |a, b| a + b);
}

fn minus(stack: &mut Stack, _: &mut Concat, _: &mut Dictionary, _: &mut RetStack) {
    two_num_op(stack, |a, b| a - b, |a, b| a - b);
}

fn star(stack: &mut Stack, _: &mut Concat, _: &mut Dictionary, _: &mut RetStack) {
    two_num_op(stack, |a, b| a * b, |a, b| a * b);
}

fn slash(stack: &mut Stack, _: &mut Concat, _: &mut Dictionary, _: &mut RetStack) {
    two_num_op(stack, |a, b| a / b, |a, b| a / b);
}

fn percent(stack: &mut Stack, _: &mut Concat, _: &mut Dictionary, _: &mut RetStack) {
    two_num_op(stack, |a, b| a % b, |a, b| a % b);
}
