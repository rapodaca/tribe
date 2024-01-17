use serde::{Serialize, Deserialize};

use crate::Date;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Citation {
    pub date: Option<Date>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub accessed: Option<Date>,
}