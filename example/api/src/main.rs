use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use patina::Patina;
use serde::Deserialize;

#[get("/{book}/{chapter}/{verse}")]
async fn get_verse(path: web::Path<(String, String, String)>) -> impl Responder {
    let (book, chapter, verse) = path.into_inner();

    let chapter: Option<u32> = if chapter.is_empty() {
        None
    } else {
        match chapter.parse() {
            Ok(num) => Some(num),
            Err(_) => return HttpResponse::BadRequest().body("Invalid chapter format"),
        }
    };

    let verse: Option<u32> = if verse.is_empty() {
        None
    } else {
        match verse.parse() {
            Ok(num) => Some(num),
            Err(_) => return HttpResponse::BadRequest().body("Invalid verse format"),
        }
    };

    // Pass book as &str, and chapter/verse as Option<u32>
    let result = Patina::new()
        .search_by_reference(&book, chapter, verse)
        .unwrap_or_else(|| "Reference not found.".to_string());

    HttpResponse::Ok().body(result)
}


#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

#[get("/")]
async fn search(q: web::Query<SearchQuery>) -> impl Responder {
    let search_query = &q.q;

    let result = Patina::new()
        .search_by_text(search_query)
        .unwrap_or_else(|| "No results found".to_string());

    HttpResponse::Ok().body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_book) // Route for /{book}
            .service(get_chapter) // Route for /{book}/{chapter}  
            .service(get_verse) // Route for /{book}/{chapter}/{verse}
            .service(search) // Route for /?query=text
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/{book}/{chapter}")]
async fn get_chapter(path: web::Path<(String, String)>) -> impl Responder {
    let (book, chapter) = path.into_inner();

    // Parse chapter as Option<u32>
    let chapter: Option<u32> = if chapter.is_empty() {
        None
    } else {
        match chapter.parse() {
            Ok(num) => Some(num),
            Err(_) => return HttpResponse::BadRequest().body("Invalid chapter format"),
        }
    };

    let result = Patina::new()
        .search_by_reference(&book, chapter, None)
        .unwrap_or_else(|| "Reference not found.".to_string());

    HttpResponse::Ok().body(result)
}

#[get("/{book}")]
async fn get_book(path: web::Path<String>) -> impl Responder {
    let book = path.into_inner();

    let result = Patina::new()
        .search_by_reference(&book, None, None)
        .unwrap_or_else(|| "Reference not found.".to_string());

    HttpResponse::Ok().body(result)
}
