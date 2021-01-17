use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    DivideByZero,
    StackUnderflow,
    InvalidCommand,
    NoInstructions,
}


#[derive(Debug, Default)]
pub struct Interpreter {
    pub instructions: VecDeque< String >,
    pub stack: Vec< i32 >,
    history: Vec< ( String, Vec<i32> ) >,
}


impl Interpreter {

    pub fn new() -> Self {
        Interpreter { instructions: VecDeque::<String>::new(), stack: vec![], history: vec![] }
    }


    pub fn add_instructions( &mut self, instructions: &[ &str ] ) {
        for instr in instructions {
            self.instructions.push_back( instr.to_string() );
        }
    }


    pub fn current_instruction( &mut self ) -> Option< &mut String > {
        if self.instructions.is_empty() {
            None
        } else {
            Some( &mut self.instructions[0] )
        }
    }


    pub fn current_instruction_immutable( &self ) -> Option< &String > {
        if self.instructions.is_empty() {
            None
        } else {
            Some( &self.instructions[0] )
        }
    }


    fn do_operation( &mut self, op: &str ) -> Result< (), RuntimeError > {
        if self.stack.len() < 2 {
            Err( RuntimeError::StackUnderflow )
        } else {
            let num1 = self.stack.pop().unwrap();
            let num2 = self.stack.pop().unwrap();
            match op {
                "ADD" => { self.stack.push( num1 + num2 ); },
                "MUL" => { self.stack.push( num1 * num2 ); },
                "SUB" => { self.stack.push( num1 - num2 ); },
                "DIV" => {
                    if num2 == 0 {
                        self.stack.push( num2 );
                        self.stack.push( num1 );
                        return Err( RuntimeError::DivideByZero );
                    }
                    self.stack.push( num1 / num2 );
                },

                _ => { return Err( RuntimeError::InvalidCommand ); }
            };
            self.history.push( ( op.to_string(), vec![ num2, num1 ] ) );
            Ok( () )
        }
    }


    pub fn forward( &mut self ) -> Result< (), RuntimeError > {
        let curr_instr = self.current_instruction_immutable();
        let instruction : String;

        match curr_instr {
            Some( i )   => { instruction = i.to_string() },
            None        => { return Err( RuntimeError::NoInstructions ); }
        }

        let instr_vec = instruction.trim().split(' ').collect::< Vec<_> >();

        if instr_vec.is_empty() {
            Err( RuntimeError::InvalidCommand )
        } else {
            let res : Result<(), RuntimeError>;

            match instr_vec[0].trim() {

                "PUSH" => {
                    if instr_vec.len() != 2 {
                        res = Err( RuntimeError::InvalidCommand );
                    } else {
                        let parsed_int = instr_vec[1].parse::<i32>();
                        match parsed_int {
                            Ok( num ) => {
                                self.stack.push( num );
                                self.history.push( ( "PUSH".to_string(), vec![] ) );
                                res = Ok( () );
                            },
                            Err(_) => { res = Err( RuntimeError::InvalidCommand ); }
                        }
                    }
                },

                "POP" => {
                    if instr_vec.len() != 1 {
                        res = Err( RuntimeError::InvalidCommand );
                    } else {
                        match self.stack.pop() {
                            Some( num ) => {
                                self.history.push( ( "POP".to_string(), vec![ num ] ) );
                                res = Ok( () );
                            },
                            None => { res = Err( RuntimeError::StackUnderflow ); }
                        }
                    }
                },

                op if op == "ADD" || op == "MUL" || op == "SUB" || op == "DIV" => {
                    if instr_vec.len() != 1 {
                        res = Err( RuntimeError::InvalidCommand )
                    } else {
                        res = self.do_operation( op );
                    }
                },

                _ => { res = Err( RuntimeError::InvalidCommand ); }
            };

            if res.is_ok() {
                self.instructions.pop_front();
            }

            res
        }
    }


    pub fn run( &mut self ) -> Result< (), RuntimeError > {
        loop {
            match self.forward() {
                Err( RuntimeError::NoInstructions ) => return Ok( () ),
                Err( e ) => return Err( e ),
                _ => (),
            }
        }
    }


    pub fn back( &mut self ) -> Result< (), RuntimeError > {
        match self.history.pop() {

            Some( ( instr, args ) ) => {
                match instr.as_str() {

                    "PUSH"  => {
                        self.instructions.push_front( instr.clone() + " " + self.stack.pop().unwrap().to_string().as_str() );
                        Ok( () )
                    },

                    "POP"   => {
                        self.instructions.push_front( instr.clone() );
                        self.stack.push( args[0] );
                        Ok( () )
                    },

                    "ADD" | "MUL" | "SUB" | "DIV"   => {
                        self.instructions.push_front( instr.clone() );
                        self.stack.pop();
                        self.stack.push( args[0] );
                        self.stack.push( args[1] );
                        Ok( () )
                    },
                    
                    _ => Err( RuntimeError::InvalidCommand )

                }
            },

            None => Err( RuntimeError::NoInstructions )

        }
    }

}
