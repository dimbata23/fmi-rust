use std::fmt;
use std::collections::HashMap;


#[derive( Clone, PartialEq )]
pub enum DataType {
    Invalid,
    Integer,
    Real,
    Variable,
    List,
    Procedure,
    Symbol,
    Lambda,
}


pub struct Environment<'a> {
    env_data    : HashMap< String, Data >,
    parent_env  : Option< &'a Environment<'a> >
}


type ListValuesArr      = Vec< Data >;
type ProcedureArgsArr   = ListValuesArr;
type Procedure          = fn( &ProcedureArgsArr ) -> Data;

#[derive( Clone )]
pub struct Data {
    pub list        : ListValuesArr,
    pub string      : String,
    pub procedure   : Procedure,
    pub data_type   : DataType,
    pub quote_level : u16,
}


impl<'a> Environment<'a> {

    pub fn new() -> Environment<'a> {
        let mut res = Environment { env_data: HashMap::new(), parent_env: None };
        res.env_data.insert(    "#f".to_string()    , new_false_sym()       );
        res.env_data.insert(    "#t".to_string()    , new_true_sym()        );
        res.env_data.insert(    "'()".to_string()   , NULL_SYM              );
        res.add_procedure(      "+"                 , proc_add              );
        res.add_procedure(      "-"                 , proc_subtract         );
        res.add_procedure(      "*"                 , proc_multiply         );
        res.add_procedure(      "/"                 , proc_divide           );
        res.add_procedure(      "list"              , proc_list             );
        res.add_procedure(      "null?"             , proc_is_null          );
        res.add_procedure(      "pair?"             , proc_is_pair          );
        res.add_procedure(      "list?"             , proc_is_list          );
        res.add_procedure(      "string?"           , proc_is_string        );

        res
    }

    pub fn with_args( params: &ProcedureArgsArr, args: &ProcedureArgsArr, parent: &'a Environment ) -> Environment<'a> {
        let mut res = Environment { env_data: HashMap::new(), parent_env: Some( parent ) };

        for i in 0..params.len() {
            res.env_data.insert( params[ i ].string.clone(), args[ i ].clone() );
        }

        res
    }

    pub fn eval( &mut self, data: &Data ) -> Data {
        match data.data_type {
            DataType::Variable => {
                let res = self.find( data.string.as_str() );
                if let Some( x ) = res {
                    x.clone()
                }
                else {
                    print_error( Error::Undefined, data.string.as_str(), "", "");
                    INVALID_DATA
                }
            },
            DataType::Symbol => {
                if is_null_sym( data ) {
                    NULL_SYM
                }
                else {
                    data.clone()
                }
            },
            DataType::Integer | DataType::Real => {
                data.clone()
            },
            DataType::List => {
                if data.list.len() == 1 {
                    assert!( !is_null_sym( &data.list[ 0 ] ) );
                }

                if data.list.is_empty() {
                    print_error( Error::MissingProcedure, "#%app", "", "");
                    return INVALID_DATA;
                }

                if is_of_type( &DataType::Variable, &data.list[ 0 ] ) {
                    let var : &str = data.list[ 0 ].string.as_str();

                    // TODO: quote, begin

                    match var {
                        "define"    => self.eval_define( data ),
                        "lambda"    => self.eval_lambda( data ),
                        "if"        => self.eval_if( data ),
                        "cond"      => self.eval_cond( data ),
                        "apply"     => self.eval_apply( data ),
                        "map"       => self.eval_map( data ),
                        _           => self.eval_proc_lambda( data )
                    }
                }
                else {
                    self.eval_proc_lambda( data )
                }
            },
            DataType::Procedure | DataType::Lambda => {
                self.eval_proc_lambda( data )
            },
            DataType::Invalid => {
                INVALID_DATA
            }
        }
    }

    fn find( &self, variable: &str ) -> Option< &Data > {
        if !self.env_data.contains_key( variable ) {
            if self.parent_env.is_some() {
                self.parent_env.unwrap().find( variable )
            }
            else {
                None
            }
        }
        else {
            self.env_data.get( variable )
        }
    }


    fn eval_define( &mut self, data: &Data ) -> Data {
        let list_len = data.list.len();

        if list_len < 3 {
            print_error( Error::BadSyntax, "define", "at least 2 arguments needed", (list_len - 1).to_string().as_str() );
            return INVALID_DATA;
        }

        if list_len > 3 && is_of_type( &DataType::List, &data.list[ 1 ] ) {
            if list_len > 3 {
                print_error( Error::BadSyntax, "define", "exactly one expression after identifier", "");
                return INVALID_DATA;
            }
        }

        let mut res = Data::new();
        let identifier: &str;

        // Procedure definition
        let first_arg   = &data.list[ 1 ];
        if is_of_type( &DataType::List, &first_arg ) {
            // Named function definition
            identifier      = first_arg.list[ 0 ].string.as_str();
            res.data_type   = DataType::Lambda;
            res.string      = identifier.to_string();
            res.list.push( Data::from_string( DataType::Variable, "lambda".to_string() ) );
            res.list.push( Data::new_list() );

            for i in 1..first_arg.list.len() {
                res.list[ 1 ].list.push( first_arg.list[ i ].clone() );
            }

            for i in 2..data.list.len() {
                res.list.push( data.list[ i ].clone() );
            }
        }
        else {
            identifier  = data.list[ 1 ].string.as_str();
            res         = self.eval( &data.list[ 2 ] );
        }

        let res_clone = res.clone();
        self.env_data.insert( identifier.to_string(), res );

        res_clone
    }


    fn eval_lambda( &self, data: &Data ) -> Data {
        let mut res         = Data::new();
        res.data_type   = DataType::Lambda;
        res.list        = data.list.clone();
        res
    }

    
    fn eval_if( &mut self, data: &Data ) -> Data {
        let list_len = data.list.len();
        if list_len < 3 || list_len > 4 {
            print_error( Error::ArityMismatch, "if", "2 or 3", (list_len - 1).to_string().as_str() );
            return INVALID_DATA;
        }

        let is_cond_satisfied = !is_false_sym( &self.eval( &data.list[ 1 ] ) ); // Only #f is false, everything else is true
        if is_cond_satisfied {
            self.eval( &data.list[ 2 ] )
        }
        else if list_len == 3 {
            new_void_data()
        }
        else {
            self.eval( &data.list[ 3 ] )
        }
    }


    fn eval_cond( &mut self, data: &Data ) -> Data {
        let list_len = data.list.len();

        for i in 1..list_len {
            let curr_cond   = &data.list[ i ];

            if is_of_type( &DataType::List, &curr_cond ) || curr_cond.list.is_empty() {
                print_error( Error::BadSyntax, "cond", "pair?", curr_cond.to_string().as_str() );
                return INVALID_DATA;
            }

            let is_else = curr_cond.list[ 0 ].string == "else";
            if is_else && curr_cond.list.len() == 1 {
                print_error( Error::BadSyntax, "cond", "expression in `else` clause", "" );
                return INVALID_DATA;
            }

            let mut res = if is_else { INVALID_DATA } else { self.eval( &curr_cond.list[ 0 ] ) };
            if !is_false_sym( &res ) {
                for i in 1..curr_cond.list.len() {
                    res = self.eval( &curr_cond.list[ i ] );
                    if is_invalid_data( &res ) {
                        return res;
                    }
                }

                return res;
            }
        }

        new_void_data()
    }


    fn eval_apply( &mut self, data: &Data ) -> Data {
        let list_len = data.list.len();

        if list_len != 3 {
            print_error( Error::ArityMismatch, "apply", "2", (list_len - 1).to_string().as_str() );
            return INVALID_DATA;
        }

        let proc = self.eval( &data.list[ 1 ] );
        if !is_of_type( &DataType::Procedure, &proc ) && !is_of_type( &DataType::Lambda, &proc ) {
            print_error( Error::ContractViolation, "apply", "a procedure that can be applied to arguments", proc.to_string().as_str() );
            return INVALID_DATA;
        }

        let list = self.eval( &data.list[ 2 ] );
        if is_invalid_data( &list ) {
            return list;
        }

        if is_false_sym( &proc_is_list( &vec![ list.clone() ] ) ) {
            print_error( Error::ContractViolation, "apply", "list?", list.to_string().as_str() );
            return INVALID_DATA;
        }

        let mut to_eval = Data::new_list();
        to_eval.list.reserve( list.list.len() + 2 );
        to_eval.list.push( data.list[ 1 ].clone() );
        for elem in &list.list {
            to_eval.list.push( elem.clone() );
        }

        to_eval.list.pop(); // pops the '() at the end of the list

        self.eval( &to_eval )
    }


    fn eval_map( &mut self, data: &Data ) -> Data {
        let list_len = data.list.len();

        if list_len < 3 {
            print_error( Error::ArityMismatch, "map", "at least 2", (list_len - 1).to_string().as_str() );
            return INVALID_DATA;
        }

        let proc = self.eval( &data.list[ 1 ] );
        if !is_of_type( &DataType::Procedure, &proc ) && !is_of_type( &DataType::Lambda, &proc ) {
            print_error( Error::ContractViolation, "map", "a procedure that can be applied to arguments", proc.to_string().as_str() );
            return INVALID_DATA;
        }

        let list = &data.list[ 2 ];
        if is_false_sym( &proc_is_list( &vec![ list.clone() ] ) ) {
            print_error( Error::ContractViolation, "map", "list?", list.to_string().as_str() );
            return INVALID_DATA;
        }

        let len = list.list.len();
        if len == 0 {
            return NULL_SYM;
        }

        let mut res_list = Data::new_list();
        for i in 0..(len - 1) {
            let mut to_eval = Data::new_list();
            to_eval.list.reserve( len );
            to_eval.list.push( data.list[ 1 ].clone() );
            for j in 2..data.list.len() {
                let list = &data.list[ j ];
                if is_false_sym( &proc_is_list( &vec![ list.clone() ] ) ) {
                    print_error( Error::ContractViolation, "map", "list?", list.to_string().as_str() );
                    return INVALID_DATA;
                }

                let other_len = data.list[ j ].list.len();
                if other_len != len {
                    print_error( Error::ContractViolation, "map", "all lists must have same size", ( (len - 1).to_string() + " and " + ( if other_len == 0 { 0 } else { other_len - 1 } ).to_string().as_str() ).as_str() );
                    return INVALID_DATA;
                }

                to_eval.list.push( list.list[ i ].clone() );
            }

            res_list.list.push( self.eval( &to_eval ) );
        }

        res_list.list.push( NULL_SYM );

        res_list
    }


    pub fn eval_proc_lambda( &mut self, data: &Data ) -> Data {
        assert!( !data.list.is_empty() );

        let proc = self.eval( &data.list[ 0 ] );
        if is_invalid_data( &proc ) {
            return proc;
        }

        if !is_of_type( &DataType::Lambda, &proc ) && !is_of_type( &DataType::Procedure, &proc ) {
            print_error( Error::NotAProcedure, "application", "", data.list[ 0 ].to_string().as_str() );
            return INVALID_DATA;
        }

        let mut args = ProcedureArgsArr::with_capacity( data.list.len() - 1 );
        for i in 1..data.list.len() {
            let res = self.eval( &data.list[ i ] );
            if is_invalid_data( &res ) {
                return res;
            }
            else {
                args.push( res );
            }
        }

        if is_of_type( &DataType::Lambda, &proc ) {
            let params = &proc.list[ 1 ].list;
            if params.len() != args.len() {
                print_error( Error::ArityMismatch, data.list[ 0 ].string.as_str(), params.len().to_string().as_str(), args.len().to_string().as_str() );
                return INVALID_DATA;
            }

            let mut res         = Data::new();
            let mut lambda_env  = Environment::with_args( &params, &args, &self );
            for i in 2..proc.list.len() {
                let body = &proc.list[ i ];
                res = lambda_env.eval( body );
                if is_invalid_data( &res ) {
                    return res;
                }
            }

            return res;
        }

        if is_of_type( &DataType::Procedure, &proc ) {
            (proc.procedure)( &args )
        }
        else {
            INVALID_DATA
        }
    }


    fn add_procedure( &mut self, proc_name: &str, proc: Procedure ) {
        self.env_data.insert( proc_name.to_string(), Data::new_proc( proc_name, proc ) );
    }

}


impl Data {

    pub fn new() -> Data {
        Data { list: vec![], string: String::new(), procedure: NULL_PROC, data_type: DataType::Invalid, quote_level: 0 }
    }

    pub fn from_string( dtype: DataType, string: String ) -> Data {
        Data { list: vec![], string: string, procedure: NULL_PROC, data_type: dtype, quote_level: 0 }
    }

    pub fn from_string_quoted( dtype: DataType, string: String, quote_level: u16 ) -> Data {
        Data { list: vec![], string: string, procedure: NULL_PROC, data_type: dtype, quote_level: quote_level }
    }

    pub fn new_list() -> Data {
        Data { list: vec![], string: String::new(), procedure: NULL_PROC, data_type: DataType::List, quote_level: 0 }
    }

    pub fn from_list( list: ListValuesArr ) -> Data {
        Data { list: list, string: String::new(), procedure: NULL_PROC, data_type: DataType::List, quote_level: 0 }
    }

    pub fn new_proc( proc_name: &str, proc: Procedure ) -> Data {
        Data { list: vec![], string: proc_name.to_string(), procedure: proc, data_type: DataType::Procedure, quote_level: 0 }
    }

    pub fn display( &self, f: &mut fmt::Formatter<'_>, quote_level: u16 ) -> fmt::Result {
        match self.data_type {
            DataType::Lambda => {
                if self.string.is_empty() {
                    write!( f, "#<lambda>" )
                }
                else {
                    write!( f, "#<procedure:{}>", self.string.as_str() )
                }
            },
            DataType::Procedure => {
                write!( f, "#<procedure:{}>", self.string.as_str() )
            },
            DataType::List => {
                if quote_level == 0 {
                    write!( f, "'" );
                }
                
                if quote_level < self.quote_level {
                    for _ in 0..(self.quote_level - quote_level) {
                        write!( f, "'" );
                    }
                }

                write!( f, "(" );

                assert!( !self.list.is_empty() );

                if self.list.len() == 1 {
                    assert!( is_null_sym( &self.list[ 0 ] ) )
                }

                if let Err( e ) = self.list[ 0 ].display( f, quote_level + 1 ) {
                    println!( "{}", e );
                    return Err( e );
                }

                for i in 1..self.list.len() - 1 {
                    write!( f, " " );
                    if let Err( e ) = self.list[ i ].display( f, quote_level + 1 ) {
                        println!( "{}", e );
                        return Err( e );
                    }
                }

                if self.list.len() > 1 && !is_null_sym( self.list.last().unwrap() ) {
                    write!( f, " . " );
                    if let Err( e ) = self.list.last().unwrap().display( f, quote_level + 1 ) {
                        println!( "{}", e );
                        return Err( e );
                    }
                }

                write!( f, ")" )
            }
            DataType::Symbol => {
                if quote_level < self.quote_level {
                    for _ in 0..( self.quote_level - quote_level ) {
                        write!( f, "'" );
                    }
                }

                if !self.string.is_empty() {
                    return write!( f, "{}", &self.string );
                }

                write!( f, "(" );

                if !self.list.is_empty() {
                    if let Err( e ) = self.list.first().unwrap().display( f, self.quote_level ) {
                        println!( "{}", e );
                        return Err( e );
                    }

                    for i in 1..self.list.len() - 1 {
                        write!( f, " " );
                        if let Err( e ) = self.list[ i ].display( f, self.quote_level ) {
                            println!( "{}", e );
                            return Err( e );
                        }
                    }

                    assert!( is_null_sym( self.list.last().unwrap() ) );
                }

                write!( f, ")" )
            },
            DataType::Integer => {
                if !( quote_level == 0 && self.quote_level == 1 ) {
                    if quote_level < self.quote_level {
                        for _ in 0..( self.quote_level - quote_level ) {
                            write!( f, "'" );
                        }
                    }
                }

                write!( f, "{}", &self.string )
            },
            DataType::Real => {
                if !( quote_level == 0 && self.quote_level == 1 ) {
                    if quote_level < self.quote_level {
                        for _ in 0..( self.quote_level - quote_level ) {
                            write!( f, "'" );
                        }
                    }
                }

                write!( f, "{}", (&self.string).parse::<f64>().unwrap().to_string() )
            },
            DataType::Invalid => {
                write!( f, "" )
            },
            _ => {
                write!( f, "{}", self.string )
            }
        }
    }

}


impl fmt::Display for Data {
    fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        self.display( f, 0 )
    }
}


pub const NULL_PROC: fn( &Vec< Data > ) -> Data = |_| NULL_SYM;

pub const NULL_SYM      : Data = Data { list: vec![], string: String::new(), procedure: NULL_PROC, data_type: DataType::Symbol, quote_level: 1 };
pub const INVALID_DATA  : Data = Data { list: vec![], string: String::new(), procedure: NULL_PROC, data_type: DataType::Invalid, quote_level: 0 };


pub fn new_true_sym() -> Data {
    Data::from_string( DataType::Symbol, "#t".to_string() )
}


pub fn new_false_sym() -> Data {
    Data::from_string( DataType::Symbol, "#f".to_string() )
}


pub fn new_void_data() -> Data {
    Data::from_string( DataType::Invalid, "#<void>".to_string() )
}


pub fn is_null_sym( data: &Data ) -> bool {
    is_of_type( &DataType::Symbol, &data )  &&
    data.quote_level == 1                   &&
    data.string.is_empty()                  &&
    data.list.is_empty()
}


pub fn is_true_sym( data: &Data ) -> bool {
    is_of_type( &DataType::Symbol, &data )  &&
    data.quote_level    == 0                &&
    data.string         == "#t"             &&
    data.list.is_empty()
}


pub fn is_false_sym( data: &Data ) -> bool {
    is_of_type( &DataType::Symbol, &data )  &&
    data.quote_level    == 0                &&
    data.string         == "#f"             &&
    data.list.is_empty()
}


pub fn is_void_data( data: &Data ) -> bool {
    is_of_type( &DataType::Invalid, &data ) &&
    data.quote_level    == 0                &&
    data.string         == "#<void>"        &&
    data.list.is_empty()
}


pub fn is_invalid_data( data: &Data ) -> bool {
    is_of_type( &DataType::Invalid, &data ) &&
    data.quote_level    == 0                &&
    data.string.is_empty()                  &&
    data.list.is_empty()
}


enum Error {
    Unknown,
    ArityMismatch,
    NotAProcedure,
    NotImplemented,
    ContractViolation,
    Undefined,
    MissingProcedure,
    BadSyntax,
}


fn print_error( err: Error, proc: &str, expected: &str, given: &str ) {
    print!( "{}: ", proc );

    match err {
        Error::ArityMismatch        => print!( "arity mismatch;\n the expected number of arguments does not match the given number" ),
        Error::NotAProcedure        => print!( "not a procedure;\n expected a procedure that can be applied to arguments" ),
        Error::NotImplemented       => print!( "not implemented;" ),
        Error::ContractViolation    => print!( "contract violation;" ),
        Error::MissingProcedure     => print!( "missing procedure expression;\n probably originally (), which is an illegal empty application in: ({})", proc ),
        Error::Undefined            => print!( "undefined;\n cannot reference an identifier before its definition" ),
        Error::BadSyntax            => print!( "bad syntax;" ),
        _                           => print!( "an unknown error occured;" ),
    }

    if !expected.is_empty() {
        print!( "\n  expected: {}", expected );
    }

    if !expected.is_empty() || !given.is_empty() {
        print!( "\n  given: {}", given );
    }
}


fn is_of_type( data_type: &DataType, data: &Data ) -> bool {
    match ( data_type, &data.data_type ) {
        ( x, y ) if x == y  => true,
        _                   => false,
    }
}


fn _simulation_of_switch_fallthrough( res_type: &mut DataType, ires: &mut i64, fres: &mut f64, i: usize, args: &ProcedureArgsArr, is_div: bool, is_mul: bool, is_sub: bool ) {
    if let DataType::Integer = res_type {
        if is_div {
            panic!( "unreachable" );
        }
        else if is_mul {
            *ires *= args[ i ].string.parse::<f64>().unwrap() as i64;
        }
        else if is_sub {
            *ires -= args[ i ].string.parse::<f64>().unwrap() as i64;
        }
        else {
            *ires += args[ i ].string.parse::<f64>().unwrap() as i64;
        }
    }
    else {
        if is_div {
            *fres /= args[ i ].string.parse::<f64>().unwrap();
        }
        else if is_mul {
            *fres *= args[ i ].string.parse::<f64>().unwrap();
        }
        else if is_sub {
            *fres -= args[ i ].string.parse::<f64>().unwrap();
        }
        else {
            *fres += args[ i ].string.parse::<f64>().unwrap();
        }
    }
}


fn proc_arithmetic_inner( args: &ProcedureArgsArr, is_mul: bool, is_inv: bool ) -> Data {
    let is_div      = is_mul && is_inv;
    let is_sub      = !is_mul && is_inv;
    let proc_name   = if is_mul { if is_inv { "/" } else { "*" } } else { if is_inv { "-" } else { "+" } };

    if is_inv && args.is_empty() {
        print_error( Error::ArityMismatch, proc_name, "at least 1", "0" );
        return INVALID_DATA;
    }

    let mut res_type    = DataType::Integer;
    let mut ires        = if is_mul { 1 as i64 } else { 0 as i64 };
    let mut fres        = if is_mul { 1 as f64 } else { 0 as f64 };

    if is_inv {
        if !is_of_type( &DataType::Integer, &args[ 0 ] ) && !is_of_type( &DataType::Real, &args[ 0 ] ){
            print_error( Error::ContractViolation, proc_name, "number?", args[ 0 ].to_string().as_str() );
            return INVALID_DATA;
        }

        if args.len() == 1 {
            if is_div {
                return Data::from_string( DataType::Real, ( 1.0 as f64 / args[ 0 ].string.parse::<f64>().unwrap() ).to_string() );
            }

            // It's subtraction
            if is_of_type( &DataType::Real, &args[ 0 ] ) {
                return Data::from_string( DataType::Real, ( -args[ 0 ].string.parse::<f64>().unwrap() ).to_string() );
            }

            return Data::from_string( DataType::Integer, ( -args[ 0 ].string.parse::<i64>().unwrap() ).to_string() );
        }

        if is_div || is_of_type( &DataType::Real, &args[ 0 ] ) {
            fres        = args[ 0 ].string.parse::<f64>().unwrap();
            res_type    = DataType::Real;
        }
        else {  // is_of_type( &DataType::Integer, &args[ 0 ]
            ires        = args[ 0 ].string.parse::<i64>().unwrap();
        }
    }

    for i in ( if is_inv { 1 } else { 0 } )..( args.len() ) {
        match args[ i ].data_type {
            DataType::Real => {
                if let DataType::Integer = res_type {
                    res_type    = DataType::Real;
                    fres        = ires as f64;
                }
                _simulation_of_switch_fallthrough( &mut res_type, &mut ires, &mut fres, i, args, is_div, is_mul, is_sub );
            },
            DataType::Integer => {
                _simulation_of_switch_fallthrough( &mut res_type, &mut ires, &mut fres, i, args, is_div, is_mul, is_sub );
            },
            _ => {
                print_error( Error::ContractViolation, proc_name, "number?", args[ i ].to_string().as_str() );
                return INVALID_DATA;
            }
        }
    }

    match res_type {
        DataType::Integer   => Data::from_string( DataType::Integer,    ires.to_string() ),
        DataType::Real      => Data::from_string( DataType::Real,       fres.to_string() ),
        _                   => INVALID_DATA
    }
}


fn proc_add( args: &ProcedureArgsArr ) -> Data {
    proc_arithmetic_inner( args, false, false )
}


fn proc_subtract( args: &ProcedureArgsArr ) -> Data {
    proc_arithmetic_inner( args, false, true )
}


fn proc_multiply( args: &ProcedureArgsArr ) -> Data {
    proc_arithmetic_inner( args, true, false )
}


fn proc_divide( args: &ProcedureArgsArr ) -> Data {
    proc_arithmetic_inner( args, true, true )
}


fn proc_list( args: &ProcedureArgsArr ) -> Data {
    if args.is_empty() {
        NULL_SYM
    }
    else {
        let mut res = Data::from_list( args.clone() );
        res.list.push( NULL_SYM );
        res
    }
}


fn proc_is_null( args: &ProcedureArgsArr ) -> Data {
    if args.len() != 1 {
        print_error( Error::ArityMismatch, "null?", "1", args.len().to_string().as_str() );
        INVALID_DATA
    }
    else {
        if is_null_sym( &args[ 0 ] ) {
            new_true_sym()
        }
        else {
            new_false_sym()
        }
    }
}


fn proc_is_list( args: &ProcedureArgsArr ) -> Data {
    if args.len() != 1 {
        print_error( Error::ArityMismatch, "list?", "1", args.len().to_string().as_str() );
        return INVALID_DATA;
    }

    let arg = &args[ 0 ];

    if is_null_sym( &arg ) {
        new_true_sym()
    }
    else if is_of_type( &DataType::List, &arg ) && arg.list.len() > 1 && is_null_sym( &arg.list.last().unwrap() ) {
        new_true_sym()
    }
    else if is_of_type( &DataType::Symbol, &arg ) && !arg.list.is_empty() && is_null_sym( arg.list.last().unwrap() ) {
        new_true_sym()
    }
    else {
        new_false_sym()
    }
}


fn proc_is_pair( args: &ProcedureArgsArr ) -> Data {
    if args.len() != 1 {
        print_error( Error::ArityMismatch, "pair?", "1", args.len().to_string().as_str() );
        return INVALID_DATA;
    }

    if args[ 0 ].list.len() > 1 {
        new_true_sym()
    }
    else {
        new_false_sym()
    }
}


fn proc_is_string( args: &ProcedureArgsArr ) -> Data {
    if args.len() != 1 {
        print_error( Error::ArityMismatch, "string?", "1", args.len().to_string().as_str() );
        return INVALID_DATA;
    }

    let string_len  =   args[ 0 ].string.len();
    let is_string   =   is_of_type( &DataType::Symbol, &args[ 0 ] )                         &&
                        string_len >= 2                                                     &&
                        args[ 0 ].string.chars().nth( 0 ).unwrap() == '"'                   &&
                        args[ 0 ].string.chars().nth( string_len - 1 ).unwrap()  == '"';

    if is_string {
        new_true_sym()
    }
    else {
        new_false_sym()
    }
}


// TODO: proc cons and others...
