use solution::*;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    fn bi( s: &str ) -> Bigint {
        Bigint::from_str( s ).unwrap()
    }
    
    fn assert_normal( s: &str ) {
        assert_eq!( format!( "{}", bi( s ) ), s );
    }

    fn assert_delimited( s: &str, exp: &str ) {
        assert_eq!( format!( "{}", bi( s ).delimited() ), exp );
    }

    #[test]
    fn test_normal() {
        assert_normal( "0" );
        assert_normal( "1" );
        assert_normal( "10" );
        assert_normal( "100" );
        assert_normal( "1000" );
        assert_normal( "10000" );
        assert_normal( "100000" );
        assert_normal( "1000000" );
        assert_normal( "10000000" );
        assert_normal( "100000000" );
        assert_normal( "1000000000" );
        assert_normal( "10000000000" );
        assert_normal( "100000000000" );
        assert_normal( "1000000000000" );
        assert_normal( "10000000000000" );
        assert_normal( "100000000000000" );
        assert_normal( "1000000000000000" );
        assert_normal( "10000000000000000" );
        assert_normal( "100000000000000000" );
        assert_normal( "1000000000000000000" );
        assert_normal( "10000000000000000000" );
        assert_normal( "100000000000000000000" );
        assert_normal( "1000000000000000000000" );
    }
    
    #[test]
    fn test_delimited() {
        assert_delimited( "0", "0" );
        assert_delimited( "1", "1" );
        assert_delimited( "10", "10" );
        assert_delimited( "100", "100" );
        assert_delimited( "1000", "1,000" );
        assert_delimited( "10000", "10,000" );
        assert_delimited( "100000", "100,000" );
        assert_delimited( "1000000", "1,000,000" );
        assert_delimited( "10000000", "10,000,000" );
        assert_delimited( "100000000", "100,000,000" );
        assert_delimited( "1000000000", "1,000,000,000" );
        assert_delimited( "10000000000", "10,000,000,000" );
        assert_delimited( "100000000000", "100,000,000,000" );
        assert_delimited( "1000000000000", "1,000,000,000,000" );
    }

}
