use lexer::Lexer;
use parser::Parser;

pub mod lexer;
pub mod parser;

fn main() {
    let mut lexer = Lexer::new("let x = fun a b -> a in a + b * c");
    let mut parser = Parser::new(&mut lexer);
    let result = parser.parse();
    println!("{:#?}", result)
}
