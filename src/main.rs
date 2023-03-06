use std::{env, io::{Read, BufReader}};

fn main() {
    for (i, arg) in env::args().enumerate() {
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
