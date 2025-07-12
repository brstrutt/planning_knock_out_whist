use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use actix_files::Files;

#[get("/hellow_world")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                .service(hello)
                .service(echo)
                .route("/hey", web::get().to(manual_hello))
            )
            // Put this last, else it will claim the entire "/" namespace and none of the other services under it will respond
            .service(Files::new("/", "./public").index_file("index.html").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
