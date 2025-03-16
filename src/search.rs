// use crate::bible::Bible;

// pub fn find_verse(bible: &Bible, book: &str, chapter: u32, verse: u32) -> Option<&str> {
//     bible.books.get(book)?
//         .chapter.get(&chapter)?
//         .verse.get(&verse)
//         .map(String::as_str)
// }