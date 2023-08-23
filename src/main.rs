use chicken_esolang::*;
use std::fs::File;
use std::io::Read;

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
            let mut program = Program::new(user_input, buffer);
            program.run()
        },
        Err(_) => panic!("Error reading file")
    };
    println!("{:?}", result);
}