use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize};

use crate::state::{AppState, Session};

#[derive(Deserialize)]
pub struct ConnectData {
    session_uuid: String,
}

const MAX_SESSIONS: usize = 20;

#[post("/connect")]
pub async fn post_connect(req_body: web::Json<ConnectData>, data: web::Data<AppState>) -> impl Responder {
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
