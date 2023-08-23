use chicken_esolang::*;
use std::fs::File;
use std::io::Read;

fn usage(prefix: &str) {
    println!("Usage: {} -- -f <filename> [-i optional_input] [--debug]", prefix);
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        usage(&args[0]);
        return;
    }

    let mut filename = String::new();
    let mut optional_input = String::new();
    let mut debug_mode = false;

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--debug" {
            debug_mode = true;
        } else if args[i] == "-f" {
            if i + 1 < args.len() {
                filename = args[i + 1].clone();
                i += 1;
            } else {
                println!("Missing filename after -f");
                return;
            }
        } else if args[i] == "-i" {
            if i + 1 < args.len() {
                optional_input = args[i + 1].clone();
                i += 1;
            } else {
                println!("Missing optional input after -i");
                return;
            }
        } else {
            usage(&args[0]);
            return;
        }
        i += 1;
    }

    if filename == "" {
        usage(&args[0]);
        return;
    }

    let mut buffer = String::new();
    let file = File::open(filename);

    let read_result = match file {
        Ok(mut file) => file.read_to_string(&mut buffer),
        Err(_) => panic!("Error opening file")
    };
    let result = match read_result {
        Ok(_) => {
            let mut program = Program::new(buffer, optional_input.as_str(), debug_mode);
            program.run()
        },
        Err(_) => panic!("Error reading file")
    };
    println!("{:?}", result);
}
