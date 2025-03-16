use serde::Deserialize;
use std::collections::HashMap;

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