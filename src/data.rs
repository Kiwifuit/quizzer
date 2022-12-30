use std::fmt::Display;
use std::io::{Read, Write};

use crate::quiz::Quiz;
use bincode::{deserialize_from, serialize_into};

#[derive(Debug)]
pub enum ErrorKind {
    SerializationError(bincode::ErrorKind),
    DeserializationError(bincode::ErrorKind),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DeserializationError(e) => format!("Unable to read data: {}", e),
                Self::SerializationError(e) => format!("Unable to write data: {}", e),
            }
        )
    }
}

pub fn save_quiz<F>(quiz: &Quiz, file: &mut F) -> Result<(), ErrorKind>
where
    F: Write,
{
    match serialize_into(file, quiz) {
        Ok(_) => Ok(()),
        Err(e) => Err(ErrorKind::SerializationError(*e)),
    }
}

pub fn read_quiz<F>(file: &mut F) -> Result<Quiz, ErrorKind>
where
    F: Read,
{
    match deserialize_from(file) {
        Ok(q) => Ok(q),
        Err(e) => Err(ErrorKind::DeserializationError(*e)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{Cursor, Seek};

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
    }

    #[test]
    fn can_read() {
        let mut cursor = Cursor::new(vec![]);

        let mut quiz = Quiz::new("Quiz Name", 5, 5);

        quiz.add_new("First Name", "Amogus Cronut");
        quiz.add_new("Last Name", "Suuuus");
        quiz.add_new("A?", "AAAAAAAAAAAAAAAAAAAAAAAAAAA");
        quiz.add_new("HMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM", "noic");
        quiz.add_new("lel", "waaaaaaaaaaaaaaaaaa");

        save_quiz(&quiz, &mut cursor).unwrap();

        cursor.seek(std::io::SeekFrom::Start(0));

        let res = read_quiz(&mut cursor);

        dbg!(&res);

        if res.is_err() {
            dbg!(cursor.into_inner());
        }

        assert!(res.is_ok())
    }
}
