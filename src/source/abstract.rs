use serde::{Serialize, Deserialize};

use super::Article;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Abstract {
    pub articles: Option<Vec<Article>>
}

impl Abstract {
    pub fn new() -> Self {
        Self {
            articles: None
        }
    }
}