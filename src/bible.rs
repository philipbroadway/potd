use serde::Deserialize;
use std::collections::HashMap;

/// A representation of the Bible in JSON format.
///
/// Expected JSON format:
/// ```json
/// {
/// "bookName": {
///   "chapter" : {
///     "chapterNumber": {
///       "verse": {
///         "verseNumber": "passageString",
///       }
///     }
///   }
/// }
/// ```

#[derive(Debug, Deserialize)]
pub struct Bible {
    #[serde(flatten)]
    pub books: HashMap<String, Book>,
}

#[derive(Debug, Deserialize)]
pub struct Book {
    pub chapter: HashMap<String, Chapter>,
}

#[derive(Debug, Deserialize)]
pub struct Chapter {
    pub verse: HashMap<String, String>,
}