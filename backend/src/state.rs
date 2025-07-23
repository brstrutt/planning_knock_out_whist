use std::sync::Mutex;


pub struct Session {
    pub uuid: String
}

pub struct AppState {
    pub message: Mutex<String>,
    pub sessions: Mutex<Vec<Session>>
}