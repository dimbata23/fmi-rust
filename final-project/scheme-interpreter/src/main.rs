mod parser;
mod interpreter;
use parser::*;
use interpreter::*;
use std::io;
use std::fs;
use regex::Regex;


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






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_basics() {
        let mut parser = Parser::new();
        parser.load( "       0  \r\n    1   \r   2  \n 3   \t4" );
        for ( i, data ) in (&mut parser).enumerate() {
            assert_eq!( data, Data::from_string( DataType::Integer, i.to_string() ) );
        }

        parser.load( " (         define    x 5.0)          " );
        assert_eq!( parser.next().unwrap(), Data::from_list( vec![ Data::from_string( DataType::Variable, "define".to_string() ), Data::from_string( DataType::Variable, "x".to_string() ), Data::from_string( DataType::Real, "5.0".to_string() ) ] ) );
    
        parser.load( "" );
        assert!( parser.next().is_none() );
    }

    #[test]
    fn test_eval_basics() {
        let mut parser      = Parser::new();
        let mut environment = Environment::new();
        parser.load( "+ car (define x 5.0) x (x) (define (1+ x) (+ x 1)) (1+ 5) (if (<= 2 3) 1.0 0) #t #f (null? '())" );
        
        assert_eq!( environment.eval( &parser.next().unwrap() ), Data::from_string( DataType::Procedure, "+".to_string() ) );
        assert_eq!( environment.eval( &parser.next().unwrap() ), Data::from_string( DataType::Procedure, "car".to_string() ) );
        assert_eq!( environment.eval( &parser.next().unwrap() ), Data::from_string( DataType::Real, "5.0".to_string() ) );
        assert_eq!( environment.eval( &parser.next().unwrap() ), Data::from_string( DataType::Real, "5.0".to_string() ) );
        assert_eq!( environment.eval( &parser.next().unwrap() ), INVALID_DATA );
        assert!( !is_invalid_data( &environment.eval( &parser.next().unwrap() ) ) );
        assert_eq!( environment.eval( &parser.next().unwrap() ), Data::from_string( DataType::Integer, "6".to_string() ) );
        assert_eq!( environment.eval( &parser.next().unwrap() ), Data::from_string( DataType::Real, "1.0".to_string() ) );
        assert_eq!( environment.eval( &parser.next().unwrap() ), new_true_sym() );
        assert_eq!( environment.eval( &parser.next().unwrap() ), new_false_sym() );
        assert_eq!( environment.eval( &parser.next().unwrap() ), new_true_sym() );
    }
}
