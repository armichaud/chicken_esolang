use core::panic;
use std::fs::File;
use std::io::Read;
use std::ops::{AddAssign, Add, Sub, Mul};
use std::cmp::Ordering;
use std::process::exit;
use std::char::from_u32;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Chars(String),
    Num(i64)
}

impl PartialEq<i64> for Token {
    fn eq(&self, other: &i64) -> bool {
        match (self, other) {
            (Token::Num(num1), num2) => num1 == num2,
            _ => panic!("Comparison attempted on at least one string – a: {:?} | b: {:?}", self, other)
        }
    }
}

impl PartialOrd<i64> for Token {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        match (self, other) {
            (Token::Num(num1), num2) => num1.partial_cmp(&num2),
            _ => panic!("Comparison attempted on at least one string – a: {:?} | b: {:?}", self, other)
        }
    }
}

impl AddAssign<i64> for Token {
    fn add_assign(&mut self, other: i64) {
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
        println!("{:?} - {:?}", self, other);
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
    // Constructor
    fn new(user_input: &str) -> Program {
        Program {
            stack: Vec::from([Token::Num(2), Token::Chars(String::from(user_input))])
        }
    }

    // Program methods
    fn load_instructions(&mut self, input: String) {
        for (line_number, line) in input.split("\n").collect::<Vec<&str>>().iter().enumerate() {
            let mut chicken_count = 0;
            for symbol in line.split(" ") {
                match symbol {
                    "chicken" => { chicken_count += 1 },
                    "" => {},
                    _ => panic!("Invalid instruction '{}', line number {}", symbol, line_number)
                }
            }
            self.stack.push(Token::Num(chicken_count));
        }
        println!("{:?}", self.stack);
        self.stack.push(Token::Num(0));
    }

    fn run(&mut self) -> Token{
        while self.stack[0] < self.stack.len() as i64 {
            let instruction = self.next_token();
            if instruction == 0 {
                break;
            }
            self.execute(instruction);
        }
        return self.pop_stack();
    }

    fn execute(&mut self, n: i64) {
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
    
    // Helpers 
    fn next_token(&mut self) -> i64 {
        if let Token::Num(index) = self.stack[0] {
            self.check_stack_index(index); 
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

    fn check_stack_index(&self, index: i64) {
        if index < 0 || index >= self.stack.len() as i64 {
            panic!("Attempted to load from stack but index is out of bounds: {:?}", index);
        }
    }

    // OPs

    fn exit(&mut self) {
        exit(0);
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
        self.stack.push(b - a);
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
        let source = self.next_token();
        match source {
            0 => self.load_stack(),
            1 => self.load_input(),
            _ => panic!("Attempted to load from invalid source: {:?}", source)
        }
    }

    fn load_stack(&mut self) {
        let token = self.pop_stack();
        if let Token::Num(index) = token {
            self.check_stack_index(index);
            let load = self.stack[index as usize].clone();
            self.stack.push(load);
        } else {
            panic!("Stack index is not a number: {:?}", token);
        }
    }

    fn load_input(&mut self) {
        let token = self.pop_stack();
        let input = self.stack[1].clone();
        match (&token, input) {
            (Token::Num(index), Token::Chars(s)) =>{
                let load = Token::Chars(s.chars().nth(*index as usize).expect("Attempted to load from user input but index is out of bounds").to_string());
                self.stack.push(load);
            }
            _ => panic!("Input index is not a number: {:?}", token)
        }
    }

    fn store(&mut self) {
        let (stack_address, value_to_load) = self.pop_stack_twice();
        match stack_address {
            Token::Num(index) => {
                self.check_stack_index(index);
                self.stack[index as usize] = value_to_load;
            },
            _ => panic!("Attempted to store {:?} but stack index is string: {:?}", value_to_load, stack_address)
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
        self.stack.push(if let Token::Num(n) = n { Token::Chars(String::from(from_u32(n as u32).expect(format!("Error converting token {} to ASCII in CHAR op", n).as_str()))) } else { n });
    }

    fn push(&mut self, n: i64) {
        self.stack.push(Token::Num(n - 10));
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let args_len = args.len();
    if args_len < 2 || args_len > 3{
        panic!("Usage: <filename> [optional_input]");
    }

    let path = &args[1];
    if !path.ends_with(".chn") {
        panic!("File must be a .chn file");
    }

    let user_input = if args.len() == 3 { &args[2] } else { "" };

    let mut buffer = String::new();
    let file = File::open(path);

    let read_result = match file {
        Ok(mut file) => file.read_to_string(&mut buffer),
        Err(_) => panic!("Error opening file")
    };
    let result = match read_result {
        Ok(_) => {
            let mut program = Program::new(user_input);
            program.load_instructions(buffer);
            program.run()
        },
        Err(_) => panic!("Error reading file")
    };
    println!("{:?}", result);
}
