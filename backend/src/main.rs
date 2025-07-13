use std::sync::Mutex;

use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    text: String,
}

#[get("/hey")]
async fn get_hey(data: web::Data<AppStateWithMessage>) -> web::Json<Message> {
    web::Json(Message {
        text: data.message.lock().unwrap().clone(),
    })
}

#[post("/hey")]
async fn post_hey(req_body: web::Json<Message>, data: web::Data<AppStateWithMessage>) -> impl Responder {
    let new_message = req_body.text.clone();
    let mut current_message = data.message.lock().unwrap();
    *current_message = new_message;
    HttpResponse::Ok()
}

struct AppStateWithMessage {
    message: Mutex<String>, // <- Mutex is necessary to mutate safely across threads
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let data = web::Data::new(AppStateWithMessage {
        message: Mutex::new(String::from("no message")),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(data.clone())
            .service(
                web::scope("/api")
                .service(get_hey)
                .service(post_hey)
            )
            // Put this last, else it will claim the entire "/" namespace and none of the other services under it will respond
            .service(
                Files::new("/", "./public")
                    .index_file("index.html")
                    .prefer_utf8(true),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
