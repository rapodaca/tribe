use std::io;

#[derive(Debug, PartialEq)]
pub enum Error {
    DateFormat(String),
    Io(io::ErrorKind),
    Filename,
    Extension(String),
    NoCitation,
}

impl From<chrono::ParseError> for Error {
    fn from(error: chrono::ParseError) -> Self {
        Self::DateFormat(error.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.kind())
    }
}

impl From<yaml_rust::ScanError> for Error {
    fn from(value: yaml_rust::ScanError) -> Self {
        todo!()
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        todo!()
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(value: handlebars::TemplateError) -> Self {
        todo!()
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self {
        println!("error {:?}", value);
        todo!()
    }
}