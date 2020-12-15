use std::str::FromStr;
use std::fmt;


// Declaration of BigInt given for the task
#[derive( Debug, Clone, PartialEq, Eq )]
pub struct Bigint {
    pub digits: Vec<u8>,
}


// Implementation of BigInt given for the task
impl FromStr for Bigint {
    type Err = &'static str;

    fn from_str( s: &str ) -> Result< Self, Self::Err > {
        let mut digits = Vec::with_capacity( s.len() );

        for c in s.chars() {
            if let Some( digit ) = c.to_digit( 10 ) {
                digits.push( digit as u8 );
            } else {
                return Err( "Invalid input!" );
            }
        }

        Ok( Bigint { digits } )
    }
}


impl fmt::Display for Bigint {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        for num in self.digits.iter() {
            let res = write!( f,  "{}", num );
            if res.is_err() {
                return res;
            }
        }

        Ok(())
    }
}


pub struct Delimited<'a> {
    bigint: &'a Bigint,
}


impl Bigint {
    pub fn delimited( &self ) -> Delimited {
        Delimited { bigint: self }
    }
}


impl<'a> fmt::Display for Delimited<'a> {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        for ( i, num ) in self.bigint.digits.iter().enumerate() {
            let len = self.bigint.digits.len();
            let ind = len - 1 - i;
            let del = ind % 3 == 0 && i != len - 1;
            let res;
            if del {
                res = write!( f, "{},", num );
            } else {
                res = write!( f, "{}", num );
            }
            if res.is_err() {
                return res;
            }
        }

        Ok(())
    }
}
