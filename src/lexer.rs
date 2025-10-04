// WHAT IS NEEDED HERE: SIMD WHITESPACE SKIPPING, DFA-BASED LEXER

// SOON: IMPERATIVE PROGRAMMING (blocs, unit type, instruction)
#[derive(Clone, Debug)]
pub enum Kind {
    LParen,
    RParen,
    Let, // kw
    In,  // kw
    DoubleColon,
    Colon,
    Semicolon,
    Eq,
    LBracket,
    RBracket,
    Pipe,
    Fun, // kw
    Ifx, // kw, must be attached to two arguments functions
    Arrow,
    If,   // kw
    Then, // kw
    Else, // kw
    TokId(String),
    TokBool(bool),
    TokStr(String),
    TokChar(char),
    TokInt(i32),
}

// Need push-back linked list?

pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
    bytes: &'a [u8],
    peeked: Option<Kind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            cursor: 0,
            bytes: input.as_bytes(),
            peeked: None,
        }
    }

    fn next_byte(&mut self) -> Option<u8> {
        if self.cursor < self.bytes.len() {
            let b = self.bytes[self.cursor];
            self.cursor += 1;
            Some(b)
        } else {
            None
        }
    }

    pub fn peek_byte(&self) -> Option<u8> {
        self.bytes.get(self.cursor).copied()
    }

    pub fn peek(&mut self) -> Option<Kind> {
        if self.peeked.is_none() {
            self.peeked = self.next();
        }
        self.peeked.clone()
    }
}

// TODO: Implement strings, bool and chars recognition.
// Or maybe should `bool` be defined like would `+` be in core ?
impl<'a> Iterator for Lexer<'a> {
    type Item = Kind;

    fn next(&mut self) -> Option<Kind> {
        while matches!(self.peek_byte(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.cursor += 1;
        }

        let s = self.cursor;

        match self.next_byte() {
            Some(b'(') => Some(Kind::LParen),
            Some(b')') => Some(Kind::RParen),
            Some(b':') => match self.peek_byte() {
                Some(b':') => {
                    self.cursor += 1;
                    Some(Kind::DoubleColon)
                }
                _ => Some(Kind::Colon),
            },
            Some(b';') => Some(Kind::Semicolon),
            Some(b'=') => Some(Kind::Eq),
            Some(b'{') => Some(Kind::LBracket),
            Some(b'}') => Some(Kind::RBracket),
            Some(b'|') => Some(Kind::Pipe),
            Some(b'-') => {
                if self.peek_byte() == Some(b'>') {
                    self.cursor += 1;
                    Some(Kind::Arrow)
                } else {
                    Some(Kind::TokId("-".to_string()))
                }
            }
            Some(c) if is_number(c) => {
                while let Some(&b) = self.bytes.get(self.cursor) {
                    if is_number(b) {
                        self.cursor += 1;
                    } else {
                        break;
                    }
                }
                let slice = &self.input[s..self.cursor];
                let i: i32 = slice.parse().unwrap();
                return Some(Kind::TokInt(i));
            }
            Some(c) if is_id_start(c) => {
                while let Some(&b) = self.bytes.get(self.cursor) {
                    if is_id_continue(b) {
                        self.cursor += 1
                    } else {
                        break;
                    }
                }
                let slice = &self.input[s..self.cursor];
                match slice {
                    "ifx" => Some(Kind::Ifx),
                    "let" => Some(Kind::Let),
                    "if" => Some(Kind::If),
                    "then" => Some(Kind::Then),
                    "else" => Some(Kind::Else),
                    "fun" => Some(Kind::Fun),
                    "in" => Some(Kind::In),
                    _ => Some(Kind::TokId(slice.to_string())),
                }
            }
            None => None,
            Some(c) => panic!("Unrecognized token: {}.", char::from(c)),
        }
    }
}

fn is_number(b: u8) -> bool {
    (b'1'..=b'9').contains(&b)
}

// Starts with a letter, rest can either be a letter or a number
fn is_id_start(b: u8) -> bool {
    (b'a'..=b'z').contains(&b) || matches_sym(b) || (b'A'..=b'Z').contains(&b) || b == b'_'
}

// Is only composed of the characters - + * & ^ % $ # @ ! < > ? / \ ~
// BUT can only start by one of + * & ^ $ # @ ! < > ? / \ ~
// WARNING: Operators might already be defined in `core` and cannot be overridden.
fn matches_sym(b: u8) -> bool {
    matches!(
        b,
        b'!' | b'#'
            | b'$'
            | b'%'
            | b'&'
            | b'*'
            | b'+'
            | b'-'
            | b'/'
            | b'\\'
            | b'<'
            | b'>'
            | b'?'
            | b'@'
            | b'^'
            | b'~'
    )
}

fn is_id_continue(b: u8) -> bool {
    is_id_start(b) || matches!(b, b'-') || (b'0'..=b'9').contains(&b)
}
