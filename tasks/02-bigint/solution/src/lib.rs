use std::str::FromStr;
use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bigint {
    sign: i8,
    digits: Vec<i8>,
}


impl Bigint {
    pub fn new() -> Self {
        Bigint { sign: 1, digits: vec![ 0 ] }
    }

    pub fn is_positive(&self) -> bool {
        self.sign == 1 && self.digits != vec![ 0 ]
    }

    pub fn is_negative(&self) -> bool {
        self.sign == -1 && self.digits != vec![ 0 ]
    }
}


#[derive(Debug)]
pub struct ParseError;


impl FromStr for Bigint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res_big_int = Bigint::new();
        res_big_int.digits.pop();

        for (i, c) in s.chars().enumerate() {
            if c >= '0' && c <= '9' {
                if c == '0' && res_big_int.digits.is_empty() {
                    continue;
                }

                res_big_int.digits.push( c as i8 - '0' as i8 );
            } else if c == '+' && i == 0 {
                res_big_int.sign = 1;
            } else if c == '-' && i == 0 {
                res_big_int.sign = -1;
            } else {
                return Err( ParseError );
            }
        }

        if res_big_int.digits.is_empty() {
            res_big_int.digits.push( 0 );
        }

        if res_big_int.digits.len() == 1 && res_big_int.digits[ 0 ] == 0 {
            res_big_int.sign = 1;
        }

        res_big_int.digits.reverse();

        Ok( res_big_int )
    }
}


impl PartialOrd for Bigint {
    fn partial_cmp(&self, other: &Bigint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for Bigint {
    fn cmp(&self, other: &Bigint) -> Ordering {
        if self.sign < other.sign {
            Ordering::Less
        } else if self.sign > other.sign {
            Ordering::Greater
        } else if self.digits.len() < other.digits.len() {
            if self.sign < 0 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else if self.digits.len() > other.digits.len() {
            if self.sign < 0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            let mut res = Ordering::Equal;
            for (i, n) in self.digits.iter().rev().enumerate() {
                let cmp = n.cmp(&other.digits[ other.digits.len() - i - 1 ]);
                if cmp != Ordering::Equal {
                    res = cmp;
                    if self.sign == -1 {
                        res = res.reverse();
                    }
                    break;
                }
            }
            res
        }
    }
}


impl Add for Bigint {
    type Output = Bigint;

    fn add(self, other: Self) -> Self {
        if self.sign < other.sign {
            other - -self
        } else if self.sign > other.sign {
            self - -other
        } else {
            let mut smaller = &self;
            let mut res;
            if other.digits.len() < self.digits.len() {
                smaller = &other;
                res = self.clone();
            } else {
                res = other.clone();
            }
            for (i, n) in smaller.digits.iter().enumerate() {
                res.digits[ i ] += n;
                if res.digits[ i ] >= 10 {
                    res.digits[ i ] -= 10;
                    if i + 1 < res.digits.len() {
                        res.digits[ i + 1 ] += 1;
                    } else {
                        res.digits.push( 1 );
                    }
                }
            }
            res
        }
    }
}


impl Sub for Bigint {
    type Output = Bigint;

    fn sub(self, other: Self) -> Self {
        if self.sign > other.sign {
            self + -other
        } else if self.sign < other.sign {
            -(-self + other)
        } else {
            let mut res;
            let smaller;
            if self > other && self.sign > 0 || self < other && self.sign < 0 {
                res = self.clone();
                res.sign = self.sign;
                smaller = other;
            } else {
                res = other.clone();
                res.sign = -1 * self.sign;
                smaller = self;
            }

            for (i, n) in smaller.digits.iter().enumerate() {
                res.digits[ i ] -= n;
                let mut ind = i;
                let mut curr_digit = &mut res.digits[ ind ];
                while *curr_digit < 0 {
                    *curr_digit += 10;
                    ind += 1;
                    curr_digit = &mut res.digits[ ind ];
                    *curr_digit -= 1;
                }
            }

            if res.digits[ res.digits.len() - 1 ] == 0 {
                res.digits.pop();
            }

            res
        }
    }
}


impl Neg for Bigint {
    type Output = Bigint;

    fn neg(self) -> Self {
        Self { sign: self.sign * -1, digits: self.digits }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn bi(s: &str) -> Bigint {
        Bigint::from_str(s).unwrap()
    }
    
    #[test]
    fn test_basic() {
        assert_eq!(Bigint::new(), bi("0"));
        assert!(Bigint::from_str("foobar").is_err());
    
        assert!(bi("1").is_positive());
        assert!(bi("-1").is_negative());
    
        assert_eq!(bi("123") + bi("456"), bi("579"));
        assert_eq!(bi("579") - bi("456"), bi("123"));
    
        assert!(bi("123") > bi("122"));
    }

    #[test]
    fn test_zeroes() {
        assert_eq!( Bigint::new(), bi( "" ) );
        assert_eq!( Bigint::new(), bi( "0" ) );
        assert_eq!( Bigint::new(), bi( "+0" ) );
        assert_eq!( Bigint::new(), bi( "-0" ) );
        assert_eq!( Bigint::new(), bi( "+00" ) );
        assert_eq!( Bigint::new(), bi( "-00" ) );
        assert_eq!( Bigint::new(), bi( "+000" ) );
        assert_eq!( Bigint::new(), bi( "-000" ) );
        assert_eq!( Bigint::new(), bi( "+" ) );
        assert_eq!( Bigint::new(), bi( "-" ) );
        assert!( !Bigint::new().is_negative() );
        assert!( !Bigint::new().is_positive() );
    }

    #[test]
    fn test_parse() {
        assert_eq!( bi( "42" ), bi( "+42" ) );
        assert_eq!( bi( "42" ), bi( "+042" ) );
        assert_eq!( bi( "42" ), bi( "+0042" ) );
        assert_eq!( bi( "42" ), bi( "042" ) );
        assert_eq!( bi( "42" ), bi( "0042" ) );
        assert_eq!( bi( "-42" ), bi( "-0042" ) );
        assert_eq!( bi( "-42" ), bi( "-042" ) );
        assert!( Bigint::from_str( "--42" ).is_err() );
        assert!( Bigint::from_str( "42.2" ).is_err() );
        assert!( Bigint::from_str( "42." ).is_err() );
        assert!( Bigint::from_str( " 42" ).is_err() );
        assert!( Bigint::from_str( " 042" ).is_err() );
        assert!( Bigint::from_str( "42 " ).is_err() );
        assert!( Bigint::from_str( "+-42 " ).is_err() );
    }

    #[test]
    fn test_order() {
        assert!( bi( "0" ) < bi( "1" ) );
        assert!( bi( "+0" ) <= bi( "-0" ) );
        assert!( bi( "+0" ) >= bi( "-0" ) );
        assert!( bi( "+0" ) == bi( "-0" ) );
        assert!( bi( "-1" ) < bi( "1" ) );
        assert!( bi( "1" ) > bi( "-1" ) );
        assert!( !( bi( "1" ) > bi( "1" ) ) );
        assert!( !( bi( "1" ) < bi( "1" ) ) );
        assert!( bi( "1" ) >= bi( "1" ) );
        assert!( bi( "1" ) <= bi( "1" ) );
        assert!( bi( "1" ) == bi( "1" ) );
        assert!( bi( "-1" ) == bi( "-1" ) );
        assert!( bi( "-1" ) <= bi( "-1" ) );
        assert!( bi( "-1" ) >= bi( "-1" ) );
        assert!( !( bi( "-1" ) > bi( "-1" ) ) );
        assert!( !( bi( "-1" ) < bi( "-1" ) ) );
        assert!( bi( "69420" ) > bi( "42690" ) );
        assert!( bi( "69420" ) > bi( "-42690" ) );
        assert!( bi( "69420" ) >= bi( "42690" ) );
        assert!( bi( "69420" ) >= bi( "-42690" ) );
        assert!( bi( "42690" ) < bi( "69420" ) );
        assert!( bi( "-42690" ) < bi( "69420" ) );
        assert!( bi( "42690" ) <= bi( "69420" ) );
        assert!( bi( "-42690" ) <= bi( "69420" ) );
        assert!( bi( "42690" ) > bi( "-69420" ) );
        assert!( bi( "42690" ) >= bi( "-69420" ) );
        assert!( bi( "-42690" ) > bi( "-69420" ) );
        assert!( bi( "-42690" ) >= bi( "-69420" ) );
    }

    #[test]
    fn test_addition() {
        assert_eq!( bi( "123" ) + bi( "256" ), bi( "379" ) );
        assert_eq!( bi( "777" ) + bi( "256" ), bi( "1033" ) );
        assert_eq!( bi( "777" ) + bi( "-256" ), bi( "521" ) );
        assert_eq!( bi( "-777" ) + bi( "256" ), bi( "-521" ) );
        assert_eq!( bi( "-777" ) + bi( "-256" ), bi( "-1033" ) );
        assert_eq!( bi( "256" ) + bi( "777" ), bi( "1033" ) );
        assert_eq!( bi( "256" ) + bi( "-777" ), bi( "-521" ) );
        assert_eq!( bi( "-256" ) + bi( "777" ), bi( "521" ) );
        assert_eq!( bi( "-256" ) + bi( "-777" ), bi( "-1033" ) );
    }

    #[test]
    fn test_subtraction() {
        assert_eq!( bi( "777" ) - bi( "256" ), bi( "521" ) );
        assert_eq!( bi( "777" ) - bi( "-256" ), bi( "1033" ) );
        assert_eq!( bi( "777" ) - bi( "-2560" ), bi( "3337" ) );
        assert_eq!( bi( "-777" ) - bi( "256" ), bi( "-1033" ) );
        assert_eq!( bi( "-777" ) - bi( "2560" ), bi( "-3337" ) );
        assert_eq!( bi( "-777" ) - bi( "-256" ), bi( "-521" ) );
        assert_eq!( bi( "-777" ) - bi( "-2560" ), bi( "1783" ) );
        assert_eq!( bi( "-777" ) - bi( "-25" ), bi( "-752" ) );
        assert_eq!( bi( "256" ) - bi( "-777" ), bi( "1033" ) );
        assert_eq!( bi( "-256" ) - bi( "777" ), bi( "-1033" ) );
        assert_eq!( bi( "256" ) - bi( "777" ), bi( "-521" ) );
        assert_eq!( bi( "-256" ) - bi( "-777" ), bi( "521" ) );
    }

    #[test]
    fn test_big_values() {
        assert_eq!( bi( "123456789123456789123456789" ) + bi( "987654321987654321987654321" ), bi( "1111111111111111111111111110" ) );
        assert_eq!( bi( "987235987123289359812730912838795239823598214" )
                    - bi( "987671263786238746871263872365912992837467126765235467523454321987654321987654321" )
                  , bi( "-987671263786238746871263872365912991850231139641946107710723409148859082164056107" )
        );
        assert_eq!( bi( "-987235987123289359812730998137498237512838795239823598214" )
                    - bi( "987671263786238746871263872365912992837467126765235467523454321987654321987654321" )
                  , bi( "-987671263786238746871264859601900116126826939496233605021691834826449561811252535" )
        );
    }
}
