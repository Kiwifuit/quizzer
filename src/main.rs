use std::{process::exit, str::FromStr};

mod quiz;
mod data;
mod prompt;

const SCORE_CAP: u8 = 100;

enum ConfirmChoice {
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

fn get_perfect_score() -> u8 {
    loop {
        let score = prompt::prompt::<u8, &str>("Enter the perfect score", |score| {
            if score.parse::<u8>().unwrap() >= SCORE_CAP {
                return false;
            }
            true
        });

        if let Err(e) = score {
            eprintln!("An error occurred while getting user input: {}", e);

            match e {
                prompt::ErrorKind::ParseError | prompt::ErrorKind::ValidationError => continue,
                prompt::ErrorKind::StdinReadError(_) | prompt::ErrorKind::StdoutFlushError(_) => exit(1)
            };
        }

        return score.unwrap()
    }
}

fn get_quiz_count() -> u8 {
    loop {
        let count = prompt::prompt::<u8, &str>("Enter the perfect score", |score| {
            match prompt::prompt::<ConfirmChoice, &str>("Are you sure? This cannot be changed until you restart the program (yes/no)", |_| true) {
                Ok(c) => c.into(),
                Err(e) => {
                    eprintln!("An error occurred while trying to confirm the perfect score: {}", e);
                    exit(1)
                }
            }
        });

        if let Err(e) = count {
            eprintln!("An error occurred while getting user input: {}", e);

            match e {
                prompt::ErrorKind::ParseError | prompt::ErrorKind::ValidationError => continue,
                prompt::ErrorKind::StdinReadError(_) | prompt::ErrorKind::StdoutFlushError(_) => exit(2)
            };
        }

        return count.unwrap()
    }
}

fn get_name() -> String {
    // We return true on all cases because we dont need to validate anything
    // Also we use String here instead of &'a str because &str does not implement FromStr, which
    // is the constraint in `prompt`'s `Return` type
    let name = prompt::prompt::<String, &str>("Enter your name", |_| true);

    if let Err(e) = &name {
        match e {
            prompt::ErrorKind::StdoutFlushError(_) | prompt::ErrorKind::StdinReadError(_) => {eprintln!("Error while reading/writing to the terminal: {}", e); exit(1)}
            prompt::ErrorKind::ParseError | prompt::ErrorKind::ValidationError => ()
        };
    }

    name.unwrap()
}

fn main() {
    let name = get_name();
    let score = get_perfect_score();
    let count = get_quiz_count();
    let quiz = quiz::Quiz::new(name.as_str(), score, count);
}
