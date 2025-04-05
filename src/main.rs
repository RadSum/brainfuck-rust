use std::process::exit;

mod tokenizer;
use tokenizer::to_tokens;

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
        for token in tokens {
            println!("{:?}", token);
        }
    } else {
        eprintln!("Error while parsing");
    }
}

