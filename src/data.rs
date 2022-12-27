use std::{fs::File, io::{Write, Read}};

use bincode::{serialize, deserialize};
use crate::quiz::Quiz;

#[derive(Debug)]
pub enum ErrorKind {
    SerializationError(String),
    DeserializationError(String),
    WriteError(String),
    ReadError(String),
}

pub fn save_quiz(quiz: &Quiz, file: &mut File) -> Result<usize, ErrorKind>{
    let data = match serialize(quiz) {
        Ok(d) => d,
        Err(e) => return Err(ErrorKind::SerializationError(e.to_string()))
    };

    match file.write(&data) {
        Ok(size) => Ok(size),
        Err(e) => Err(ErrorKind::WriteError(e.to_string()))
    }
}

pub fn read_quiz(file: &mut File) -> Result<Quiz, ErrorKind> {
    let mut raw = vec![];

    match file.read(&mut raw) {
        Ok(_) => (),
        Err(e) => return Err(ErrorKind::ReadError(e.to_string()))
    };

    let quiz = match deserialize(&raw) {
        Ok(q) => q,
        Err(e) => return Err(ErrorKind::DeserializationError(e.to_string()))
    };

    Ok(quiz)
}