use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    pub items: Vec<QuizItem>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizItem {
    prompt: String,
    answer: String
}

impl QuizItem {
    pub fn new(prompt: String, answer: String) -> Self {
        Self {
            prompt,
            answer
        }
    }
}