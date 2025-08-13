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

fn parse(l: Lexer) -> Result<Expr, ParseError> {}
