use serde::{Serialize, Deserialize};
use super::{Citation, Abstract};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Source {
    pub id: String,
    pub citation: Citation,
    pub r#abstract: Abstract,
}