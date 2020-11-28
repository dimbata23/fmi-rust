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

    fn from_str( s: &str ) -> Result< Self, Self::Err > {
        let mut res_big_int = Bigint::new();
        res_big_int.digits.pop();

        for ( index, ch ) in s.chars().enumerate() {
            match ch {
                '0' ..= '9' =>
                {
                    if ch == '0' && res_big_int.digits.is_empty()
                        { continue }
                    else
                        { res_big_int.digits.push( ch as i8 - '0' as i8 ) }
                },
                '+'     if index == 0   =>  res_big_int.sign = 1,
                '-'     if index == 0   =>  res_big_int.sign = -1,
                _                       =>  return Err( ParseError )
            }
        }

        if res_big_int.digits.is_empty() {
            Ok( Bigint::new() )
        } else {
            res_big_int.digits.reverse();
            Ok( res_big_int )
        }
    }
}


impl PartialOrd for Bigint {
    fn partial_cmp( &self, other: &Bigint ) -> Option< Ordering > {
        Some( self.cmp( other ) )
    }
}


impl Ord for Bigint {
    fn cmp( &self, other: &Bigint ) -> Ordering {
        if      self.sign < other.sign  { Ordering::Less }
        else if self.sign > other.sign  { Ordering::Greater }
        else if self.digits.len() < other.digits.len() {
            if self.sign < 0    { Ordering::Greater }
            else                { Ordering::Less }
        } else if self.digits.len() > other.digits.len() {
            if self.sign < 0    { Ordering::Less }
            else                { Ordering::Greater }
        } else {
            let mut res = Ordering::Equal;
            
            for ( index, digit ) in self.digits.iter().rev().enumerate() {
                let cmp = digit.cmp( &other.digits[ other.digits.len() - index - 1 ] );
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

    fn add( self, other: Self ) -> Self {
        if      self.sign < other.sign { other - -self }
        else if self.sign > other.sign { self - -other }
        else {
            let mut smaller = &self;
            let mut res;
            if other.digits.len() < self.digits.len() {
                smaller = &other;
                res = self.clone();
            } else {
                res = other.clone();
            }

            let carry_over = | bigint : &mut Bigint, index : usize | { 
                if bigint.digits[ index ] >= 10 {
                    bigint.digits[ index ] -= 10;
                    if index + 1 < bigint.digits.len() {
                        bigint.digits[ index + 1 ] +=1;
                    } else {
                        bigint.digits.push( 1 );
                    }
                }
            };

            for index in 0..res.digits.len() {
                if index < smaller.digits.len() {
                    res.digits[ index ] += smaller.digits[ index ];
                }
                carry_over( &mut res, index );
            }

            res
        }
    }
}


impl Sub for Bigint {
    type Output = Bigint;

    fn sub( self, other: Self ) -> Self {
        if      self.sign > other.sign { self + -other }
        else if self.sign < other.sign { -(-self + other) }
        else {
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

            for ( index, digit ) in smaller.digits.iter().enumerate() {
                res.digits[ index ] -=  digit;

                let mut curr_ind     =  index;
                let mut curr_digit   =  &mut res.digits[ curr_ind ];

                while *curr_digit < 0 {
                    *curr_digit     +=  10;
                     curr_ind       +=  1;
                     curr_digit      =  &mut res.digits[ curr_ind ];
                    *curr_digit     -=  1;
                }
            }

            while !res.digits.is_empty() && res.digits[ res.digits.len() - 1 ] == 0 {
                res.digits.pop();
            }

            match res.digits.is_empty() {
                true    => Bigint::new(),
                false   => res
            }
        }
    }
}


impl Neg for Bigint {
    type Output = Bigint;

    fn neg( self ) -> Self {
        Self { sign: self.sign * -1, digits: self.digits }
    }
}
