
/// Max size of a word name
pub const NAME_SIZE: usize = 32;

/// Word name type
pub type WordName = [u8; NAME_SIZE];

/// Word struct, a WordName plus length
#[derive(Debug)]
pub struct Word {
    word_name: WordName,
    len: u8,
}

impl Word {
    /// Create word from parts
    pub fn from_parts(word_name: WordName, len: u8) -> Self {
        Self {
            word_name,
            len
        }
    }

    /// Create word from a str
    pub fn new(name: &str) -> Word {
        let mut word_name = WordName::default();
        for (i, b) in name.as_bytes().into_iter().enumerate() {
            if i >= NAME_SIZE {
                break;
            }
            word_name[i] = *b;
        }
        let name_len = core::cmp::min(name.as_bytes().len(), NAME_SIZE) as u8;
        Self { word_name, len: name_len }
    }

    /// Word name
    pub fn name(&self) -> WordName {
        self.word_name
    }

    /// Word name size
    pub fn len(&self) -> u8 {
        self.len
    }
}

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
    Symbol(Word),
    String(String),
    Function(Word),
    //TODO: object
    //Block(Vec<Word>),
}

impl Cell {
    //TODO: build cell from all other types

    pub fn number(word: Word) -> Option<Self> {
        // Safety note: we assume that the source code is a well formed UTF-8 text to avoid slow checks.
        let word_name_str = unsafe {
            let arr = core::slice::from_raw_parts(word.name().as_ptr(), word.len() as usize);
            core::str::from_utf8_unchecked(arr)
        };
        //IMPROVEMENT: find a faster way to parse a number, in a single pass
        if let Ok(int) = word_name_str.parse::<IntegerType>() {
            Some(Cell::Integer(int))
        }
        else if let Ok(flt) = word_name_str.parse::<FloatType>() {
            Some(Cell::Float(flt))
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

    pub fn next_cell(&mut self) -> Cell {
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
                // Found a word separator (comma, space or any control character)
                else if b == 44 || b <= 32 {
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
            let token = std::str::from_utf8(&buff).expect("Error parsing UTF-8 string");
            println!("Token found = {}", token);
            if in_string {
                Cell::String(String::from(token))
            }
            else {
                //TEST
                Cell::Boolean(true)
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

        println!("Array of token = {:?}", self.array);
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
