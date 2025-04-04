#[derive(Debug)]
pub enum Token {
    RightArrow,
    LeftArrow,
    Plus,
    Minus,
    LeftBracket,
    RightBracket,
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
        '[' => LeftBracket,
        ']' => RightBracket,
        ',' => Comma,
        '.' => Dot,
        _   => unreachable!(),
    }
}

pub fn to_tokens(file_content: &str) -> Vec<Token> {
    file_content
        .chars()
        .filter_map(|c| {
            if !is_valid_token(c) {
                return None;
            }
            Some(token_from_char(c))
        })
        .collect()
}

