use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex( r"[\-\+]?[0-9]+", |lex| lex.slice().to_string() )]
    Int( String ),

    #[regex( r"[\-\+]?([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)", |lex| lex.slice().to_string() )]
    Real( String ),

    #[regex( r"([a-zA-Z]+([a-zA-Z\*<=>!\?:\$%_&~\^\+\-\.]|[0-9])*|([0-9]+[a-zA-Z\*<=>!\?:\$%_&~\^\+\-\.]+)*|#'*[a-zA-Z]*)", |lex| lex.slice().to_string())]
    Identifier( String ),

    #[regex( "\".*\"", |lex| lex.slice().to_string() )]
    String( String ),

    #[token( "(", |lex| lex.slice().to_string() )]
    OpenBracket( String ),

    #[token( ")", |lex| lex.slice().to_string() )]
    CloseBracket( String ),

    #[token( "'", |lex| lex.slice().to_string() )]
    Quote( String ),

    #[error]
    #[regex( r"[ \t\n\f]+", logos::skip )]
    Error,
}
