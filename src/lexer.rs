// WHAT IS NEEDED HERE: SIMD WHITESPACE SKIPPING, DFA-BASED LEXER

// SOON: IMPERATIVE PROGRAMMING (blocs, unit type, instruction)
enum Kind {
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
    EOF,
}

// Need push-back linked list?

pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
    bytes: &'a [u8],
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            input,
            cursor: 0,
            bytes: input.as_bytes(),
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

    fn peek_byte(&self) -> Option<u8> {
        self.bytes.get(self.cursor).copied()
    }

    fn next(&mut self) -> Kind {
        while self.peek_byte() == Some(b' ' | b'\t' | b'\n' | b'\r') {
            self.cursor += 1;
        }

        let s = self.cursor;

        match self.next_byte() {
            Some(b'(') => Kind::LParen,
            Some(b')') => Kind::RParen,
            Some(b':') => match self.peek_byte() {
                Some(b':') => {
                    self.cursor += 1;
                    Kind::DoubleColon
                }
                _ => Kind::Colon,
            },
            Some(b';') => Kind::Semicolon,
            Some(b'=') => Kind::Eq,
            Some(b'{') => Kind::LBracket,
            Some(b'}') => Kind::RBracket,
            Some(b'|') => Kind::Pipe,
            Some(b'-') => {
                if self.peek_byte() == Some(b'>') {
                    self.cursor += 1;
                    Kind::Arrow
                } else {
                    Kind::TokId("-".to_string())
                }
            }
            Some(c) if is_alphanum_id_start(c) => {
                while let Some(&b) = self.bytes.get(self.cursor) {
                    if is_alphanum_id_continue(b) {
                        self.cursor += 1
                    } else {
                        break;
                    }
                }
                let slice = &self.input[s..self.cursor];
                match slice {
                    "ifx" => Kind::Ifx,
                    "let" => Kind::Let,
                    "if" => Kind::If,
                    "then" => Kind::Then,
                    "else" => Kind::Else,
                    "fun" => Kind::Fun,
                    "in" => Kind::In,
                    _ => Kind::TokId(slice.to_string()),
                }
            }
            Some(c) if is_symbol_id_start(c) => {
                while let Some(&b) = self.bytes.get(self.cursor) {
                    if is_symbol_id_continue(b) {
                        self.cursor += 1
                    } else {
                        break;
                    }
                }
                Kind::TokId(self.input[s..self.cursor].to_string())
            }
            None => Kind::EOF,
            Some(_) => panic!("Unrecognized token: {}.", s),
        }
    }
}

// Starts with a letter, rest can either be a letter or a number
fn is_alphanum_id_start(b: u8) -> bool {
    (b'a'..=b'z').contains(&b) || (b'A'..=b'Z').contains(&b) || b == b'_'
}

fn is_alphanum_id_continue(b: u8) -> bool {
    is_alphanum_id_start(b) || (b'0'..=b'9').contains(&b)
}

// Is only composed of the characters - + * & ^ % $ # @ ! < > ? / \ ~
// BUT can only start by one of + * & ^ $ # @ ! < > ? / \ ~
// WARNING: Operators might already be defined in `core` and cannot be overridden.
fn is_symbol_id_start(b: u8) -> bool {
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

fn is_symbol_id_continue(b: u8) -> bool {
    is_symbol_id_start(b) || matches!(b, b'-')
}
