use std::{fs, io, str::FromStr};

pub enum ConfirmChoice {
    Yes,
    No,
}

impl FromStr for ConfirmChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let s = s.as_str();

        if s == "yes" {
            Ok(Self::Yes)
        } else if s == "no" {
            Ok(Self::No)
        } else {
            Err(format!("Unknown choice: {:?}", s))
        }
    }
}

impl From<ConfirmChoice> for bool {
    fn from(val: ConfirmChoice) -> Self {
        match val {
            ConfirmChoice::Yes => true,
            ConfirmChoice::No => false,
        }
    }
}

pub enum Action {
    Read,
    Write,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let s = s.as_str();

        if s == "read" {
            Ok(Self::Read)
        } else if s == "create" || s == "write" {
            Ok(Self::Write)
        } else {
            Err(format!("Unknown choice: {:?}", s))
        }
    }
}

pub struct ReadOnlyFile {
    file: String,
}

impl TryInto<fs::File> for ReadOnlyFile {
    type Error = io::Error;

    fn try_into(self) -> Result<fs::File, Self::Error> {
        match fs::File::options().read(true).open(self.file) {
            Ok(f) => Ok(f),
            Err(e) => Err(e),
        }
    }
}

impl FromStr for ReadOnlyFile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            file: s.to_string(),
        })
    }
}
