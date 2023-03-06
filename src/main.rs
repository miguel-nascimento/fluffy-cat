use std::{env, io::{Read, stdin, BufReader}};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        let stdin = stdin();
        loop {
            let mut buffer = String::new();
            stdin.read_line(&mut buffer).expect("ERR: Could not read stdin");
            print!("{buffer}");
        }
    }

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        };
        let file = std::fs::File::open(arg).expect("ERR: File not found");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect("ERR: Could not read file");
        
        print!("{contents}");
    }
}
