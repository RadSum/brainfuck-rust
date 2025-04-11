use std::io::{Read, Write, stdin, stdout};
use std::error::Error;
use clap::Parser;

mod tokenizer;
mod interactive;
mod args;
use interactive::Command;
use tokenizer::{to_tokens, Token};
use args::Args;

const MEMORY_SIZE: usize = 30_000;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let file_contents = std::fs::read_to_string(&args.file)?;
    let tokens = to_tokens(&file_contents);

    if let Some(tokens) = tokens {
        run_program(tokens, args.interactive);
    } else {
        eprintln!("Error while parsing");
    }

    Ok(())
}

fn run_program(program: Vec<Token>, is_interactive: bool) -> () {
    let mut psw = ProgramStatus::new(program);
    let mut command_str = String::new();

    while !psw.finished() {
        if is_interactive {
            print!("cmd> ");
            stdout().flush().unwrap();
            _ = stdin().read_line(&mut command_str).unwrap();

            let cmd = Command::try_from(command_str.trim());
            if let Ok(cmd) = cmd {
                cmd.run_command(&mut psw);
            }
            command_str.clear();
        } else {
            psw.step();
        }
    }
}

struct ProgramStatus {
    program_memory: Vec<Token>,
    data_memory: [u8; 30000],
    pc: usize,
    memory_pointer: usize,
}

impl ProgramStatus {
    fn new(pm: Vec<Token>) -> Self {
        Self {
            program_memory: pm, 
            data_memory: [0; MEMORY_SIZE],
            pc: 0,
            memory_pointer: 0,
        }
    }

    fn finished(&self) -> bool {
        self.pc >= self.program_memory.len()
    }

    fn step(&mut self) {
        match self.program_memory[self.pc] {
            Token::LeftArrow =>  self.memory_pointer -= 1,
            Token::RightArrow => self.memory_pointer += 1,
            Token::LeftBracket(_pc) => {
                if self.data_memory[self.memory_pointer] == 0 {
                    self.pc = _pc;
                    return;
                }
            },
            Token::RightBracket(_pc) => {
                if self.data_memory[self.memory_pointer] != 0 {
                    self.pc = _pc;
                    return;
                }
            },
            Token::Plus => self.data_memory[self.memory_pointer] = self.data_memory[self.memory_pointer].wrapping_add(1),
            Token::Minus => self.data_memory[self.memory_pointer] = self.data_memory[self.memory_pointer].wrapping_sub(1),
            Token::Dot => {
                print!("{}", self.data_memory[self.memory_pointer] as char);
                std::io::stdout().flush().expect("error while flushing `stdout`");
            },
            Token::Comma => self.data_memory[self.memory_pointer] = read_byte(),
        }
        self.pc += 1;
    }
}

fn read_byte() -> u8 {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|r| r.ok())
        .unwrap()
}
