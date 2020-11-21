use std::borrow::Cow;

pub struct FizzBuzzer {
    labels: [String; 3],
}


impl FizzBuzzer {
    pub fn new(labels: [String; 3]) -> Self {
        FizzBuzzer { labels }
    }

    pub fn iter(&self) -> FizzBuzzerIter {
        FizzBuzzerIter{ fizzbuzzer: &self, num: 0 }
    }
}


pub struct FizzBuzzerIter<'a> {
    fizzbuzzer: &'a FizzBuzzer,
    num:        usize
}


impl<'a> Iterator for FizzBuzzerIter<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.num += 1;

        let div_by_3  = self.num % 3 == 0;
        let div_by_5  = self.num % 5 == 0;

        if div_by_3 && div_by_5 {
            Some( Cow::Borrowed( &self.fizzbuzzer.labels[2] ) )
        } else if div_by_5 {
            Some( Cow::Borrowed( &self.fizzbuzzer.labels[1] ) )
        } else if div_by_3 {
            Some( Cow::Borrowed( &self.fizzbuzzer.labels[0] ) )
        } else {
            Some( Cow::Owned( self.num.to_string() ) )
        }
    }
}
