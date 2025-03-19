use serde::Deserialize;

#[get("/{book}/{chapter}/{verse}")]
async fn get_verse(path: web::Path<(String, String, String)>) -> impl Responder {
    let (book, chapter, verse) = path.into_inner();

    let result = Patina::new()
        .search_by_reference(&book, &chapter, &verse)
        .unwrap_or_else(|| "Verse not found".to_string());

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
            .service(get_verse) // Route for /{book}/{chapter}/{verse}
            .service(search) // Route for /?query=text
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}