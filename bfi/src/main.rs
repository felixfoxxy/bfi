use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Rust Brainfuck Interpreter made by FelixFoxxy");
        println!("Usage: bfi [File.bf]");
        return;
    }
    
    if !args[1].to_lowercase().ends_with(".bf") {
        println!("Usage: bfi [File.bf]");
        return;
    }

    let read = fs::read_to_string(&args[1]);
    match read{
        Ok(s) => {
            intp(s);
        }
        Err(e) => {
            println!("Error reading File: {e}");
        }
    }
}

fn intp(source: String){
    let chars: Vec<char> = source.chars().collect();
    let mut tape: Vec<u8> = vec![0; i32::MAX as usize];
    let mut i: usize = 0;
    let mut ptr: usize = 0;
    while i < chars.len() {
        let cc: char = chars[i];
        match cc {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => tape[ptr] += 1,
            '-' => tape[ptr] -= 1,
            '.' => print!("{}", tape[ptr] as char), 
            ',' => {
                let mut input: [u8; 1] = [0];
                std::io::stdin().read_exact(&mut input).expect("Invalid Input!");
                tape[ptr] = input[0];
            }
            '[' => {
                if tape[ptr] == 0 {
                    let mut skip = 0;
                    let mut lptr = i + 1;
                    while chars[lptr] != ']' || skip > 0 {
                        if chars[lptr] == '[' {
                            skip += 1;
                        }
                        else if chars[lptr] == ']' {
                            skip -= 1;
                        }
                        lptr += 1;
                        i = lptr;
                    }
                }
            }
            ']' => {
                if tape[ptr] != 0 {
                    let mut skip = 0;
                    let mut lptr = i - 1;
                    while chars[lptr] != '[' || skip > 0 {
                        if chars[lptr] == ']' {
                            skip += 1;
                        }
                        else if chars[lptr] == '[' {
                            skip -= 1;
                        }
                        lptr -= 1;
                        i = lptr;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
}