use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use std::str::FromStr;

#[derive(Debug)]
pub enum ErrorKind {
    StdoutFlushError(String),
    StdinReadError(String),
    ParseError(String),
    ValidationError,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ValidationError => String::from("Unable to validate data"),
                Self::ParseError(e) => format!("Unable to parse data: {}", e),
                Self::StdoutFlushError(e) => format!("Unable to flush stdout: {}", e),
                Self::StdinReadError(e) => format!("Unable to read stdin: {}", e),
            }
        )
    }
}

pub fn prompt<Return, Prompt>(
    prompt: Prompt,
    validation: fn(&String) -> bool,
) -> Result<Return, ErrorKind>
where
    Prompt: Display,
    Return: FromStr,
    <Return as FromStr>::Err: ToString,
{
    let mut buffer = String::new();

    print!("{}: ", prompt);
    match stdout().flush() {
        Ok(_) => (),
        Err(e) => return Err(ErrorKind::StdoutFlushError(e.to_string())),
    };

    match stdin().read_line(&mut buffer) {
        Ok(_) => (),
        Err(e) => return Err(ErrorKind::StdinReadError(e.to_string())),
    };

    let buffer = buffer[..buffer.len() - 1].to_owned();

    if validation(&buffer) {
        return match buffer.parse() {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(ErrorKind::ParseError(e.to_string())),
        };
    }
    Err(ErrorKind::ValidationError)
}
