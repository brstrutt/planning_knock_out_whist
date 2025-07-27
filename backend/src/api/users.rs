use actix_web::{Responder, Result, get, web};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

// GET
#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
}

// LIST
#[get("/users")]
pub async fn list(data: web::Data<AppState>) -> Result<impl Responder> {
    let current_sessions = data.sessions.lock().unwrap();
    let users: Vec<User> = current_sessions
        .iter()
        .map(|session| User {
            id: session.uuid.clone(),
            name: session.name.clone().unwrap_or(session.uuid.clone()),
        })
        .collect();

    Ok(web::Json(users))
}

// CREATE
// UPDATE
// DELETE

#[cfg(test)]
mod tests {
    use crate::state::{AppState, Session};
    use actix_web::{App, test, web};
    use std::sync::Mutex;

    #[actix_web::test]
    async fn list_empty() {
        // Setup actix
        let app_data = web::Data::new(AppState {
            message: Mutex::new(String::from("")),
            sessions: Mutex::new(Vec::new()),
        });
        let app =
            test::init_service(App::new().app_data(app_data.clone()).service(super::list)).await;
        let req = test::TestRequest::get().uri("/users").to_request();

        // // Call the API
        let resp: Vec<super::User> = test::call_and_read_body_json(&app, req).await;

        // Check the response
        assert_eq!(resp.len(), 0);
    }

    #[actix_web::test]
    async fn list() {
        // Setup actix
        let app_data = web::Data::new(AppState {
            message: Mutex::new(String::from("")),
            sessions: Mutex::new(vec![
                Session {
                    uuid: String::from("1"),
                    name: Some(String::from("testing")),
                },
                Session {
                    uuid: String::from("several"),
                    name: None,
                },
                Session {
                    uuid: String::from("Third"),
                    name: Some(String::from("manymanymanyword long name here")),
                },
            ]),
        });
        let app =
            test::init_service(App::new().app_data(app_data.clone()).service(super::list)).await;
        let req = test::TestRequest::get().uri("/users").to_request();

        // // Call the API
        let resp: Vec<super::User> = test::call_and_read_body_json(&app, req).await;

        // Check the response
        assert_eq!(resp.len(), 3);
        assert_eq!(resp[0].id, "1");
        assert_eq!(resp[0].name, "testing");
        assert_eq!(resp[1].id, "several");
        assert_eq!(resp[1].name, "several");
        assert_eq!(resp[2].id, "Third");
        assert_eq!(resp[2].name, "manymanymanyword long name here");
    }
}
