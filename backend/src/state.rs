use std::sync::Mutex;

#[derive(Clone)]
pub struct Session {
    pub uuid: String,
    pub id: u32,
    pub name: Option<String>,
}

pub struct AppState {
    pub message: Mutex<String>,
    pub sessions: Mutex<Vec<Session>>,
    pub next_user_id: Mutex<u32>,
}

impl AppState {
    pub fn default() -> Self {
        AppState {
            message: Mutex::new(String::from("no message")),
            sessions: Mutex::new(Vec::new()),
            next_user_id: Mutex::new(1),
        }
    }
}
