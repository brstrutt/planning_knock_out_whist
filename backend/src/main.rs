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
async fn get_hey(data: web::Data<AppState>) -> web::Json<Message> {
    web::Json(Message {
        text: data.message.lock().unwrap().clone(),
    })
}

#[post("/hey")]
async fn post_hey(req_body: web::Json<Message>, data: web::Data<AppState>) -> impl Responder {
    let new_message = req_body.text.clone();
    let mut current_message = data.message.lock().unwrap();
    *current_message = new_message;
    HttpResponse::Ok()
}


#[derive(Deserialize)]
struct ConnectData {
    session_uuid: String,
}

const MAX_SESSIONS: usize = 20;

#[post("/connect")]
async fn post_connect(req_body: web::Json<ConnectData>, data: web::Data<AppState>) -> impl Responder {
    let session_uuid = req_body.session_uuid.clone();
    let mut current_sessions = data.sessions.lock().unwrap();

    let session_already_active = current_sessions.iter().any(|session| session.uuid == session_uuid);
    if session_already_active {
        return HttpResponse::Ok().body("Already active");
    }

    if current_sessions.len() >= MAX_SESSIONS {
        return HttpResponse::Ok().body("Too many sessions");
    }

    current_sessions.push(Session{uuid: session_uuid});
    HttpResponse::Ok().body("New session")
}

struct Session {
    uuid: String
}

struct AppState {
    message: Mutex<String>,
    sessions: Mutex<Vec<Session>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let data = web::Data::new(AppState {
        message: Mutex::new(String::from("no message")),
        sessions: Mutex::new(Vec::new()),
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
                .service(post_connect)
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
