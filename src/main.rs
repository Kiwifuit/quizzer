use std::process::exit;

mod quiz;
mod data;
mod prompt;

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

fn get_name<'a>() -> &'a str {
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

    name.unwrap().as_str()
}

fn main() {
    let name = get_name();
    let score = get_perfect_score();
    let quiz = quiz::Quiz::new(name, score, items);
}
