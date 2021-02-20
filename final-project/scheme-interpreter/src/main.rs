// use logos::Logos;
mod parser;
mod interpreter;
use parser::Parser;
use interpreter::Environment;
use std::io;
use std::fs;
use regex::Regex;

fn is_exit_input( string: &str ) -> bool {
    let re = Regex::new( r"^ *\( *exit *\) *$" ).unwrap();
    re.is_match( string.trim() )
}


fn load_file( input: String ) -> String {
    let re = Regex::new( "^ *\\( *load \"([^\\(\\)]+)\" *\\) *$" ).unwrap();
    if let Some( capts ) = re.captures( input.as_str().trim() ) {
        if capts.len() == 2 {
            let file_name = capts.get( 1 ).map_or( "", |m| m.as_str() );
            let result = fs::read_to_string( file_name );
            if let Ok( content ) = result {
                content
            }
            else {
                println!( "open-input-file: cannot open input file\n  file: `{}`", file_name );
                input
            }
        }
        else {
            input
        }
    }
    else {
        input
    }
}


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

        if is_exit_input( input.as_str() ) {
            break;
        }

        input = load_file( input );

        parser.load( input.as_str() );

        for data in &mut parser {
            println!( "{}", environment.eval( &data ) );
        }
    }

}
