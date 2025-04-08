#[derive(Debug)]
pub enum Command {
    Next(usize),
    Print(PrintValue),
}

impl TryFrom<&str> for Command {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let i = value.find(' ').unwrap_or(value.len());
        let cmd = &value[0..i];
        let rest = value[i..].trim();

        match cmd {
            "pc" => Ok(Self::Print(PrintValue::ProgramCounter)),
            "n" | "next" => {
                let c = if rest.is_empty() { 1 } else { rest.parse::<usize>().map_err(|e| {
                        format!("Error while parsing: {}", e)
                    })?
                };
                Ok(Self::Next(c))
            },
            "p" | "print" => {
                Ok(Self::Print(PrintValue::try_from(rest)?))
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
    Memory(MemoryInterval, MemoryType),
}

#[derive(Debug)]
pub(crate) enum MemoryType {
    Instruction,
    Data,
}

impl TryFrom<&str> for PrintValue {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        let mut it = value.split_whitespace();   

        let mem_type: MemoryType;
        match it.nth(0).unwrap() {
            "im" | "instruction" => mem_type = MemoryType::Instruction,
            "dm" | "data" => mem_type = MemoryType::Data,
            "pc" | "ip" => return Ok(Self::ProgramCounter), 
            _ => return Err("invalid print argument".to_owned()),
        };

        let mem_interval = MemoryInterval::try_from(it.nth(0).unwrap())?; 
        Ok(Self::Memory(mem_interval, mem_type))
    }
}

#[derive(Debug, Default)]
pub(crate) struct MemoryInterval(usize, usize);

impl TryFrom<&str> for MemoryInterval {
    type Error = String; 

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains(':') {
            // safe to unwrap since value for sure contains ':'
            let (s, e) = value.split_once(':').unwrap();
            Ok(MemoryInterval(s.parse().map_err(|e| format!("Error while parsing: {}", e))?,
                e.parse().map_err(|e| format!("Error while parsing: {}", e))?))
        } else {
            let loc = value.parse::<usize>().map_err(|e| {
                format!("Error while parsing: {}", e)
            })?;
            Ok(MemoryInterval(loc, loc + 1))
        }
    }
}
