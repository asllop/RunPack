
/// Integer type alias
pub type IntegerType = i64;

/// Float type alias
pub type FloatType = f64;

/// Data primitive
#[derive(Debug)]
pub enum Cell {
    Empty,
    Integer(IntegerType),
    Float(FloatType),
    Boolean(bool),
    Symbol(String),
    String(String),
    Word(String),
    //TODO: object
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

/// Concatenation of words
pub struct Concat<T: Iterator<Item=u8> + Sized> {
    array: Vec<Cell>,
    reader: T,
}

impl<T: Iterator<Item=u8> + Sized> Concat<T> {
    /// Create Concat using a u8 iterator
    pub fn new(reader: T) -> Self {
        Self {
            array: Vec::new(),
            reader
        }
    }

    fn next_cell(&mut self) -> Cell {
        //TODO: parse next entity and convert into cell
        let mut word_found = false;
        let mut in_string = false;
        let mut last_was_escape = false;
        let mut buff = Vec::new();

        while let Some(b) = self.reader.next() {
            if in_string {
                if b == 92 {    // backslash
                    last_was_escape = !last_was_escape;
                    if !last_was_escape {
                        buff.push(b);
                    }
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
            Cell::Empty
        }
    }
    
    /// Tokenize and parse
    pub fn tokenize(&mut self) {
        loop {
            let cell = self.next_cell();
            if let Cell::Empty = cell {
                break;
            }
            else {
                self.array.push(cell);
            }
        }

        println!("Array of tokens = {:?}", self.array);
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
