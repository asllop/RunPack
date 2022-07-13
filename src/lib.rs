//TODO: use no_std, hashbrown::HashMap and alloc::vec::Vec
use std::collections::HashMap;

/// Block reference, 0: init pos in the Catvec, 1: block len
pub type BlockRef = (usize, usize);

#[derive(Debug)]
enum DictEntry {
    Native, //TODO: ref to native function
    Defined(BlockRef),
}

struct Dictionary {
    dict: HashMap<String, DictEntry>,
}

impl Dictionary {
    fn new() -> Self {
        Self { dict: HashMap::new() }
    }
}

struct RetStack {
    stack: Vec<usize>,
}

impl RetStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn push(&mut self, ret_pos: usize) {
        self.stack.push(ret_pos)
    }

    fn pop(&mut self) -> Option<usize> {
        self.stack.pop()
    }
}

#[derive(Debug)]
/// TODO: Custom object
pub struct Object {

}

/*
Definir una paraula:
    Una paraula dins el diccionari és només una pos dins el CatVec, que ens indica allà on comença el codi de la paraula. I una llargada que ens indica quantes paraules la conformen.
    Si la paraula és nadiua, és una funció.

Model d'execució:
    
    Quan trobem una dada (int, float, string, symbol, bool), la fiquem a la pila
    Quan trobem una paraula, la cerquem al diccionari i l'executem.
    Paraula nadiua:
        Guardem la pos de retorn a la pila de retorn. La pos de retorn és l'índex següent dins el CatVec.
        Executem la funció nadiua associada a la paraula.
        Quan s'acaba d'executar, obtenim adreça de return i continuem avaluant per allà.
    Paraula definida:
        Guardem la pos de retorn a la pila de retorn. La pos de retorn és l'índex següent dins el CatVec.
        Movem el punter d'execució a l'inici de la paraula dins del CatVec.
 */

/// Integer type alias
pub type IntegerType = i64;

/// Float type alias
pub type FloatType = f64;

#[derive(Debug)]
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
    /// Parse a number into a cell
    pub fn number(token: &str) -> Option<Self> {
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

    /// Parse a symbol into a cell
    pub fn symbol(token: &str) -> Option<Self> {
        if token.starts_with("#") {
            Some(Cell::Symbol(token.into()))
        }
        else {
            None
        }
    }

    /// Parse a boolean into a cell
    pub fn boolean(token: &str) -> Option<Self> {
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

/// Concatenation of words. It contains the Concatenation Vector (CatVec): the array of words that conform the program.
struct Concat {
    array: Vec<Cell>,
}

impl Concat {
    fn new() -> Self {
        Self { array: Vec::new() }
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
    pub fn new() -> Self {
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
    dict: Dictionary,
    ret: RetStack,
    catvec: Concat,
    reader: T,
}

impl<T: Iterator<Item=u8> + Sized> Script<T> {
    pub fn new(reader: T) -> Self {
        Self {
            dict: Dictionary::new(),
            ret: RetStack::new(),
            catvec: Concat::new(),
            reader,
        }
    }

    fn next_cell(&mut self) -> Cell {
        let mut word_found = false;
        let mut in_string = false;
        let mut last_was_escape = false;
        let mut buff = Vec::new();

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
    
    /// Tokenize and parse
    pub fn tokenize(&mut self) {
        loop {
            let cell = self.next_cell();
            if let Cell::Empty = cell {
                //TODO: add error handling, when the parser fails somewhere
                break;
            }
            else {
                self.catvec.array.push(cell);
            }
        }

        println!("Tokens =\n{:?}", self.catvec.array);
    }
}