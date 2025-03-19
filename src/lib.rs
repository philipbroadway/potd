mod bible;
mod search;
mod utils;

use bible::Bible;
use search::find_verse;
use utils::kebob_to_title;

/// Struct to represent the Patina Bible search utility.
pub struct Patina {
    bible: Bible,
}

impl  Patina {

    pub fn new() -> Self {
        let bible_json = include_str!("data/bible.json");
        let bible: Bible = serde_json::from_str(bible_json).expect("Failed to parse JSON");
        Patina { bible }
    }

    pub fn search_by_reference<'a>(&self, book: &str, chapter: &str, verse: &str) -> Option<String>  {
      find_verse(&self.bible, &book, chapter, verse)
      .map(|verse_text| format!("{}", verse_text))
      .or_else(|| Some(format!("{} {}:{} not found.", kebob_to_title(&book), chapter, verse)))
        // find_verse(&self.bible, book, chapter, verse)
    }

    pub fn search_by_text(&self, query: &str) -> Option<String> {
        let query_parts: Vec<&str> = query.rsplitn(2, ' ').collect();
        if query_parts.len() != 2 {
            return None;
        }

        let chapter_verse = query_parts[0];
        let book = query_parts[1].to_lowercase().replace(" ", "-");

        let chapter_verse_parts: Vec<&str> = chapter_verse.split(':').collect();
        if chapter_verse_parts.len() != 2 {
            return None;
        }

        let chapter = chapter_verse_parts[0];
        let verse = chapter_verse_parts[1];

        find_verse(&self.bible, &book, chapter, verse)
            .map(|verse_text| format!("{}", verse_text))
            .or_else(|| Some(format!("{} {}:{} not found.", kebob_to_title(&book), chapter, verse)))
    }
}