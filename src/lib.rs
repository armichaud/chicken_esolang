use core::panic;
use std::char::from_u32;
use std::cmp::Ordering;
use std::ops::{AddAssign, Add, Sub, Mul};

#[derive(Debug, Clone)]
enum Token {
    Chars(String),
    Num(i64)
}

impl Token {
    fn js_eq(&self, other: &Token) -> bool {
        match (self, other) {
            (Token::Num(num1), Token::Num(num2)) => num1 == num2,
            (Token::Chars(s1), Token::Chars(s2)) => s1 == s2,
            (Token::Num(n), Token::Chars(s)) | (Token::Chars(s), Token::Num(n)) => {
                if s == "" && *n == 0 {
                    true
                } else {
                    false
                }
            },
        }
    }

    fn js_sub(self, other: Token) -> Token {
        match (&self, &other) {
            (Token::Num(num1), Token::Num(num2)) => Token::Num(num1 - num2),
            _ => Token::Chars(String::from("NaN"))
        }
    }
}

impl PartialEq<Token> for Token {
    fn eq(&self, other: &Token) -> bool {
        match (self, other) {
            (Token::Num(num1), Token::Num(num2)) => num1 == num2,
            (Token::Chars(s1), Token::Chars(s2)) => s1 == s2,
            _ => false,
        }
    }
}

impl PartialEq<i64> for Token {
    fn eq(&self, other: &i64) -> bool {
        match (self, other) {
            (Token::Num(num1), num2) => num1 == num2,
            _ => false,
        }
    }
}

impl PartialEq<String> for Token {
    fn eq(&self, other: &String) -> bool {
        match (self, other) {
            (Token::Chars(num1), num2) => num1 == num2,
            _ => false,
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
            (Token::Chars(s1), Token::Chars(s2)) => Token::Chars(format!("{}{}", s1, s2)),
            (Token::Chars(s), Token::Num(n)) => Token::Chars(format!("{}{}", s, n)),
            (Token::Num(n), Token::Chars(s)) => Token::Chars(format!("{}{}", n, s))
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

pub struct Program {
    stack: Vec<Token>,
    data_stack_index: usize,
    debug: bool,
    backwards_compatible: bool
}

impl Program {
    // Constructor
    pub fn new(code: String, user_input: &str, debug: bool, backwards_compatible: bool) -> Program {
        let input = if let Ok(num) = user_input.parse::<i64>() {
            if num >= 0 {
                Token::Num(num)
            } else {
                Token::Chars(String::from(user_input))
            }
        } else {
            Token::Chars(String::from(user_input))
        };
        let mut program = Program {
            stack: Vec::from([Token::Num(2), input]),
            data_stack_index: 2,
            debug,
            backwards_compatible
        };
        for (line_number, line) in code.split("\n").collect::<Vec<&str>>().iter().enumerate() {
            let mut chicken_count = 0;
            for symbol in line.split(" ") {
                match symbol {
                    "chicken" => { chicken_count += 1 },
                    "" => {},
                    _ => panic!("Invalid instruction '{}', line number {}", symbol, line_number)
                }
            }
            program.stack.push(Token::Num(chicken_count));
        }
        program.stack.push(Token::Num(0));
        program.data_stack_index = program.stack.len();
        program
    }

    // Main loop
    pub fn run(&mut self) -> String {
        while self.stack[0] < self.stack.len() as i64 {
            if self.debug {
                println!("Loading Instruction {:?}", self.stack[0]);
            }
            let instruction = self.next_token();
            if self.debug {
                println!("Executing {}", instruction);
            }
            if instruction == 0 {
                break;
            }
            self.execute(instruction);
        }
        match self.pop_stack() {
            Token::Chars(s) => s,
            Token::Num(n) => n.to_string()
        }
    }

    // Helpers 
    fn execute(&mut self, n: i64) {
        match n {
         // 0 => The EXIT OP is effectively implemented in the program's main loop above.
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
        if self.debug {
            println!("Resulting Input Register: {:?}", self.stack[1]);
            println!(
                "Resulting Data Stack: {:?}", 
                if let Some(data) = self.stack.get(self.data_stack_index..) {
                    data.clone().into_iter().map(|token| 
                        match token {
                            Token::Chars(s) => s.clone(),
                            Token::Num(n) => n.to_string(),
                        }
                    ).collect::<Vec<String>>()
                } else {
                    Vec::new()
                }
            );
            println!("\n");
        }
    }
    
    fn next_token(&mut self) -> i64 {
        if let Token::Num(index) = self.stack[0] {
            self.check_stack_index(index); 
            let op = self.stack[index as usize].clone();
            self.stack[0] += 1;
            if let Token::Num(n) = op { n } else { panic!("Attempted to get next token but instruction is string: {:?}", op) }
        } else {
            panic!("Attempted to get next token but stack index is string: {:?}", self.stack[0]);
        }
    }

    fn pop_stack(&mut self) -> Token {
        if self.stack.len() == self.data_stack_index {
            self.data_stack_index -= 1;
        }
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
    fn chicken(&mut self) {
        self.stack.push(Token::Chars(String::from("chicken")));
    }

    fn add(&mut self) {
        let (a, b) = self.pop_stack_twice();
        self.stack.push(b + a);
    }

    fn sub(&mut self) {
        let (a, b) = self.pop_stack_twice();
        if self.backwards_compatible {
            self.stack.push(b.js_sub(a));
        } else {
            self.stack.push(b - a);
        }
    }

    fn mul(&mut self) {
        let (a, b) = self.pop_stack_twice();
        self.stack.push(a * b);
    }

    fn compare(&mut self) {
        let (a, b) = self.pop_stack_twice();
        self.stack.push(if (self.backwards_compatible && a.js_eq(&b)) || a == b { Token::Num(1) } else { Token::Num(0) } ); 
    }

    fn load(&mut self) {
        let source = self.next_token();
        match source {
            0 => self.load_from_stack(),
            _ => self.load_from_token(source),
        }
    }

    fn load_from_stack(&mut self) {
        let token = self.pop_stack();
        if let Token::Num(index) = token {
            self.check_stack_index(index);
            let load = self.stack[index as usize].clone();
            self.stack.push(load);
        } else {
            panic!("Stack index is not a number: {:?}", token);
        }
    }

    fn load_from_token(&mut self, stack_index: i64) {
        let token = self.pop_stack();
        let input = self.stack[stack_index as usize].clone();
        match (&token, input) {
            (Token::Num(token_index), Token::Chars(s)) =>{
                let load = Token::Chars(s.chars().nth(*token_index as usize).map(|c| c.to_string()).unwrap_or_else({|| 
                    if self.backwards_compatible {
                        "undefined".to_string()
                    } else { 
                        panic!("Attempted to load from string but index is out of bounds: {:?}", token_index);
                    }
                }));
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
                if c.trim() != String::from("") && !(self.backwards_compatible && c.trim() == String::from("NaN")) {
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
        if self.backwards_compatible {
            match n {
                Token::Chars(n) => self.stack.push(Token::Chars(format!("&#{};", n))),
                Token::Num(n) => self.stack.push(Token::Chars(format!("&#{};", n))),
            }
        } else {
            self.stack.push(if let Token::Num(n) = n { Token::Chars(String::from(from_u32(n as u32).expect(format!("Error converting token {} to ASCII in CHAR op", n).as_str()))) } else { n });
        }
    }

    fn push(&mut self, n: i64) {
        self.stack.push(Token::Num(n - 10));
    }
}
