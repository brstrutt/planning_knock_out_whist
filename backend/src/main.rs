use std::sync::Mutex;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;

mod state;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let data = web::Data::new(state::AppState {
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
                .service(api::hey::get_hey)
                .service(api::hey::post_hey)
                .service(api::session::post_connect)
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
