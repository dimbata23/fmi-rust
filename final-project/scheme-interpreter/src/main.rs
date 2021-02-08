use logos::Logos;
mod parser;

fn main() {
    let mut lex = parser::Token::lexer(r"(define x 5)");

    let mut data = lex.next();
    while !data.is_none() {
        println!( "{:?}", data.unwrap() );
        data = lex.next();
    }

    println!("Hello, world!");
}
