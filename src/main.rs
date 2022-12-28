use std::{process::exit, fs::File};

mod quiz;
mod data;
mod prompt;
mod objects;

const SCORE_CAP: u8 = 100;

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
        let count = prompt::prompt::<u8, &str>("Enter the perfect score", |_| {
            match prompt::prompt::<objects::ConfirmChoice, &str>("Are you sure? This cannot be changed until you restart the program (yes/no)", |_| true) {
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

fn store_quiz() {
    let name = get_name();
    let score = get_perfect_score();
    let count = get_quiz_count();
    let mut quiz = quiz::Quiz::new(name.as_str(), score, count);

    for _ in 0..count {
        let question = prompt::prompt::<String, String>(format!("Enter question #{}", &count), |_| true);

        if let Err(e) = &question {
            match e {
                prompt::ErrorKind::StdoutFlushError(_) | prompt::ErrorKind::StdinReadError(_) => {eprintln!("Error while reading/writing to the terminal: {}", e); exit(3)}
                prompt::ErrorKind::ParseError | prompt::ErrorKind::ValidationError => ()
            };
        }

        let answer = prompt::prompt::<String, String>(format!("Enter answer #{}", &count), |_| true);

        if let Err(e) = &answer {
            match e {
                prompt::ErrorKind::StdoutFlushError(_) | prompt::ErrorKind::StdinReadError(_) => {eprintln!("Error while reading/writing to the terminal: {}", e); exit(3)}
                prompt::ErrorKind::ParseError | prompt::ErrorKind::ValidationError => ()
            };
        }

        quiz.add_new(question.unwrap(), answer.unwrap());
    }

    let mut save_file = File::options().write(true).open(format!("{}.quiz", name)).unwrap();

    if let Err(e) = data::save_quiz(&quiz, &mut save_file) {
        eprintln!("An error occurred while trying to save quiz to a file: {}", e);
        exit(5)
    }
}

fn main() {
    let action = prompt::prompt::<objects::Action, &str>("Enter action (read/write)", |_| true);

    if let Err(e) = action {
        eprintln!("An error occurred while trying to request for an action: {}", e);
        exit(6)
    }

    if action.unwrap().into() {
        store_quiz()
    }
}
