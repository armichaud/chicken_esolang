use core::panic;
use std::process::exit;
use std::ops::{AddAssign, Add, Sub, Mul};

#[derive(Debug)]
enum Token {
    Chars(String),
    Num(u32)
}

impl AddAssign<u32> for Token {
    fn add_assign(&mut self, other: u32) {
        match self {
            Token::Num(n) => *n += other,
            _ => panic!("Mathematical operation attempted on string types")
        }
    }
}

impl Add for Token {
    type Output = Token;

    fn add(self, other: Token) -> Token {
        match (self, other) {
            (Token::Num(num1), Token::Num(num2)) => Token::Num(num1 + num2),
            _ => panic!("Mathematical operation attempted on string types")
        }
    }
}

impl Mul for Token {
    type Output = Token;

    fn mul(self, other: Token) -> Token {
        match (self, other) {
            (Token::Num(num1), Token::Num(num2)) => Token::Num(num1 * num2),
            _ => panic!("Mathematical operation attempted on string types")
        }
    }
}

impl Sub for Token {
    type Output = Token;

    fn sub(self, other: Token) -> Token {
        match (self, other) {
            (Token::Num(num1), Token::Num(num2)) => Token::Num(num1 - num2),
            _ => panic!("Mathematical operation attempted on string types")
        }
    }
}

struct Program {
    stack: Vec<Token>,
}

impl Program {
    fn new(user_input: &str) -> Program {
        Program {
            stack: Vec::from([Token::Num(2), Token::Chars(String::from(user_input))])
        }
    }

    fn load_instructions(&mut self, input: String) {
        for (line_number, line) in input.split("\n").collect::<Vec<&str>>().iter().enumerate() {
            let mut chicken_count = 0;
            for symbol in line.split(" ") {
                if symbol != "chicken" {
                    panic!("Invalid instruction '{}', line number {}", symbol, line_number);
                }
                chicken_count += 1;
            }
            self.stack.push(Token::Num(chicken_count));
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

    fn pop_stack(&mut self) -> Token {
        self.stack.pop().expect("Error popping from stack")
    }

    fn exit(&self) {
        exit(0);
    }

    fn chicken(&mut self) {
        self.stack.push(Token::Chars(String::from("chicken")));
    }

    fn add(&mut self) {
        let (a, b) = self.get_top_two_stack_values();
        self.stack.push(a + b);
    }

    fn sub(&mut self) {
        let (a, b) = self.get_top_two_stack_values();
        self.stack.push(a - b);
    }

    fn mul(&mut self) {
        let (a, b) = self.get_top_two_stack_values();
        self.stack.push(a * b);
    }

    fn compare(&mut self) {
        let (a, b) = self.get_top_two_stack_values();
        match (&a, &b) {
            (Token::Chars(s), Token::Chars(t)) => {
                self.stack.push(Token::Num(if s == t { 1 } else { 0 } ));
            },
            (Token::Num(m), Token::Num(n)) => {
                self.stack.push(Token::Num(if m == n { 1 } else { 0 } ));
            },
            _ => panic!("Mismatched types, a: {:?}, b: {:?}", a, b)
        }
    }

    fn load(&self) {
        println!("TODO");
    }

    fn store(&self) {
        println!("TODO");
    }

    fn jump(&mut self) {
        let offset = self.pop_stack();
        let condition = self.pop_stack();
        match (offset, condition) {
            (Token::Chars(s), _) => panic!("Stack offset is not a number: {:?}", s),
            (Token::Num(offset), Token::Chars(c)) => {
                if c.trim() != String::from("") {
                    self.stack[0] += offset;
                }
            },
            (Token::Num(offset), Token::Num(n)) => { 
                if n != 0 {
                    self.stack[0] += offset;
                }
            }
        }
    }

    fn char(&mut self) {
        let n = self.pop_stack();
        if let Token::Num(n) = n {
            self.stack.push(Token::Chars(String::from(std::char::from_u32(n).unwrap())))
        } else {
            self.stack.push(n)
        }
    }

    fn push(&mut self, n: u32) {
        self.stack.push(Token::Num(n - 10));
    }

    fn get_top_two_stack_values(&mut self) -> (Token, Token) {
        let top = self.stack.pop().unwrap(); // TODO: Handle unwrap
        let second = self.stack.pop().unwrap(); // TODO: Handle unwrap
        (top, second)
    }
}

fn main() {
    let input = String::new();
    let user_input = " ";
    let mut program = Program::new(user_input);
    program.load_instructions(input);
}
