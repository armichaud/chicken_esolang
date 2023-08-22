use core::panic;
use std::ops::{AddAssign, Add, Sub, Mul};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Chars(String),
    Num(u32)
}

impl PartialEq<u32> for Token {
    fn eq(&self, other: &u32) -> bool {
        match (self, other) {
            (Token::Num(num1), num2) => num1 == num2,
            _ => panic!("Comparison attempted on at least one string – a: {:?} | b: {:?}", self, other)
        }
    }
}

impl PartialOrd<u32> for Token {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        match (self, other) {
            (Token::Num(num1), num2) => num1.partial_cmp(&num2),
            _ => panic!("Comparison attempted on at least one string – a: {:?} | b: {:?}", self, other)
        }
    }
}

impl AddAssign<u32> for Token {
    fn add_assign(&mut self, other: u32) {
        match self {
            Token::Num(n) => *n += other,
            _ => panic!("Mathematical operation attempted on a string – {:?}", self)
        }
    }
}

impl Add for Token {
    type Output = Token;

    fn add(self, other: Token) -> Token {
        match (&self, &other) {
            (Token::Num(num1), Token::Num(num2)) => Token::Num(num1 + num2),
            _ => panic!("Mathematical operation attempted on at least one string – a: {:?} | b: {:?}", self, other)
        }
    }
}

impl Mul for Token {
    type Output = Token;

    fn mul(self, other: Token) -> Token {
        match (&self, &other) {
            (Token::Num(num1), Token::Num(num2)) => Token::Num(num1 * num2),
            _ => panic!("Mathematical operation attempted on at least one string – a: {:?} | b: {:?}", self, other)
        }
    }
}

impl Sub for Token {
    type Output = Token;

    fn sub(self, other: Token) -> Token {
        match (&self, &other) {
            (Token::Num(num1), Token::Num(num2)) => Token::Num(num1 - num2),
            _ => panic!("Mathematical operation attempted on at least one string – a: {:?} | b: {:?}", self, other)
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
        self.stack.push(Token::Num(0));
    }

    fn run(&mut self) -> Token{
        while self.stack[0] < self.stack.len() as u32 {
            let instruction = self.next_token();
            if instruction == 0 {
                break;
            }
            self.execute(instruction);
        }
        return self.pop_stack();
    }

    fn execute(&mut self, instruction: u32) {
        match instruction {
            1 => self.chicken(),
            2 => self.add(),
            3 => self.sub(),
            4 => self.mul(),
            5 => self.compare(),
            6 => self.load(),
            7 => self.store(),
            8 => self.jump(),
            9 => self.char(),
            _ => self.push(instruction),
        }
    }

    fn next_token(&mut self) -> u32 {
        if let Token::Num(index) = self.stack[0] {
            let op = self.stack[index as usize].clone();
            self.stack[0] += 1;
            if let Token::Num(n) = op {
                n
            } else {
                panic!("Attempted to get next token but instruction is string: {:?}", op);
            }
        } else {
            panic!("Attempted to get next token but stack index is string: {:?}", self.stack[0]);
        }
    }

    fn pop_stack(&mut self) -> Token {
        self.stack.pop().expect("Error popping from stack")
    }

    fn pop_stack_twice(&mut self) -> (Token, Token) {
        (self.pop_stack(), self.pop_stack())
    }

    fn chicken(&mut self) {
        self.stack.push(Token::Chars(String::from("chicken")));
    }

    fn add(&mut self) {
        let (a, b) = self.pop_stack_twice();
        self.stack.push(a + b);
    }

    fn sub(&mut self) {
        let (a, b) = self.pop_stack_twice();
        self.stack.push(a - b);
    }

    fn mul(&mut self) {
        let (a, b) = self.pop_stack_twice();
        self.stack.push(a * b);
    }

    fn compare(&mut self) {
        let (a, b) = self.pop_stack_twice();
        self.stack.push(if a == b { Token::Num(1) } else { Token::Num(0) } ); 
    }

    fn load(&mut self) {
        let stack_index = self.next_token();
        let string_token = self.stack[stack_index as usize].clone();
        let string_index = &self.pop_stack();
        match (&string_token, string_index) {
            (Token::Chars(s), Token::Num(i)) => {
                if let Some((char_index, _)) = s.char_indices().nth(*i as usize) {
                    let char_at_index = s[char_index..].chars().next().unwrap();
                    if (*i as usize) < s.len() {
                        self.stack.push(Token::Chars(char_at_index.to_string()));
                    } else {
                        panic!("Attempted to load from {:?} but index {:?} is out of bounds", s, i)
                    }
                } else {
                    panic!("Attempted to load from {:?} but index {:?} is out of bounds", s, i)
                }
            },
            (Token::Num(_), _) => panic!("Attempted to load char at index {:?} from {:?} but it is not a string", string_index, string_token),
            _ => panic!("Attempted to load index {:?} but it is not a string", string_index)
        }
    }

    fn store(&mut self) {
        let (stack_address, value_to_load) = self.pop_stack_twice();
        if let Token::Num(index) = stack_address {
            if (index as usize) < self.stack.len() {
                self.stack[index as usize] = value_to_load;
            } else {
                panic!("Attempted to store {:?} at index {:?} of stack length {:?}", value_to_load, stack_address, self.stack.len());
            }
        } else {
            panic!("Attempted to store {:?} but stack index is string: {:?}", value_to_load, stack_address);
        }
    } 

    fn jump(&mut self) {
        let (offset, condition) = self.pop_stack_twice();

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
}

fn main() {
    let input = String::from("chicken");
    let user_input = "";
    let mut program = Program::new(user_input);
    program.load_instructions(input);
    let result = program.run();
    println!("{:?}", result);
}
