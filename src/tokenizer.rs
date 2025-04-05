#[derive(Debug)]
pub enum Token {
    RightArrow,
    LeftArrow,
    Plus,
    Minus,
    LeftBracket(usize),
    RightBracket(usize),
    Comma,
    Dot,
}

const BRAINFUCK_TOKENS: &'static str = "><+-[],.";

fn is_valid_token(c: char) -> bool {
    BRAINFUCK_TOKENS.contains(c)
}

fn token_from_char(c: char) -> Token {
    use Token::*;
    match c {
        '>' => RightArrow,
        '<' => LeftArrow,
        '+' => Plus,
        '-' => Minus,
        '[' => LeftBracket(0),
        ']' => RightBracket(0),
        ',' => Comma,
        '.' => Dot,
        _   => unreachable!(),
    }
}

pub fn to_tokens(file_content: &str) -> Option<Vec<Token>> {
    let mut curr_pc: usize = 0;
    let mut program = Vec::new();
    let mut bracket_stack: Vec<usize> = Vec::new();
    for c in file_content.chars() {
        if !is_valid_token(c) {
            continue;
        }

        if c == '[' {
            bracket_stack.push(curr_pc);
            program.push(Token::LeftBracket(0));
        } else if c == ']' {
            if bracket_stack.is_empty() {
                return None;
            }
            let start_pc = bracket_stack.pop().unwrap();
            program[start_pc] = Token::LeftBracket(curr_pc);
            program.push(Token::RightBracket(start_pc));
        } else {
            program.push(token_from_char(c));
        }

        curr_pc += 1;
    }
    if !bracket_stack.is_empty() {
        return None;
    }
    Some(program)
}

