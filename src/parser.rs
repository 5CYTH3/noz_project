use crate::lexer::{Kind, Lexer};

struct ParseError(String);

// WHAT IS NEEDED HERE: ARENA ALLOCATION IN A LL(*) PARSER
enum Expr {
    Let {
        id: String,
        def: Definition,
        in_expr: Box<Expr>,
    },
    If {
        predicate: Box<Expr>,
        fulfilled: Box<Expr>,
        unfulfilled: Box<Expr>,
    },
    Function {
        ifx: bool,
        args: Vec<String>,
        body: Box<Expr>,
    },
    App(Box<Expr>, Box<Expr>), // Function, Param(s)
    Id(String),
    Literal(Literal),
    Grouped(Box<Expr>),
}

enum Definition {
    TypeDef(TypeDef),
    ExprDef {
        explicit_type: Option<String>,
        body: Box<Expr>,
    },
}

enum TypeDef {
    TypeId(String),
    ProductType(Vec<(String, String)>),
    SumType(Vec<TypeDef>),
    FunctionType(Box<TypeDef>, Box<TypeDef>),
}

enum Literal {
    Int(i32),
    Str(String),
    Bool(bool),
    Char(char),
}

struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Expr {
        let expr = self.parse_expr();
        expr
    }

    fn parse_expr(&mut self) -> Expr {
        if let Some(nxt) = self.lexer.next() {
            match nxt {
                Kind::Let => self.parse_let(),
                Kind::If => self.parse_if(),
                Kind::Fun => self.parse_fun(),
            }
        } else {
            panic!("parse_expr: if next is None")
        }
    }

    fn parse_let(&mut self) -> Expr {
        if let Some(Kind::TokId(id)) = self.lexer.next() {
            if let Some(Kind::TokId(i)) = self.lexer.next() {
                // Parse def
                if let Some(Kind::In) = self.lexer.next() {}
            }
        }
    }

    fn parse_if(&mut self) -> Expr {}

    fn parse_fun(&mut self) -> Expr {}

    fn parse_def(&mut self) -> Expr {
        if let Some(t) = self.lexer.next() {
            match t {
                Kind::DoubleColon => self.parse_type_expr(),
            }
        }
    }

    fn parse_type_expr(&mut self) -> Expr {}
}
