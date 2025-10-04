use lexer::Lexer;

pub mod lexer;
pub mod parser;

fn main() {
    let lexer = Lexer::new("let x = fun x y -> x + y");
    lexer.for_each(|x| println!("{:?}", x));
}
