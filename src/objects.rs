use std::{str::FromStr, fs, io};

pub enum ConfirmChoice {
    Yes,
    No
}

impl FromStr for ConfirmChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        if s == String::from("yes") {
            Ok(Self::Yes)
        } else if s == String::from("no") {
            Ok(Self::No)
        } else {
            Err(format!("Unknown choice: {:?}", s))
        }
    }
}

impl Into<bool> for ConfirmChoice {
    fn into(self) -> bool {
        match self {
            Self::Yes => true,
            Self::No => false
        }
    }
}

pub enum Action {
    Read,
    Write
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        if s == String::from("read") {
            Ok(Self::Read)
        } else if s == String::from("write") {
            Ok(Self::Write)
        } else {
            Err(format!("Unknown choice: {:?}", s))
        }
    }
}

impl Into<bool> for Action {
    fn into(self) -> bool {
        match self {
            Self::Write => true,
            Self::Read => false
        }
    }
}

pub struct ReadOnlyFile {
    file: String
}

impl TryInto<fs::File> for ReadOnlyFile {
    type Error = io::Error;

    fn try_into(self) -> Result<fs::File, Self::Error> {
        match fs::File::options().read(true).open(self.file) {
            Ok(f) => Ok(f),
            Err(e) => Err(e)
        }
    }
}

impl FromStr for ReadOnlyFile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self{ file: s.to_string() })
    }
}