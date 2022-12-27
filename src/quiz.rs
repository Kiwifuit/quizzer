use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    name: String,
    perfect_score: u8,
    items: u8,
    content: Vec<QuizItem>
}

impl Quiz {
    pub fn new(name: &str, perfect_score: u8, items: u8) -> Self {
        Self {
            name: name.to_string(),
            perfect_score,
            items,
            content: Vec::new()
        }
    }

    pub fn add_new<Entry>(&mut self, prompt: Entry, answer: Entry)
        where
        Entry: ToString
        {
            self.content.push(QuizItem { prompt: prompt.to_string(), answer: answer.to_string() });
        }
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