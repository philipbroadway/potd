use crate::bible::Bible;

pub fn find_verse<'a>(bible: &'a Bible, book: &str, chapter: &str, verse: &str) -> Option<&'a str> {
  // println!("Searching for book: {}, chapter: {}, verse: {}", book, chapter, verse);

let book_entry = bible.books.get(book);
if book_entry.is_none() {
    println!("Book '{}' not found.", book);
    return None;
}

let chapter_entry = book_entry.unwrap().chapters.get(chapter);
if chapter_entry.is_none() {
    println!("Chapter '{}' not found in book '{}'.", chapter, book);
    return None;
}

let verse_entry = chapter_entry.unwrap().verses.get(verse);
if verse_entry.is_none() {
    println!("Verse '{}' not found in book '{}' chapter '{}'.", verse, book, chapter);
    return None;
}


  // println!("Found verse: {}", verse_entry.unwrap());
  Some(verse_entry.unwrap().as_str())
}