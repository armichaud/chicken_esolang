use core::panic;

enum OPCODES {
    EXIT,
    CHICKEN,
    ADD,
    SUB,
    MUL,
    COMPARE,
    LOAD,
    STORE,
    JUMP,
    CHAR,
}

struct Interpreter {
    stack: [usize; 16],
}

impl Interpreter {
    fn parse_and_execute_instruction(&self, n: usize) {
        if n > 9 {
            return;
        }
        let opcode = match n {
            0 => OPCODES::EXIT,
            1 => OPCODES::CHICKEN,
            2 => OPCODES::ADD, 
            3 => OPCODES::SUB,
            4 => OPCODES::MUL,
            5 => OPCODES::COMPARE,
            6 => OPCODES::LOAD,
            7 => OPCODES::STORE,
            8 => OPCODES::JUMP,
            9 => OPCODES::CHAR,
            _ => panic!("Something went wrong parsing the number of chickens")
        };
        self.execute_instruction(opcode);
    }

    fn execute_instruction(&self, opcode: OPCODES) {}
}

fn main() {
    let input = String::from("chicken");
    let interpreter = Interpreter {
        stack: [0; 16],
    };

    for (line_number, line) in input.split("\n").collect::<Vec<&str>>().iter().enumerate() {
        let mut chicken_count = 0;
        for symbol in line.split(" ") {
            if (symbol != "chicken") {
                panic!("Invalid instruction '{}', line number {}", symbol, line_number);
            }
            chicken_count += 1;
        }
        interpreter.parse_and_execute_instruction(chicken_count);
    }
}
