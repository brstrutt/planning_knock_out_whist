use actix_web::{post, web};
use serde::{Deserialize, Serialize};

use crate::state::{AppState, Session};

#[derive(Deserialize)]
struct ConnectRequest {
    session_uuid: String,
}

#[derive(Serialize)]
enum ResponseType {
    SessionRestored,
    SessionCreated,
    TooManySessions
}

#[derive(Serialize)]
struct ConnectResponse {
    session_status: ResponseType
}

const MAX_SESSIONS: usize = 20;

#[post("/connect")]
pub async fn post_connect(req_body: web::Json<ConnectRequest>, data: web::Data<AppState>) -> web::Json<ConnectResponse> {
    let session_uuid = req_body.session_uuid.clone();
    let mut current_sessions = data.sessions.lock().unwrap();

    let session_already_active = current_sessions.iter().any(|session| session.uuid == session_uuid);
    if session_already_active {
        return web::Json(ConnectResponse {
            session_status: ResponseType::SessionRestored,
        })
    }

    if current_sessions.len() >= MAX_SESSIONS {
        return web::Json(ConnectResponse {
            session_status: ResponseType::TooManySessions,
        })
    }

    current_sessions.push(Session{uuid: session_uuid});
    web::Json(ConnectResponse {
        session_status: ResponseType::SessionCreated,
    })
}
