use std::fs;

use kin::{date::Date, error::Error};
use yaml_rust::YamlLoader;

#[derive(Debug, PartialEq)]
pub struct Source {
    pub citation: Citation,
    pub r#abstract: Abstract,
}

#[derive(Debug, PartialEq)]
pub enum Artifact {
    Jpeg(Vec<u8>),
    Png(Vec<u8>),
    Pdf(Vec<u8>),
    Markdown(String),
    Text(String),
    Excel(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub struct Citation {
    date: Option<Date>,
    title: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Abstract {

}

fn read_citation(path: std::path::PathBuf) -> Result<Citation, Error> {
    match fs::read_to_string(path) {
        Ok(string) => {
            let yaml = YamlLoader::load_from_str(&string)?;

            Ok(Citation {
                date: match &yaml[0]["date"].as_str() {
                    Some(date) => Some(Date::new(date)?),
                    None => None
                },
                title: yaml[0]["title"].as_str().map(|t| t.to_string())
            })
        },
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => Err(Error::NoCitation),
            kind => Err(Error::Io(kind))
        }
    }
}

fn read_abstract(path: std::path::PathBuf) -> Result<Abstract, Error> {
    match fs::read_to_string(path) {
        Ok(string) => {
            // 1. parse string
            let yaml = YamlLoader::load_from_str(&string)?;

            Ok(Abstract {
                // 2. pull fields from yaml
            })
        },
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                return Ok(Abstract { })
            },
            kind => return Err(Error::Io(kind))
        }
    }   
}

pub fn main() -> Result<(), Error> {
    let source_paths = fs::read_dir("./sources")?;
    let mut sources = Vec::new();

    for source_path in source_paths {
        println!("source {:?}", source_path);
        let base_path = source_path?;

        if base_path.file_type()?.is_dir() {
            let citation = read_citation(base_path.path().join("citation.yml"))?;
            let r#abstract = read_abstract(base_path.path().join("abstract.yml"))?;

            sources.push(Source { citation, r#abstract: r#abstract })
            // Instead of iterating over all files, find the files:
            // 1. citation.yml
            // let citation_path = base_path.path().join("citation.yml");

            // 2. abstract.yml
            // 3. source.*
            // for item_path in item_paths {
            //     let item_path = item_path?.path();
            //     let item_name = item_path
            //         .file_name()
            //         .and_then(OsStr::to_str)
            //         .ok_or(Error::Filename)?;

            // match item_name {
            //     "citation.yml" => {

            //         // let citation = read_citation(item_path)?;
            //         // citation = Some(read_citation(item_path)?);
            //         citation.replace(read_citation(item_path)?);
            //     },
            //     "abstract.yml" => {
            //         // process abstract
            //     },
            //     _ => {
            //         let extension = item_path
            //             .extension()
            //             .and_then(OsStr::to_str)
            //             .ok_or(Error::Filename)?;

            //         let artifact = match extension {
            //             "png" => Artifact::Png(vec![]),
            //             "jpg" => Artifact::Jpeg(vec![]),
            //             "jpeg" => Artifact::Jpeg(vec![]),
            //             "md" => Artifact::Markdown(String::new()),
            //             "pdf" => Artifact::Pdf(vec![]),
            //             "xls" => Artifact::Excel(vec![]),
            //             "txt" => Artifact::Text(String::new()),
            //             _ => return Err(Error::Extension(extension.to_string()))
            //         };
            //     }
            // }

            // }

            // sources.push(Source {
            //     citation: citation.ok_or(Error::NoCitation)?,
            // });
        }
    }

    Ok(())
}
