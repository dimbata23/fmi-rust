// use logos::Logos;
mod parser;
mod interpreter;
use parser::Parser;
use interpreter::Environment;
use std::io;

fn main() {

    let mut environment = Environment::new();
    let mut parser      = Parser::new();
    let mut input       = String::new();
    loop {
        input.clear();
        if let Err( e ) = io::stdin().read_line( &mut input ) {
            println!( "{}", e );
            break;
        }

        parser.load( input.as_str() );

        for data in &mut parser {
            println!( "{}", environment.eval( &data ) );
        }
    }

}
