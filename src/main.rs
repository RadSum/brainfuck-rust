use std::process::exit;
use std::io::{Read, Write};

mod tokenizer;
use tokenizer::{to_tokens, Token};

const MEMORY_SIZE: usize = 30_000;

fn main() {
    let Some(filename) = std::env::args().nth(1) else {
        eprintln!("USAGE: {} [filename].bf", std::env::args().nth(0).unwrap());
        exit(1);
    };

    let file_content = std::fs::read_to_string(&filename).unwrap_or_else(|e| {
        eprintln!("There was an error opening file: \"{}\", err: {}", filename, e);
        exit(1);
    });

    let tokens = to_tokens(&file_content);
    
    if let Some(tokens) = tokens {
        run_program(&tokens);
    } else {
        eprintln!("Error while parsing");
    }
}

fn run_program(program: &Vec<Token>) -> () {
    let mut pc = 0;
    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    let mut memory_pointer: usize = 0;

    while pc < program.len() {
        match program[pc] {
            Token::LeftArrow => memory_pointer -= 1,
            Token::RightArrow => memory_pointer += 1,
            Token::LeftBracket(_pc) => {
                if memory[memory_pointer] == 0 {
                    pc = _pc;
                    continue;
                }
            },
            Token::RightBracket(_pc) => {
                if memory[memory_pointer] != 0 {
                    pc = _pc;
                    continue;
                }
            },
            Token::Plus => memory[memory_pointer] = memory[memory_pointer].wrapping_add(1),
            Token::Minus => memory[memory_pointer] = memory[memory_pointer].wrapping_sub(1),
            Token::Dot => {
                print!("{}", memory[memory_pointer] as char);
                std::io::stdout().flush().expect("error while flushing `stdout`");
            },
            Token::Comma => memory[memory_pointer] = read_byte(),
        }
        pc += 1;
    }
}

fn read_byte() -> u8 {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|r| r.ok())
        .unwrap()
}
