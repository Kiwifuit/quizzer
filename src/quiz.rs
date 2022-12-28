use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn get_item_at(&mut self, index: &usize) -> Option<QuizItem> {
        let content = self.clone().content;
        let item = content.get(*index);

        self.content.remove(*index);

        Some(item.unwrap().to_owned())
    }

    pub fn get_perfect_score(&self) -> u8 {
        self.perfect_score
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn count(&self) -> usize {
        self.items as usize
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct QuizItem {
    prompt: String,
    answer: String
}