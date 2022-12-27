use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    name: String,
    perfect_score: u8,
    items: Vec<QuizItem>
}

impl Quiz {
    pub fn new(name: &str, perfect_score: u8, items: Vec<(&str, &str)>) -> Self {
        let items = items.iter().map(|&i| QuizItem::new(i.0, i.1)).collect();

        Self {
            name: name.to_string(),
            perfect_score,
            items
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizItem {
    prompt: String,
    answer: String
}

impl QuizItem {
    pub fn new(prompt: &str, answer: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            answer: answer.to_string()
        }
    }
}