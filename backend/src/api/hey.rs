use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Serialize, Deserialize)]
struct Message {
    text: String,
}

#[get("/hey")]
pub async fn get_hey(data: web::Data<AppState>) -> web::Json<Message> {
    web::Json(Message {
        text: data.message.lock().unwrap().clone(),
    })
}

#[post("/hey")]
pub async fn post_hey(req_body: web::Json<Message>, data: web::Data<AppState>) -> impl Responder {
    let new_message = req_body.text.clone();
    let mut current_message = data.message.lock().unwrap();
    *current_message = new_message;
    HttpResponse::Ok()
}
