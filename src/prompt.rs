use std::fmt::{Display, write};
use std::io::{Write, stdout, stdin};
use std::str::FromStr;

#[derive(Debug)]
pub enum ErrorKind {
    StdoutFlushError(String),
    StdinReadError(String),
    ParseError,
    ValidationError
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::ParseError => String::from("Unable to parse data"),
            Self::ValidationError => String::from("Unable to validate data"),
            Self::StdoutFlushError(e) => format!("Unable to flush stdout: {}", e),
            Self::StdinReadError(e) => format!("Unable to read stdin: {}", e)
        })
    }
}

pub fn prompt<Return, Prompt>(prompt:Prompt, validation: fn(&String) -> bool) -> Result<Return, ErrorKind>
    where
    Prompt: Display,
    Return: FromStr {
    let mut buffer = String::new();

    print!("{}: ", prompt);
    match stdout().flush() {
        Ok(_) => (),
        Err(e) => return Err(ErrorKind::StdoutFlushError(e.to_string()))
    };

    match stdin().read_line(&mut buffer) {
        Ok(_) => (),
        Err(e) => return Err(ErrorKind::StdinReadError(e.to_string()))
    };

    if validation(&buffer) {
        return match buffer.parse() {
            Ok(parsed) => Ok(parsed),
            Err(_) => Err(ErrorKind::ParseError)
        };
    }
    Err(ErrorKind::ValidationError)
}