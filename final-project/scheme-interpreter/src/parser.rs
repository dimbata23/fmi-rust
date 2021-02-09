use crate::interpreter;
use logos::Logos;
use interpreter::Data;
use interpreter::DataType;
use interpreter::NULL_SYM;

#[derive( Logos, Debug, PartialEq )]
pub enum Token {
    #[regex( r"[\-\+]?[0-9]+", |lex| lex.slice().to_string() )]
    Int( String ),

    #[regex( r"[\-\+]?([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)", |lex| lex.slice().to_string() )]
    Real( String ),

    #[regex( r"(([a-zA-Z\*<=>!\?:\$%_&~\^\+\-\.]+[0-9]*)+|([0-9]+[a-zA-Z\*<=>!\?:\$%_&~\^\+\-\.]+)*|#'*[a-zA-Z]*)", |lex| lex.slice().to_string())]
    Identifier( String ),

    #[regex( "\".*\"", |lex| lex.slice().to_string() )]
    String( String ),

    #[token( "(", |lex| lex.slice().to_string() )]
    OpenBracket( String ),

    #[token( ")", |lex| lex.slice().to_string() )]
    CloseBracket( String ),

    #[token( "'", |lex| lex.slice().to_string() )]
    Quote( String ),

    #[regex( r"[ \t\n\f]+", logos::skip )]
    Skip,

    #[error]
    Error,
}


pub struct Parser {
    tokens_arr  : Vec< Token >,
    index       : usize,
}


impl Parser {

    pub fn new() -> Parser {
        Parser { tokens_arr: vec![], index: 0 }
    }

    pub fn load( &mut self, input_str: &str ) {

        let mut lex = Token::lexer( input_str );

        self.tokens_arr.clear();
        self.index = 0;
        let mut data = lex.next();
        while !data.is_none() {
            self.tokens_arr.push( data.unwrap() );
            data = lex.next();
        }
    }


    fn parse_next( &mut self, quote_level: u16 ) -> Option< Data > {

        if self.index >= self.tokens_arr.len() {
            return None;
        }

        match &self.tokens_arr[ self.index ] {
            Token::Int( data )          => Some( Data::from_string_quoted( DataType::Integer,   data, quote_level ) ),
            Token::Real( data )         => Some( Data::from_string_quoted( DataType::Real,      data, quote_level ) ),
            Token::Identifier( data )   =>
                Some(
                    Data::from_string_quoted(
                        if quote_level == 0 { DataType::Variable } else { DataType::Symbol }
                        , data
                        , quote_level
                    )
                ),
            Token::Quote( _ )        => { self.index += 1; self.parse_next( quote_level + 1 ) }
            Token::String( data )       => Some( Data::from_string_quoted( DataType::Symbol,    data, quote_level ) ),
            Token::OpenBracket( _ )  => {

                let mut res_list = Data::new_list();
                if quote_level > 0 {
                    res_list.data_type          = DataType::Symbol;
                    res_list.quote_level    = quote_level;
                }

                self.index += 1;

                while self.index < self.tokens_arr.len() {
                    if let Token::CloseBracket(_) = self.tokens_arr[ self.index ] {
                        break;
                    }

                    let res = self.parse_next( quote_level );
                    if res.is_none() {
                        return None;
                    }
                    res_list.list.push( res.unwrap() );

                    if self.index >= self.tokens_arr.len() {
                        return None;
                    }

                    self.index += 1;
                }

                if self.index == self.tokens_arr.len() /*|| let Token::CloseBracket(_) = self.fTokens[ self.fIndex ]*/ {
                    //self.print_error( Error::ReadSyntax, "expected a `)` to close `(`" );
                    // TODO: Better error
                    println!( "Expected a `)` to close `(`" );
                    return None;
                }

                if quote_level == 1 && res_list.list.is_empty() {
                    return Some( NULL_SYM );
                }

                if let DataType::Symbol = res_list.data_type {
                    if res_list.list.is_empty() {
                        res_list.list.push( NULL_SYM );
                    }
                }

                Some( res_list )
            },

            Token::CloseBracket( _ )    => {
                // PrintError( Error::READ_SYNTAX, "unexpected `)`" );
                // TODO: Better error
                println!( "Unexpected `)`" );
                None
            },

            Token::Error                => {
                // PrintError( Error::READ_SYNTAX, "unknown parsing error occured..." );
                // TODO: Better error
                println!( "Unknown parsing error occured..." );
                None
            }

            Token::Skip                 => {
                // unreachable
                assert!( false );
                None
            }
        }

    }

}


impl Iterator for Parser {

    type Item = Data;
    
    fn next( &mut self ) -> Option< Data > {
        let res = self.parse_next( 0 );
        self.index += 1;
        res
    }

}
