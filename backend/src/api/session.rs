use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::state::{AppState, Session};

#[derive(Deserialize)]
struct ConnectRequest {
    session_uuid: String,
}

#[derive(Serialize)]
enum CreateResponseType {
    SessionRestored,
    SessionCreated,
    TooManySessions
}

#[derive(Serialize)]
struct CreateResponse {
    session_status: CreateResponseType
}

const MAX_SESSIONS: usize = 20;

#[post("/connect")]
pub async fn create_session(req_body: web::Json<ConnectRequest>, data: web::Data<AppState>) -> web::Json<CreateResponse> {
    let session_uuid = req_body.session_uuid.clone();
    let mut current_sessions = data.sessions.lock().unwrap();

    let session_already_active = current_sessions.iter().any(|session| session.uuid == session_uuid);
    if session_already_active {
        return web::Json(CreateResponse {
            session_status: CreateResponseType::SessionRestored,
        })
    }

    if current_sessions.len() >= MAX_SESSIONS {
        return web::Json(CreateResponse {
            session_status: CreateResponseType::TooManySessions,
        })
    }

    current_sessions.push(Session{uuid: session_uuid, name: None});
    web::Json(CreateResponse {
        session_status: CreateResponseType::SessionCreated,
    })
}


#[derive(Deserialize)]
struct SetNameRequest {
    session_uuid: String,
    name: String,
}

#[post("/set_name")]
pub async fn set_name(req_body: web::Json<SetNameRequest>, data: web::Data<AppState>) -> impl Responder {
    let session_uuid = req_body.session_uuid.clone();
    let mut current_sessions = data.sessions.lock().unwrap();

    if let Some(session) = current_sessions.iter_mut().find(|session| session.uuid == session_uuid) {
        *session = Session{uuid: session_uuid, name: Some(req_body.name.clone())};
    }

    HttpResponse::Ok()
}


#[derive(Serialize)]
struct GetResponse {
    uuid: String,
    name: String,
}

#[derive(Serialize)]
struct ListResponse {
    sessions: Vec<GetResponse>
}

#[get("/sessions")]
pub async fn list_sessions(data: web::Data<AppState>) -> web::Json<ListResponse> {
    let current_sessions = data.sessions.lock().unwrap();
    let sessions = current_sessions
        .iter()
        .filter(|session| session.name.is_some())
        .map(|session| GetResponse {uuid: session.uuid.clone(), name: session.name.clone().unwrap()}).collect();

    web::Json(ListResponse {
        sessions,
    })
}
