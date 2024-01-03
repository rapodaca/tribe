use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, PartialEq)]
pub struct Date(chrono::NaiveDate);

impl Date {
    pub fn new(source: &str) -> Result<Self, Error> {
        Ok(Self(chrono::NaiveDate::from_str(source)?))
    }
}