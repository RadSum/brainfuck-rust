#[derive(Debug)]
pub enum Command {
    Next(usize),
    Print(PrintValue),
}

impl TryFrom<&str> for Command {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut it = value.split_whitespace();

        let cmd = it.nth(0).ok_or_else(|| {
            "Empty line is not allowed".to_string()  
        })?;

        match cmd {
            "n" | "next" => {
                let c = it.nth(0).unwrap_or("1").parse::<usize>().map_err(|e| {
                    format!("Error while parsing: {}", e)
                })?;
                Ok(Command::Next(c))
            },
            "p" | "print" => {
                let to_p = it.nth(0).ok_or("There has to be an argument to `print`")?;
                Ok(Command::Print(PrintValue::Debug(to_p.into())))
            },
            _ => Err(format!("{cmd} is not a command")),
        } 
    }
}

impl TryFrom<String> for Command {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::try_from(value.as_str())?)
    }
}

#[derive(Debug)]
pub enum PrintValue {
    ProgramCounter,
    DataMemory(MemoryInterval),
    InstMemory(MemoryInterval),
    Debug(String),
}

type MemoryInterval = (usize, usize);
