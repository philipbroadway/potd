mod bible;
mod search;
mod utils;

use std::env;
use bible::Bible;
use search::find_verse;
use utils::kebob_to_title;

fn main() {

  let args: Vec<String> = env::args().collect();

  if args.len() <= 1 {
      eprintln!("------------------\nPassage of the day\n------------------");
      eprintln!("\nU{} -s <book> <chapter> <verse>", args[0]);
      eprintln!("{} -q \"book chapter:verse[,-s]\"\n", args[0]);
      return;
  }

  let bible_json = include_str!("data/bible.json");
  let bible: Bible = serde_json::from_str(bible_json).expect("Failed to parse JSON");

  if args[1] == "-s" {
    let book = &args[2].to_lowercase().replace(" ", "-");
    let chapter = &args[3];
    let verse = &args[4];
  
    match find_verse(&bible, book, chapter, verse) {
      Some(verse_text) => println!("{}", verse_text),
      None => println!("{} {}:{} not found.", book, chapter, verse),
    }
    return;
  } else if args[1] == "-q" {
    let query = &args[2];
    let query_parts: Vec<&str> = query.rsplitn(2, ' ').collect();
    if query_parts.len() != 2 {
        eprintln!("Invalid query format. Expected format: \"<book> <chapter:verse>\"");
        return;
    }

    let chapter_verse = query_parts[0];
    let book = query_parts[1].to_lowercase().replace(" ", "-");

    let chapter_verse_parts: Vec<&str> = chapter_verse.split(':').collect();
    if chapter_verse_parts.len() != 2 {
        eprintln!("Invalid chapter:verse format. Expected format: \"<chapter:verse>\"");
        return;
    }

    let chapter = chapter_verse_parts[0];
    let verse = chapter_verse_parts[1];

    match find_verse(&bible, &book, chapter, verse) {
        Some(verse_text) => println!("{}", verse_text),
        None => println!("{} {}:{} not found.", kebob_to_title(&book), chapter, verse),
    }
    return; 
  }
} 
