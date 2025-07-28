use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use env_logger::Env;

mod api;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let data = web::Data::new(state::AppState::default());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(data.clone())
            .service(web::scope("/api").service(api::users::api()))
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
