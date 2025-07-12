use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use actix_files::Files;
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    text: String,
}

#[get("/hey")]
async fn hey() -> web::Json<Message> {
    web::Json(
        Message {
            text: String::from("HeloooOOOooOOO")
        }
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                .service(hey)
            )
            // Put this last, else it will claim the entire "/" namespace and none of the other services under it will respond
            .service(Files::new("/", "./public").index_file("index.html").prefer_utf8(true))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
