use std::fmt;

#[derive( Debug )]
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


pub struct Environment {

}


pub struct Data {
    pub list        : Vec< Data >,
    pub string      : String,
    pub procedure   : fn( &Vec< Data > ) -> Data,
    pub data_type   : DataType,
    pub quote_level : u16,
    //pub fEnvironment    : &'a Environment
}


pub const NULL_SYM: Data = Data { list: vec![], string: String::new(), procedure: | _ | NULL_SYM, data_type: DataType::Symbol, quote_level: 1 };


impl fmt::Debug for Data {

    fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        f.debug_tuple( "" )
            .field( &self.data_type )
            .field( &self.string )
            .field( &self.quote_level )
            .field( &self.list )
            .finish()
    }

}


impl Data {

    pub fn new() -> Data {
        Data { list: vec![], string: String::new(), procedure: | _ | NULL_SYM, data_type: DataType::Invalid, quote_level: 0 }
    }

    pub fn from_string( dtype: DataType, string: &str ) -> Data {
        Data { list: vec![], string: string.to_string(), procedure: | _ | NULL_SYM, data_type: dtype, quote_level: 0 }
    }

    pub fn from_string_quoted( dtype: DataType, string: &str, quote_level: u16 ) -> Data {
        Data { list: vec![], string: string.to_string(), procedure: | _ | NULL_SYM, data_type: dtype, quote_level: quote_level }
    }

    pub fn new_list() -> Data {
        Data { list: vec![], string: String::new(), procedure: | _ | NULL_SYM, data_type: DataType::List, quote_level: 0 }
    }

}

