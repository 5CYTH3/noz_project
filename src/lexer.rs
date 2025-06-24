// WHAT IS NEEDED HERE: SIMD WHITESPACE SKIPPING, DFA-BASED LEXER

// SOON: IMPERATIVE PROGRAMMING (blocs, unit type, instruction)
enum Kind {
    LParen,
    RParen,
    Let, // kw
    In,  // kw
    DoubleColon,
    Colon,
    Eq,
    LBracket,
    RBracket,
    Pipe,
    Ifx, // kw
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

// I suppose to recognize kws from ids, we could just have a kw_table hashtable (String, Kind) and manually enter
// all keywords of the program. If there is a match for a current sequence, it is a keyword and
// should be treated as such, else it's an id.

// Need push-back linked list

pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer { input, cursor: 0 }
    }

    fn next(&mut self) {}
}
