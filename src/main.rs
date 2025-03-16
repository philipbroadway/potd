mod bible;
mod search;

use std::fs;
use bible::Bible;
use search::find_verse;

fn main() {
    let bible_json = include_str!("data/bible.json");
    let bible: Bible = serde_json::from_str(bible_json).expect("Failed to parse JSON");

    println!("Bible loaded with {} books.", bible.books.len());
}
