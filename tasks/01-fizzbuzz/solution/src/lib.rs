pub fn fizzbuzz( n: usize ) -> Vec<String> {
    custom_buzz( n, 3, 5 )
}


pub fn custom_buzz( n: usize, k1: u8, k2: u8 ) -> Vec<String> {
    FizzBuzzer {
        k1,
        k2,
        labels: [
            String::from( "Fizz" ),
            String::from( "Buzz" ),
            String::from( "Fizzbuzz" )
        ],
    }.take( n )
}


pub struct FizzBuzzer {
    pub k1:     u8,
    pub k2:     u8,
    pub labels: [ String; 3 ],
}


impl FizzBuzzer {

    pub fn take( &self, n: usize ) -> Vec<String> {
        if self.k1 == 0 || self.k1 == 1 || self.k2 == 0 || self.k2 == 1 {
            panic!( "FizzBuzzer: k1/k2 expected [2, 255], found {}/{}", self.k1, self.k2 );
        }

        let mut res :   Vec<String> = Vec::new();

        for i in 1 ..= n {

            let div_by_k1  = i % self.k1 as usize == 0;
            let div_by_k2  = i % self.k2 as usize == 0;

            if div_by_k1 && div_by_k2 {
                res.push( self.labels[ 2 ].clone() );
            }
            else if div_by_k2 {
                res.push( self.labels[ 1 ].clone() );
            }
            else if div_by_k1 {
                res.push( self.labels[ 0 ].clone() );
            }
            else {
                res.push( i.to_string() );
            }

        }

        res
    }


    pub fn change_label( &mut self, index: usize, value: &String ) {
        match index {
            0 ..= 2 => self.labels[ index ] = value.clone(),
            _       => panic!( "FizzBuzzer::change_label( index, value ): Index out of bounds! Expected [0, 2], found {}", index ),
        }
    }

}
