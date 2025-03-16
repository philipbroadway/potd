use crate::bible::Bible;

pub fn find_verse<'a>(bible: &'a Bible, book: &str, chapter: &str, verse: &str) ->  Option<&'a str> {
    bible.books.get(book)?
        .chapter.get(chapter)?
        .verse.get(verse)
        .map(String::as_str)
}