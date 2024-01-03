pub mod date;
pub mod error;

pub use date::Date;
pub use error::Error;
// #[derive(Debug, PartialEq)]
// pub struct Persona {

// }

// #[derive(Debug, PartialEq)]
// pub struct Url {

// }

// #[derive(Debug, PartialEq)]
// pub enum Date {
//     Day, // e.g., Jan. 1, 1970
//     Month,  // e.g., Jan. 1970
//     Year,  // e.g., 1970
// }

// #[derive(Debug, PartialEq)]
// pub enum Time {
//     On(Date),
//     About(Date),
//     Before(Date),
//     After(Date),
//     Between(Date, Date),
// }

// #[derive(Debug, PartialEq)]
// pub enum Place {
//     // US Settlement e.g., San Francisco, San Francisco County, California
//     // World Settlement e.g., Paris, France
//     // US County, e.g., Marion County, Oregon
//     // US State, e.g., Nevada
//     // US Address (???, Rural Route)
// }

// #[derive(Debug, PartialEq)]
// pub struct Birth {
//     persona: Persona,
//     mother: Option<Persona>,
//     time: Option<Time>,
//     place: Option<Place>,
// }

// #[derive(Debug, PartialEq)]
// pub struct Abstract {
//     births: Vec<Birth>,
// }

// #[derive(Debug, PartialEq)]
// pub enum Repository {
//     DatabaseWithImages {
//         name: String,
//         url: Url,
//         custodian: String,
//     },
//     Database {
//         name: String,
//         url: String,
//     },
//     Book {

//     },
//     RareBook {
//       // Collection
//     },
// }

// #[derive(Debug, PartialEq)]
// pub struct Citation {
//     date: String,
//     title: Option<String>,
//     subtitle: String,
//     author: String,
//     repository: Option<Repository>,
// }

// #[derive(Debug, PartialEq)]
// pub enum Artifact {
//     Jpeg(Vec<u8>),
//     Png(Vec<u8>),
//     Pdf(Vec<u8>),
//     Markdown(String),
//     Text(String)
// }

// #[derive(Debug, PartialEq)]
// pub struct Source {
//     pub r#abstract: Abstract,
//     pub citation: Citation,
//     pub artifact: Option<Artifact>
// }