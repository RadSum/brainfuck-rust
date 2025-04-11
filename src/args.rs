use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Brainfuck source file
    #[arg(required = true)]
    pub file: String, 

    /// Flag whether to run in interactive mode
    #[arg(short, long)]
    pub interactive: bool,
}
