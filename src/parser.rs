use std::collections::HashMap;

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

const APPLICATIVE_BP: u8 = 100;
const MULTIPLICATIVE_BP: u8 = 20;
const ADDITIVE_BP: u8 = 10;
const TERM_BP: u8 = 0;

#[derive(Debug, Clone)]
struct OperatorInfo {
    lbp: u8,
    rbp: u8,
    assoc: Assoc,
}

#[derive(Debug, Clone)]
enum Assoc {
    L,
    R,
}

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    fixities: HashMap<String, OperatorInfo>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let mut fixities = HashMap::new();
        fixities.insert(
            "+".to_string(),
            OperatorInfo {
                lbp: ADDITIVE_BP,
                rbp: ADDITIVE_BP + 1,
                assoc: Assoc::L,
            },
        );
        fixities.insert(
            "*".to_string(),
            OperatorInfo {
                lbp: MULTIPLICATIVE_BP,
                rbp: MULTIPLICATIVE_BP + 1,
                assoc: Assoc::L,
            },
        );
        Self { lexer, fixities }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.pratt_parse(0)
    }

    fn pratt_parse(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        if let Some(kind) = self.lexer.next() {
            let mut lhs = self.nud(kind)?;
            while let Some(t) = self.lexer.peek().cloned() {
                let lbp = self.lbp(&t);
                if lbp <= min_bp {
                    break;
                }
                if let Some(op) = self.lexer.next() {
                    lhs = self.led(lhs, op)?;
                }
            }
            Ok(lhs)
        } else {
            Err(ParseError::EarlyEof)
        }
    }

    // This basically corresponds to app and terms
    fn nud(&mut self, kind: Kind) -> Result<Expr, ParseError> {
        match kind {
            Kind::Let => self.parse_let(),
            Kind::If => self.parse_if(),
            Kind::Fun => self.parse_fun(),
            Kind::TokInt(i) => Ok(Expr::Literal(Literal::Int(i))),
            Kind::TokId(name) if self.fixities.contains_key(&name) => {
                let op = name.clone();
                let op_info = self.fixities[&op].clone();
                let rhs = self.pratt_parse(op_info.rbp)?;
                Ok(Expr::App(Box::new(Expr::Id(op)), Box::new(rhs)))
            }
            // Yeah this is bad tbf
            Kind::TokId(name) => Ok(Expr::Id(name)),

            Kind::LParen => {
                // WARNING: Need check if next token is RParen.
                let expr = self.pratt_parse(0);
                self.lexer.next();
                expr
            }

            _ => Err(ParseError::Unexpected(
                vec![Kind::TokId("".to_string())],
                kind,
            )),
        }
    }

    fn led(&mut self, lhs: Expr, kind: Kind) -> Result<Expr, ParseError> {
        if let Kind::TokId(i) = kind {
            if let Some(info) = self.fixities.get(&i) {
                let rhs = self.pratt_parse(info.rbp)?;
                // "x plus y" → "((plus x) y)"
                Ok(Expr::App(
                    Box::new(Expr::App(Box::new(Expr::Id(i)), Box::new(lhs))),
                    Box::new(rhs),
                ))
            } else {
                panic!("identifier {:?} is not an infix operator", i);
            }
        } else {
            Err(ParseError::Unexpected(
                vec![kind],
                Kind::TokId("".to_string()),
            ))
        }
    }

    fn lbp(&self, kind: &Kind) -> u8 {
        if let Kind::TokId(name) = kind {
            if let Some(opinfo) = self.fixities.get(name) {
                return opinfo.lbp;
            }
        }
        0
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
