use core::panic;
use std::process::exit;

struct Program {
    stack: Vec<u8>,
}

impl Program {
    fn new() -> Program {
        Program {
            stack: Vec::new(),
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
            if (chicken_count > 255) {
                panic!("Overflow error: Chicken count of {} exceeds u8 capacity, line number {}", chicken_count, line_number);
            }
            self.stack.push(chicken_count as u8);
        }
    }

    fn execute_instruction(&mut self, n: u8) {
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
        // TODO 
    }

    fn chicken(&mut self) {
        for byte in "Chicken".as_bytes() {
            self.stack.push(*byte);
        }
    }

    fn add(&mut self) {
        let (a, b) = self.get_top_two_stack_values();
        a + b;
    }

    fn sub(&mut self) {
        let (a, b) = self.get_top_two_stack_values();
        a - b;
    }

    fn mul(&mut self) {
        let (a, b) = self.get_top_two_stack_values();
        a * b;
    }

    fn compare(&mut self) {
        let (a, b) = self.get_top_two_stack_values();
        if (a == b) {
            self.stack.push(1 as u8);
        } else {
            self.stack.push(0 as u8);
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

    fn char(&self) {
        println!("TODO");
    }

    fn push(&mut self, n: u8) {
        self.stack.push(n - 10);
    }

    fn get_top_two_stack_values(&mut self) -> (u8, u8) {
        let top = self.stack.pop().unwrap(); // TODO: Handle unwrap
        let second = self.stack.pop().unwrap(); // TODO: Handle unwrap
        (top, second)
    }
}

fn main() {
    let mut program = Program::new();
}
