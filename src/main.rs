mod bible;
mod search;

use std::env;
use bible::Bible;
use search::find_verse;

fn main() {

  let args: Vec<String> = env::args().collect();

  if args.len() > 0 {
      eprintln!("------------------\nPassage of the day\n------------------");
      eprintln!("\nOptions:\n{} -s <book> <chapter> <verse>", args[0]);
      eprintln!("{} -q \"book chapter:verse[,-s]\"\n", args[0]);
      return;
  }
  
  //TODO: Setup CLI flags for different query options.

  let book = &args[2].to_lowercase().replace(" ", "-");
  let chapter = &args[3];
  let verse = &args[4];

  let bible_json = include_str!("data/bible.json");
  let bible: Bible = serde_json::from_str(bible_json).expect("Failed to parse JSON");

  match find_verse(&bible, book, chapter, verse) {
    Some(verse_text) => println!("{}", verse_text),
    None => println!("{} {}:{} not found.", book, chapter, verse),
  }
}
