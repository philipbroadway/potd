use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Bible {
    pub books: HashMap<String, Book>,
}

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub chapters: HashMap<String, Chapter>,
}

#[derive(Serialize, Deserialize)]
pub struct Chapter {
    pub verses: HashMap<String, String>,
}
