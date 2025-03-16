/// A representation of the Bible in JSON format.
///
/// Expected JSON format:
/// ```json
/// {
/// "bookName": {
///     "chapterNumber": {
///         "verseNumber": "passageString",
///       }
///   }
/// }
/// ```

use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Bible {
    #[serde(flatten)]
    pub books: HashMap<String, Book>,
}

#[derive(Deserialize, Debug)]
pub struct Book {
    #[serde(flatten)]
    pub chapters: HashMap<String, Chapter>,
}

#[derive(Deserialize, Debug)]
pub struct Chapter {
    #[serde(flatten)]
    pub verses: HashMap<String, String>,
}