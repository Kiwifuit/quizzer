use std::fmt::Display;
use std::io::{Read, Write};

use crate::quiz::Quiz;
use bincode::{deserialize, serialize};

#[derive(Debug)]
pub enum ErrorKind {
    SerializationError(String),
    DeserializationError(String),
    WriteError(String),
    ReadError(String),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DeserializationError(e) => format!("Unable to read data: {}", e),
                Self::SerializationError(e) => format!("Unable to write data: {}", e),
                Self::ReadError(e) => format!("Unable to read file: {}", e),
                Self::WriteError(e) => format!("Unable to write data to file: {}", e),
            }
        )
    }
}

pub fn save_quiz<F>(quiz: &Quiz, file: &mut F) -> Result<usize, ErrorKind>
where
    F: Write,
{
    let data = match serialize(quiz) {
        Ok(d) => d,
        Err(e) => return Err(ErrorKind::SerializationError(e.to_string())),
    };

    match file.write(&data) {
        Ok(size) => Ok(size),
        Err(e) => Err(ErrorKind::WriteError(e.to_string())),
    }
}

pub fn read_quiz<F>(file: &mut F) -> Result<Quiz, ErrorKind>
where
    F: Read,
{
    let mut raw = vec![];

    match file.read(&mut raw) {
        Ok(_) => (),
        Err(e) => return Err(ErrorKind::ReadError(e.to_string())),
    };

    let quiz = match deserialize(&raw) {
        Ok(q) => q,
        Err(e) => return Err(ErrorKind::DeserializationError(e.to_string())),
    };

    Ok(quiz)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn can_write() {
        let mut cursor = Cursor::new(vec![0u8]);

        let mut quiz = Quiz::new("Quiz Name", 5, 5);

        quiz.add_new("First Name", "Amogus Cronut");
        quiz.add_new("Last Name", "Suuuus");
        quiz.add_new("A?", "AAAAAAAAAAAAAAAAAAAAAAAAAAA");
        quiz.add_new("HMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM", "noic");
        quiz.add_new("lel", "waaaaaaaaaaaaaaaaaa");

        let res = save_quiz(&quiz, &mut cursor);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 233);
    }

    #[test]
    fn can_read() {
        let mut cursor = Cursor::new(vec![0u8]);

        let mut quiz = Quiz::new("Quiz Name", 5, 5);

        quiz.add_new("First Name", "Amogus Cronut");
        quiz.add_new("Last Name", "Suuuus");
        quiz.add_new("A?", "AAAAAAAAAAAAAAAAAAAAAAAAAAA");
        quiz.add_new("HMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM", "noic");
        quiz.add_new("lel", "waaaaaaaaaaaaaaaaaa");

        save_quiz(&quiz, &mut cursor).unwrap();

        let res = read_quiz(&mut cursor);

        dbg!(&res);

        if res.is_err() {
            dbg!(cursor.into_inner());
        }

        assert!(res.is_ok())
    }
}
