use std::fmt::Display;
use std::io::{Write, stdout, stdin};

pub enum ErrorKind {
    StdoutFlushError(String),
    StdinReadError(String),
    ValidationError
}

pub fn prompt<Return, Prompt>(prompt:Prompt, validation: fn(&String) -> bool) -> Result<Return, ErrorKind>
    where
    Prompt: Display,
    Return: From<String> {
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
        return Ok(Return::from(buffer));
    }
    Err(ErrorKind::ValidationError)
}