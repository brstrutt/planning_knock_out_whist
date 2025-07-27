use actix_web::{HttpResponse, Responder, get, post, put, web};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

// GET
#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
}

#[get("/users/{id}")]
pub async fn get() -> impl Responder {
    HttpResponse::NotImplemented()
}

// LIST
#[get("/users")]
pub async fn list(data: web::Data<AppState>) -> impl Responder {
    let current_sessions = data.sessions.lock().unwrap();
    let users: Vec<User> = current_sessions
        .iter()
        .map(|session| User {
            id: session.id.clone().to_string(),
            name: session
                .name
                .clone()
                .unwrap_or(format!("Unknown User {}", session.id.clone())),
        })
        .collect();

    web::Json(users)
}

// CREATE
#[post("/users")]
pub async fn create() -> impl Responder {
    HttpResponse::NotImplemented()
}

// UPDATE
#[put("/users/{id}")]
pub async fn update() -> impl Responder {
    HttpResponse::NotImplemented()
}

#[cfg(test)]
mod tests {
    use crate::state::{AppState, Session};
    use actix_web::{App, test, web};
    use std::sync::Mutex;

    use super::*;

    mod list {
        use super::*;

        #[actix_web::test]
        async fn list_empty() {
            // Setup actix
            let app_data = web::Data::new(AppState {
                message: Mutex::new(String::from("")),
                sessions: Mutex::new(Vec::new()),
            });
            let app =
                test::init_service(App::new().app_data(app_data.clone()).service(super::list))
                    .await;
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
                        id: 1,
                        name: Some(String::from("testing")),
                    },
                    Session {
                        uuid: String::from("several"),
                        id: 2,
                        name: None,
                    },
                    Session {
                        uuid: String::from("Third"),
                        id: 3,
                        name: Some(String::from("manymanymanyword long name here")),
                    },
                ]),
            });
            let app =
                test::init_service(App::new().app_data(app_data.clone()).service(super::list))
                    .await;
            let req = test::TestRequest::get().uri("/users").to_request();

            // // Call the API
            let resp: Vec<super::User> = test::call_and_read_body_json(&app, req).await;

            // Check the response
            assert_eq!(resp.len(), 3);
            assert_eq!(resp[0].id, "1");
            assert_eq!(resp[0].name, "testing");
            assert_eq!(resp[1].id, "2");
            assert_eq!(resp[1].name, "Unknown User 2");
            assert_eq!(resp[2].id, "3");
            assert_eq!(resp[2].name, "manymanymanyword long name here");
        }
    }

    mod get {
        // Test a valid user ID produces the correct user data
        // Test a non-existant user ID produces a 404 error
        // Test an invalid user Id produces a 404 error
    }

    mod create {
        // Test a missing UUID returns a bad_request error
        // Test a valid UUID returns the expected new user and that user is persisted
    }

    mod update {
        // Test a missing UUID returns a bad_request error
        // Test a UUID that doesn't match the modified user returns a forbidden error
        // Test modifying a non-existant user returns 404
        // Test modification of ID throws bad_request
        // Test modification of Name works fine
    }
}
