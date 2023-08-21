use core::panic;
use std::process::exit;

const OFFSET: u32 = 2;

#[derive(Debug)]
enum StackFrame {
    Token(String),
    Num(u32)
}

struct Program {
    stack: Vec<StackFrame>,
}

impl Program {
    fn new(user_input: &str) -> Program {
        Program {
            stack: Vec::from([StackFrame::Num(0), StackFrame::Token(String::from(user_input))])
        }
    }

    fn load_instructions(&mut self, input: String) {
        for (line_number, line) in input.split("\n").collect::<Vec<&str>>().iter().enumerate() {
            let mut chicken_count = 0;
            for symbol in line.split(" ") {
                if (symbol != "chicken") {
                    panic!("Invalid instruction '{}', line number {}", symbol, line_number);
                }
                chicken_count += 1;
            }
            self.stack.push(StackFrame::Num(chicken_count));
        }
    }

    fn execute_instruction(&mut self, n: u32) {
        match n {
            0 => self.exit(),
            1 => self.chicken(),
            2 => self.add(),
            3 => self.sub(),
            4 => self.mul(),
            5 => self.compare(),
            6 => self.load(),
            7 => self.store(),
            8 => self.jump(),
            9 => self.char(),
            _ => self.push(n),
        }
    }

    fn exit(&self) {
        exit(0);
    }

    fn chicken(&mut self) {
        self.stack.push(StackFrame::Token(String::from("chicken")));
    }

    fn add(&mut self) {
        let (a, b) = self.get_top_two_stack_values();
        let result = Program::mathematical_operation(a, b, |m, n| m + n ); 
        // TODO do something with result
    }

    fn sub(&mut self) {
        let (a, b) = self.get_top_two_stack_values();
        let result = Program::mathematical_operation(a, b, |m, n| m - n ); 
        // TODO do something with result
    }

    fn mul(&mut self) {
        let (a, b) = self.get_top_two_stack_values();
        let result = Program::mathematical_operation(a, b, |m, n| m * n );
        // TODO do something with result
    }

    fn compare(&mut self) {
        let (a, b) = self.get_top_two_stack_values();
        match a {
            StackFrame::Token(s) => {
                if let StackFrame::Token(t) = b {
                    self.stack.push(StackFrame::Num(if s == t { 1 } else { 0 } ));
                } else {
                    Program::type_mismatch(StackFrame::Token(s), b);
                }
            },
            StackFrame::Num(m) => {
                if let StackFrame::Num(n) = b {
                    self.stack.push(StackFrame::Num(if m == n { 1 } else { 0 } ));
                } else {
                    Program::type_mismatch(a,b); 
                }
            }
        }
    }

    fn load(&self) {
        println!("TODO");
    }

    fn store(&self) {
        println!("TODO");
    }

    fn jump(&self) {
        println!("TODO");
    }

    fn char(&mut self) {
        let n = self.stack.pop().unwrap();
        if let StackFrame::Num(n) = n {
            self.stack.push(StackFrame::Token(String::from(std::char::from_u32(n).unwrap())))
        } else {
            self.stack.push(n)
        }
    }

    fn push(&mut self, n: u32) {
        self.stack.push(StackFrame::Num(n - 10));
    }

    fn get_top_two_stack_values(&mut self) -> (StackFrame, StackFrame) {
        let top = self.stack.pop().unwrap(); // TODO: Handle unwrap
        let second = self.stack.pop().unwrap(); // TODO: Handle unwrap
        (top, second)
    }

    fn type_mismatch(a: StackFrame, b:StackFrame) {
        panic!("Mismatched types, a: {:?}, b: {:?}", a, b)
    }

    fn mathematical_operation<F: Fn(u32, u32) -> u32>(a: StackFrame, b: StackFrame, operation: F) -> u32 {
        if let StackFrame::Num(a) = a {
            if let StackFrame::Num(b) = b {
                operation(a, b)
            } else {
                panic!("Mathematical operation applied to string {:?}", b); 
            }
        } else { 
            panic!("Mathematical operation applied to string {:?}", a); 
        }
    }
}

fn main() {
    let input = String::new();
    let user_input = " ";
    let mut program = Program::new(user_input);
    program.load_instructions(input);
}
