use std::{fs, path::PathBuf};

use handlebars::Handlebars;
use serde_json::json;
use tribe::{
    error::Error,
    source::{Abstract, Citation, Source},
};

#[derive(Debug, PartialEq)]
pub enum Artifact {
    Jpeg(Vec<u8>),
    Png(Vec<u8>),
    Pdf(Vec<u8>),
    Markdown(String),
    Text(String),
    Excel(Vec<u8>),
}

fn read_citation(path: std::path::PathBuf) -> Result<Citation, Error> {
    match fs::read_to_string(path.clone()) {
        Ok(string) => Ok(serde_yaml::from_str(&string)?),
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => Err(Error::NoCitation),
            kind => Err(Error::Io(kind)),
        },
    }
}

fn read_abstract(path: std::path::PathBuf) -> Result<Abstract, Error> {
    match fs::read_to_string(path) {
        Ok(string) => Ok(serde_yaml::from_str(&string)?),
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => return Ok(Abstract::new()),
            kind => return Err(Error::Io(kind)),
        },
    }
}

pub fn register_templates(reg: &mut Handlebars) -> Result<(), Error> {
    let mut path = PathBuf::new();

    path.push("views");

    path.push("partials");
    path.push("wrapper.hbs");
    reg.register_template_file("wrapper", &path)?;
    path.pop();
    path.push("footer.hbs");
    reg.register_template_file("footer", &path)?;
    path.pop();
    path.pop();

    path.push("layouts");
    path.push("index.hbs");
    reg.register_template_file("index", &path)?;
    path.pop();

    path.push("sources");
    path.push("index.hbs");
    reg.register_template_file("sources", &path)?;
    path.pop();
    path.push("source.hbs");
    reg.register_template_file("source", &path)?;

    Ok(())
}

pub fn build(sources: Vec<Source>) -> Result<(), Error> {
    let mut handlebars = Handlebars::new();

    register_templates(&mut handlebars)?;

    let context = json!({"sources": sources});
    let mut path = PathBuf::new();

    path.push("build");
    fs::create_dir(path.clone())?;
    path.push("index.html");
    fs::write(&path, handlebars.render("index", &context).unwrap())?;
    path.pop();
    path.push("sources");
    fs::create_dir(&path)?;

    for source in sources.iter() {
        let source_context = serde_json::to_value(source)?;
        path.push(&source.id);
        fs::create_dir(&path)?;
        path.push("index.html");
        fs::write(&path, handlebars.render("source", &source_context).unwrap())?;
        path.pop();
        path.pop();
    }

    path.push("index.html");
    fs::write(&path, handlebars.render("sources", &context).unwrap())?;

    Ok(())
}

pub fn main() -> Result<(), Error> {
    let source_paths = fs::read_dir("./sources")?;
    let mut sources = Vec::new();

    for source_path in source_paths {
        let base_path = source_path?;

        if base_path.file_type()?.is_dir() {
            sources.push(Source {
                id: base_path.file_name().to_str().ok_or(Error::Filename)?.to_string(),
                citation: read_citation(base_path.path().join("citation.yml"))?,
                r#abstract: read_abstract(base_path.path().join("abstract.yml"))?,
            });
        }
    }

    build(sources)
}
