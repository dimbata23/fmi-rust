use solution::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_commands() {
        let mut interpreter = Interpreter::new();

        let initial_instructions = &[
            "PUSH",
            "POP 2",
            "POP i am an intruder",
            "wrong command",
            "ADD 9",
            "MUL 0",
            "SUB 3",
            "DIV 7",
        ];

        interpreter.add_instructions( initial_instructions );

        while !interpreter.instructions.is_empty() {
            assert_eq!( interpreter.forward(), Err( RuntimeError::InvalidCommand ) );
            assert_eq!( interpreter.stack, &[] );
            interpreter.instructions.pop_front();
        }

        assert_eq!( interpreter.stack, &[] );
    }

    #[test]
    fn test_zero_division() {
        let mut interpreter = Interpreter::new();

        let initial_instructions = &[
            "PUSH 5",
            "PUSH 13",
            "PUSH -13",
            "ADD",
            "PUSH 10",
            "DIV",
        ];

        interpreter.add_instructions( initial_instructions );

        interpreter.forward().unwrap();
        interpreter.forward().unwrap();
        interpreter.forward().unwrap();
        interpreter.forward().unwrap();
        interpreter.forward().unwrap();

        assert_eq!( interpreter.stack, &[ 5, 0, 10 ] );

        assert_eq!( interpreter.forward(), Err( RuntimeError::DivideByZero ) );

        assert_eq!( interpreter.stack, &[ 5, 0, 10 ] );
        assert_eq!( interpreter.instructions, &[ "DIV" ] );
    }

    #[test]
    fn test_stack_underflow() {
        let mut interpreter = Interpreter::new();

        let initial_instructions = &[
            "PUSH 5",
            "PUSH 13",
            "PUSH -13",
            "ADD",
            "SUB",
            "MUL",
        ];

        interpreter.add_instructions( initial_instructions );
        assert_eq!( interpreter.run(), Err( RuntimeError::StackUnderflow ) );
        assert_eq!( interpreter.instructions, &[ "MUL" ] );
        assert_eq!( interpreter.stack, &[ -5 ] );

        interpreter.back().unwrap();

        assert_eq!( interpreter.instructions, &[ "SUB", "MUL" ] );
        assert_eq!( interpreter.stack, &[ 5, 0 ] );

        interpreter.back().unwrap();

        assert_eq!( interpreter.instructions, &[ "ADD", "SUB", "MUL" ] );
        assert_eq!( interpreter.stack, &[ 5, 13, -13 ] );

        interpreter.back().unwrap();

        assert_eq!( interpreter.instructions, &[ "PUSH -13", "ADD", "SUB", "MUL" ] );
        assert_eq!( interpreter.stack, &[ 5, 13 ] );

        interpreter.back().unwrap();

        assert_eq!( interpreter.instructions, &[ "PUSH 13", "PUSH -13", "ADD", "SUB", "MUL" ] );
        assert_eq!( interpreter.stack, &[ 5 ] );

        interpreter.back().unwrap();

        assert_eq!( interpreter.instructions, &[ "PUSH 5", "PUSH 13", "PUSH -13", "ADD", "SUB", "MUL" ] );
        assert_eq!( interpreter.stack, &[] );

        assert_eq!( interpreter.back(), Err( RuntimeError::NoInstructions ) );
    }

    #[test]
    fn test_run() {
        let mut interpreter = Interpreter::new();

        let initial_instructions = &[
            "PUSH 3",
            "PUSH 4",
            "PUSH 9",
            "DIV",
        ];

        interpreter.add_instructions( initial_instructions );

        assert_eq!( interpreter.instructions, initial_instructions );
        assert_eq!( interpreter.stack, &[] );

        interpreter.run().unwrap();

        assert_eq!( interpreter.instructions.len(), 0 );
        assert_eq!( interpreter.stack, &[ 3, 2 ] );

        interpreter.back().unwrap();
        interpreter.back().unwrap();

        assert_eq!( interpreter.instructions, &[
            "PUSH 9",
            "DIV",
        ] );
        assert_eq!( interpreter.stack, &[ 3, 4 ] );

        interpreter.add_instructions( &[ "MUL" ] );

        interpreter.run().unwrap();

        assert_eq!( interpreter.instructions.len(), 0 );
        assert_eq!( interpreter.stack, &[ 6 ] );

        assert_eq!( interpreter.forward(), Err( RuntimeError::NoInstructions ) );
        interpreter.back().unwrap();
        interpreter.back().unwrap();
        interpreter.back().unwrap();
        interpreter.back().unwrap();
        interpreter.back().unwrap();

        let post_initial_instructions = &[
            "PUSH 3",
            "PUSH 4",
            "PUSH 9",
            "DIV",
            "MUL",
        ];

        assert_eq!( interpreter.instructions, post_initial_instructions );

        interpreter.run().unwrap();
        assert_eq!( interpreter.instructions.len(), 0 );
        assert_eq!( interpreter.stack, &[ 6 ] );
    }
}


// Tests ran by the team

#[test]
fn test_basic() {
    let mut interpreter = Interpreter::new();
    interpreter.add_instructions(&[
        "PUSH 1",
        "PUSH 2",
        "PUSH 3",
        "ADD",
    ]);

    assert_eq!(interpreter.instructions, &[
        "PUSH 1",
        "PUSH 2",
        "PUSH 3",
        "ADD",
    ]);
    assert_eq!(interpreter.stack, &[]);

    interpreter.forward().unwrap();
    interpreter.forward().unwrap();
    interpreter.forward().unwrap();

    assert_eq!(interpreter.instructions, &["ADD"]);
    assert_eq!(interpreter.stack, &[1, 2, 3]);

    interpreter.run().unwrap();

    assert_eq!(interpreter.instructions.len(), 0);
    assert_eq!(interpreter.stack, &[1, 5]);

    interpreter.back().unwrap();
    interpreter.back().unwrap();

    assert_eq!(interpreter.instructions, &[
        "PUSH 3",
        "ADD",
    ]);
    assert_eq!(interpreter.stack, &[1, 2]);

    interpreter.add_instructions(&["ADD", "ADD"]);

    assert_eq!(interpreter.run(), Err(RuntimeError::StackUnderflow));
    assert_eq!(interpreter.current_instruction().unwrap(), "ADD");
}
