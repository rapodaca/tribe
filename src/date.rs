use std::str::FromStr;
use serde::{Serialize, Deserialize};

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Date(chrono::NaiveDate);

impl Date {
    pub fn new(source: &str) -> Result<Self, Error> {
        Ok(Self(chrono::NaiveDate::from_str(source)?))
    }
}
