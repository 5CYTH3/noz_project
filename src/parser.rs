use crate::lexer::{Kind, Lexer};

#[derive(Debug)]
pub enum ParseError {
    Unexpected(Vec<Kind>, Kind),
    EarlyEof,
}

// WHAT IS NEEDED HERE: ARENA ALLOCATION IN A LL(*) PARSER
#[derive(Debug)]
pub enum Expr {
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
        ifx: bool, // WARNING: if ifx is set to true, args should not be of length > 2
        args: Vec<String>,
        body: Box<Expr>,
    },
    App(Box<Expr>, Box<Expr>), // Function, Param(s)
    Id(String),
    Literal(Literal),
    Grouped(Box<Expr>),
}

#[derive(Debug)]
pub enum Definition {
    TypeDef(TypeDef),
    ExprDef {
        explicit_type: Option<TypeDef>,
        body: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum TypeDef {
    TypeId(String),
    ProductType(Vec<(String, String)>),
    SumType(Vec<TypeDef>),
    FunctionType(Box<TypeDef>, Box<TypeDef>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Str(String),
    Bool(bool),
    Char(char),
}

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        if let Some(nxt) = self.lexer.next() {
            match nxt {
                Kind::Let => self.parse_let(),
                Kind::TokInt(i) => Ok(Expr::Literal(Literal::Int(i))), // temp
                Kind::If => self.parse_if(),
                Kind::Fun => self.parse_fun(),
                _ => todo!(),
            }
        } else {
            panic!("parse_expr: if next is None")
        }
    }

    fn parse_let(&mut self) -> Result<Expr, ParseError> {
        if let Some(Kind::TokId(id)) = self.lexer.next() {
            let def = self.parse_def()?;
            match self.lexer.next() {
                Some(Kind::In) => {
                    let _in = self.parse_expr()?;
                    Ok(Expr::Let {
                        id,
                        def,
                        in_expr: Box::from(_in),
                    })
                }
                Some(t) => Err(ParseError::Unexpected(vec![Kind::In], t)),
                None => Err(ParseError::EarlyEof),
            }
        } else {
            Err(ParseError::Unexpected(vec![], Kind::TokId("".to_string())))
        }
    }

    fn parse_if(&mut self) -> Result<Expr, ParseError> {
        let predicate: Expr = self.parse_expr()?;
        if let Some(Kind::Then) = self.lexer.next() {
            let fulfilled: Expr = self.parse_expr()?;
            if let Some(Kind::Else) = self.lexer.next() {
                let unfulfilled = self.parse_expr()?;
                Ok(Expr::If {
                    predicate: Box::from(predicate),
                    fulfilled: Box::from(fulfilled),
                    unfulfilled: Box::from(unfulfilled),
                })
            } else {
                Err(ParseError::EarlyEof)
            }
        } else {
            Err(ParseError::EarlyEof)
        }
    }

    // If the <name> field is Some(_), it means the anonymous function is bound to a let-binding
    // allowing the parser to edit the fixity table.

    fn parse_fun(&mut self) -> Result<Expr, ParseError> {
        let mut is_infix: bool = false;
        if let Some(Kind::Ifx) = self.lexer.peek() {
            self.lexer.next();
            is_infix = true;
        }

        let args: Vec<String> = self.parse_args();

        match self.lexer.next() {
            Some(Kind::Arrow) => {
                let body = self.parse_expr()?;
                Ok(Expr::Function {
                    ifx: is_infix,
                    args,
                    body: Box::from(body),
                })
            }
            Some(t) => Err(ParseError::Unexpected(vec![Kind::Arrow], t)),
            None => Err(ParseError::EarlyEof),
        }
    }

    fn parse_args(&mut self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();
        while let Some(Kind::TokId(i)) = self.lexer.peek() {
            println!("{}", i);
            args.push(i.clone());
            self.lexer.next();
        }
        args
    }

    fn parse_def(&mut self) -> Result<Definition, ParseError> {
        if let Some(t) = self.lexer.next() {
            match t {
                Kind::DoubleColon => {
                    let def = self.parse_type_def()?;
                    Ok(Definition::TypeDef(def))
                }
                Kind::Colon => {
                    let t: TypeDef = self.parse_type_def()?;
                    // TODO: Collapse the if-let and the match into one match statement
                    if let Some(tok) = self.lexer.next() {
                        match tok {
                            Kind::Eq => {
                                let body = self.parse_expr()?;
                                Ok(Definition::ExprDef {
                                    explicit_type: Some(t),
                                    body: Box::from(body),
                                })
                            }
                            t => Err(ParseError::Unexpected(vec![Kind::Eq], t)),
                        }
                    } else {
                        Err(ParseError::EarlyEof)
                    }
                }
                Kind::Eq => {
                    // body should be matched to detect if a function has been defined. If it is
                    // the case, the `fixity` field should be modified accordingly
                    let body = self.parse_expr()?;
                    Ok(Definition::ExprDef {
                        explicit_type: None,
                        body: Box::from(body),
                    })
                }
                t => Err(ParseError::Unexpected(
                    vec![Kind::DoubleColon, Kind::Colon, Kind::Eq],
                    t,
                )),
            }
        } else {
            Err(ParseError::EarlyEof)
        }
    }

    fn parse_type_def(&mut self) -> Result<TypeDef, ParseError> {
        if let Some(t) = self.lexer.next() {
            match t {
                // NOTE: Type name is just a special case of function type. The grammar rule
                // is simply S -> T ['->' S]
                Kind::Pipe => todo!(),
                Kind::LBracket => todo!(),
                _ => todo!(),
            }
        } else {
            Err(ParseError::EarlyEof)
        }
    }

    // INFO: Should be using PRATT parsing (infix arrow operator)
    // fn parse_fun_type(&mut self) -> Result<TypeDef, ParseError> {}
}
