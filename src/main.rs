use bincode::{serialize, deserialize};

mod quiz;

fn main() {
    let quizzes = vec![("What is 9 + 10", "21"), ("Burgur", "yes Burgur")];

    let quiz = quiz::Quiz::new("Third Periodical Exam", 100, quizzes);

    let data = match serialize(&quiz) {
        Ok(d) => d,
        Err(e) => panic!("Erorr while seriaizing: {:?}", e)
    };

    let quiz = match deserialize::<quiz::Quiz>(&data[..]) {
        Ok(q) => q,
        Err(e) => panic!("Error while deserializing: {:?}", e)
    };

    println!("Size of data: {}", data.len());
    dbg!(quiz);
}
