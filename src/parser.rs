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
    App(Application),
}

enum Application {
    App(Box<Expr>, Term),
    Term(Term),
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

enum Term {
    Id(String),
    Literal(Literal),
    Grouped(Box<Expr>),
}

enum Literal {
    Int(i32),
    Str(String),
    Bool(bool),
    Char(char),
}
