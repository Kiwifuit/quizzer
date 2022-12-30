use std::fs::File;
use std::process::exit;

mod data;
mod objects;
mod prompt;
mod quiz;

const SCORE_CAP: u8 = 100;

fn get_perfect_score() -> u8 {
    loop {
        let score = prompt::prompt::<u8, &str>("Enter the perfect score", |score| {
            let score = score.parse::<u8>();

            if score.is_err() {
                return false;
            } else if score.unwrap() > SCORE_CAP {
                return false;
            } else {
                true
            }
        });

        if let Err(e) = score {
            eprintln!("An error occurred while getting user input: {}", e);

            match e {
                prompt::ErrorKind::ParseError(_) | prompt::ErrorKind::ValidationError => continue,
                prompt::ErrorKind::StdinReadError(_) | prompt::ErrorKind::StdoutFlushError(_) => {
                    exit(1)
                }
            };
        }

        return score.unwrap();
    }
}

fn get_quiz_count() -> u8 {
    loop {
        let count =
            prompt::prompt::<u8, &str>(
                "Enter the amount of quiz items",
                |_| match prompt::prompt::<objects::ConfirmChoice, &str>(
                    "Are you sure? This cannot be changed until you restart the program (yes/no)",
                    |_| true,
                ) {
                    Ok(c) => c.into(),
                    Err(e) => {
                        eprintln!(
                        "An error occurred while trying to confirm the amount of quiz items: {}",
                        e
                    );
                        exit(1)
                    }
                },
            );

        if let Err(e) = count {
            eprintln!("An error occurred while getting user input: {}", e);

            match e {
                prompt::ErrorKind::ParseError(_) | prompt::ErrorKind::ValidationError => continue,
                prompt::ErrorKind::StdinReadError(_) | prompt::ErrorKind::StdoutFlushError(_) => {
                    exit(2)
                }
            };
        }

        return count.unwrap();
    }
}

fn get_name() -> String {
    // We return true on all cases because we dont need to validate anything
    // Also we use String here instead of &'a str because &str does not implement FromStr, which
    // is the constraint in `prompt`'s `Return` type
    let name = prompt::prompt::<String, &str>("Enter quiz name", |_| true);

    if let Err(e) = &name {
        match e {
            prompt::ErrorKind::StdoutFlushError(_) | prompt::ErrorKind::StdinReadError(_) => {
                eprintln!("Error while reading/writing to the terminal: {}", e);
                exit(1)
            }
            prompt::ErrorKind::ParseError(_) | prompt::ErrorKind::ValidationError => (),
        };
    }

    name.unwrap()
}

fn store_quiz() {
    let name = get_name();
    let score = get_perfect_score();
    let count = get_quiz_count();
    let mut quiz = quiz::Quiz::new(name.as_str(), score, count);

    for number in 0..count {
        let question =
            prompt::prompt::<String, String>(format!("Enter question #{}", number), |_| true);

        if let Err(e) = &question {
            match e {
                prompt::ErrorKind::StdoutFlushError(_) | prompt::ErrorKind::StdinReadError(_) => {
                    eprintln!("Error while reading/writing to the terminal: {}", e);
                    exit(3)
                }
                prompt::ErrorKind::ParseError(_) | prompt::ErrorKind::ValidationError => (),
            };
        }

        let answer =
            prompt::prompt::<String, String>(format!("Enter answer #{}", number), |_| true);

        if let Err(e) = &answer {
            match e {
                prompt::ErrorKind::StdoutFlushError(_) | prompt::ErrorKind::StdinReadError(_) => {
                    eprintln!("Error while reading/writing to the terminal: {}", e);
                    exit(3)
                }
                prompt::ErrorKind::ParseError(_) | prompt::ErrorKind::ValidationError => (),
            };
        }

        quiz.add_new(question.unwrap(), answer.unwrap());
    }

    let mut save_file = match File::options()
        .write(true)
        .create_new(true)
        .open(format!("{}.quiz", name))
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("An error occurred while trying to open the file: {}", e);
            exit(9);
        }
    };

    if let Err(e) = data::save_quiz(&quiz, &mut save_file) {
        eprintln!(
            "An error occurred while trying to save quiz to a file: {}",
            e
        );
        exit(5)
    }
}

fn read_quiz() {
    let file = prompt::prompt::<objects::ReadOnlyFile, &str>("Enter file to read: ", |_| true);

    if let Err(e) = &file {
        eprintln!(
            "{}",
            match e {
                prompt::ErrorKind::ParseError(e) => String::from(e),
                other => format!(
                    "An unexpected error occurred while getting a RO file: {}",
                    other
                ),
            }
        );

        exit(7)
    }

    let file = file.unwrap();
    let mut quiz = match data::read_quiz::<File>(&mut file.try_into().unwrap()) {
        Ok(q) => q,
        Err(e) => {
            match e {
                data::ErrorKind::DeserializationError(err) => {
                    eprintln!("An error occurred while deserializing the data: {}", err)
                }
                _false_positive => (),
            };

            exit(8);
        }
    };

    println!(
        "Quiz Name: {}\nPerfect Score: {}\n",
        quiz.get_name(),
        quiz.get_perfect_score()
    );

    for indx in 0..quiz.count() {
        let item = quiz.get_item_at(&indx);

        if let Some(item) = item {
            println!(
                "Question #{}:\n\t{:?}\n\tAnswer: {:?}",
                indx,
                item.get_prompt(),
                item.get_answer()
            )
        }
    }
}

fn main() {
    let action = prompt::prompt::<objects::Action, &str>("Enter action (read/write)", |_| true);

    if let Err(e) = action {
        eprintln!(
            "An error occurred while trying to request for an action: {}",
            e
        );
        exit(6)
    }

    match action.unwrap() {
        objects::Action::Read => read_quiz(),
        objects::Action::Write => store_quiz(),
    };
}
