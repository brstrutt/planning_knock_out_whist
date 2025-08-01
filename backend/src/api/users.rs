use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};

use crate::state::{AppState, Session};

// GET
#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
}

impl User {
    pub fn from_session(session: Session) -> Self {
        User {
            id: session.id.to_string(),
            name: session
                .name
                .unwrap_or(format!("Unknown User {}", session.id)),
        }
    }
}

#[get("/users/{id}")]
pub async fn get(path: web::Path<(u32,)>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner().0;
    let sessions = data.sessions.lock().unwrap();
    let session = sessions.iter().find(|user| user.id == id);

    match session {
        None => HttpResponse::NotFound().body("User not found with the specified ID"),
        Some(session) => HttpResponse::Ok().json(User::from_session(session.clone())),
    }
}

// LIST
#[get("/users")]
pub async fn list(data: web::Data<AppState>) -> impl Responder {
    let current_sessions = data.sessions.lock().unwrap();
    let users: Vec<User> = current_sessions
        .iter()
        .map(|session| User::from_session(session.clone()))
        .collect();

    web::Json(users)
}

// CREATE
#[derive(Deserialize)]
struct NewUser {
    uuid: String,
    name: Option<String>,
}
#[post("/users")]
pub async fn create(req_body: web::Json<NewUser>, data: web::Data<AppState>) -> impl Responder {
    let mut current_sessions = data.sessions.lock().unwrap();

    let existing_session = current_sessions
        .iter_mut()
        .find(|session| session.uuid == req_body.uuid);

    if existing_session.is_some() {
        let existing_session = existing_session.unwrap();

        if req_body.name.is_some() {
            existing_session.name = req_body.name.clone();
        }

        return web::Json(User::from_session(existing_session.clone()));
    } else {
        let mut next_user_id = data.next_user_id.lock().unwrap();

        let new_session = Session {
            uuid: req_body.uuid.clone(),
            id: *next_user_id,
            name: req_body.name.clone(),
        };

        current_sessions.push(new_session.clone());
        *next_user_id = *next_user_id + 1;

        return web::Json(User::from_session(new_session));
    }
}

// UPDATE
#[derive(Deserialize)]
struct ModifiedUser {
    uuid: String,
    name: String,
}
#[post("/users/{id}")]
pub async fn update(
    path: web::Path<(u32,)>,
    req_body: web::Json<ModifiedUser>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner().0;
    print!("id: {}", id);

    let mut current_sessions = data.sessions.lock().unwrap();
    let session = current_sessions.iter_mut().find(|session| session.id == id);

    if session.is_none() {
        return HttpResponse::NotFound().body("User not found with the specified ID");
    }
    let session = session.unwrap();

    if session.uuid != req_body.uuid {
        return HttpResponse::Forbidden().body("You do not have permission to modify this user");
    }

    session.name = Some(req_body.name.clone());
    HttpResponse::Ok().json(User::from_session(session.clone()))
}

#[cfg(test)]
mod tests {
    use crate::state::{AppState, Session};
    use actix_web::http::{StatusCode, header::ContentType};
    use actix_web::{App, test, web};

    use super::*;

    mod list {
        use super::*;

        #[actix_web::test]
        async fn list_empty() {
            // Setup actix
            let app_data = web::Data::new(AppState::default());
            let app =
                test::init_service(App::new().app_data(app_data.clone()).service(super::list))
                    .await;

            // Call the API
            let req = test::TestRequest::get().uri("/users").to_request();
            let resp: Vec<super::User> = test::call_and_read_body_json(&app, req).await;

            // Check the response
            assert_eq!(resp.len(), 0);
        }

        #[actix_web::test]
        async fn list() {
            // Setup actix
            let app_data = web::Data::new(AppState::default());
            app_data.sessions.lock().unwrap().push(Session {
                uuid: String::from("1"),
                id: 1,
                name: Some(String::from("testing")),
            });
            app_data.sessions.lock().unwrap().push(Session {
                uuid: String::from("several"),
                id: 2,
                name: None,
            });
            app_data.sessions.lock().unwrap().push(Session {
                uuid: String::from("Third"),
                id: 3,
                name: Some(String::from("manymanymanyword long name here")),
            });

            let app =
                test::init_service(App::new().app_data(app_data.clone()).service(super::list))
                    .await;

            // Call the API
            let req = test::TestRequest::get().uri("/users").to_request();
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
        use super::*;

        #[actix_web::test]
        async fn get_missing_user() {
            // Setup actix
            let app_data = web::Data::new(AppState::default());
            app_data.sessions.lock().unwrap().push(Session {
                uuid: String::from("123-456-789"),
                id: 97, // ASCII letter `a`
                name: Some(String::from("A very real name")),
            });
            let app =
                test::init_service(App::new().app_data(app_data.clone()).service(super::get)).await;

            // Test invalid ID returns 404
            let req = test::TestRequest::get().uri("/users/a").to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);

            // Test missing ID returns 404
            let req = test::TestRequest::get().uri("/users/1337").to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        }

        #[actix_web::test]
        async fn get_user() {
            // Setup actix
            let app_data = web::Data::new(AppState::default());
            app_data.sessions.lock().unwrap().push(Session {
                uuid: String::from("123-456-789"),
                id: 254,
                name: Some(String::from("A very real name")),
            });

            let app =
                test::init_service(App::new().app_data(app_data.clone()).service(super::get)).await;

            // Test the user gets returned
            let req = test::TestRequest::get().uri("/users/254").to_request();
            let resp: super::User = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.id, "254");
            assert_eq!(resp.name, "A very real name");
        }
    }

    mod create {
        use super::*;

        #[actix_web::test]
        async fn create_user_incorrectly() {
            // Setup actix
            let app_data = web::Data::new(AppState::default());
            let app =
                test::init_service(App::new().app_data(app_data.clone()).service(super::create))
                    .await;

            // Test missing body returns bad_request
            let req = test::TestRequest::post().uri("/users").to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

            // Test missing UUID in body returns bad_request
            let req = test::TestRequest::post()
                .uri("/users")
                .set_payload(r#"{"name": "Pinnochio"}"#)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        }

        #[actix_web::test]
        async fn create_user() {
            // Setup actix
            let app_data = web::Data::new(AppState::default());
            let app =
                test::init_service(App::new().app_data(app_data.clone()).service(super::create))
                    .await;

            // Test providing UUID creates user successfully with default name
            let req = test::TestRequest::post()
                .uri("/users")
                .insert_header(ContentType::json())
                .set_payload(
                    r#"{
                        "uuid": "my-definitely-unique-id"
                    }"#,
                )
                .to_request();
            let resp: super::User = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.id, "1");
            assert_eq!(resp.name, "Unknown User 1");

            // Test providing UUID AND name creates user successfully with specified name
            let req = test::TestRequest::post()
                .uri("/users")
                .insert_header(ContentType::json())
                .set_payload(
                    r#"{
                        "uuid": "a-second-definitely-unique-id",
                        "name": "Pinnochio"
                    }"#,
                )
                .to_request();
            let resp: super::User = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.id, "2");
            assert_eq!(resp.name, "Pinnochio");
        }

        #[actix_web::test]
        async fn create_user_with_already_in_use_uuid() {
            // Setup actix
            let app_data = web::Data::new(AppState::default());
            app_data.sessions.lock().unwrap().push(Session {
                uuid: String::from("i-am-not-unique"),
                id: 123,
                name: Some(String::from("Benjo the Banjo")),
            });
            let app =
                test::init_service(App::new().app_data(app_data.clone()).service(super::create))
                    .await;

            // Test reusing UUID simply returns the already existing User
            let req = test::TestRequest::post()
                .uri("/users")
                .insert_header(ContentType::json())
                .set_payload(
                    r#"{
                        "uuid": "i-am-not-unique"
                    }"#,
                )
                .to_request();
            let resp: super::User = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.id, "123");
            assert_eq!(resp.name, "Benjo the Banjo");
            assert_eq!(app_data.sessions.lock().unwrap().len(), 1);

            // Test reusing ID and providing name returns the already existing user with the new name
            let req = test::TestRequest::post()
                .uri("/users")
                .insert_header(ContentType::json())
                .set_payload(
                    r#"{
                        "uuid": "i-am-not-unique",
                        "name": "Pinnochio"
                    }"#,
                )
                .to_request();
            let resp: super::User = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.id, "123");
            assert_eq!(resp.name, "Pinnochio");
            assert_eq!(app_data.sessions.lock().unwrap().len(), 1);
        }

        // TODO: Test the ID sequence generates correctly
        // TODO: Test the function returns 201 not 200 on success
        // TODO: Test the ID sequence works even if there are gaps in the sequence
        // TODO: Test the ID sequence works even if the last few entries have been removed (eg if Id 5 is added, then removed, the next addition should be Id 6 not Id 5 again)
    }

    mod update {
        use super::*;

        #[actix_web::test]
        async fn update_user_incorrectly() {
            // Setup actix
            let app_data = web::Data::new(AppState::default());
            app_data.sessions.lock().unwrap().push(Session {
                uuid: String::from("testing-uuid-123"),
                id: 123,
                name: Some(String::from("Schmebulock the THIRD")),
            });
            let app =
                test::init_service(App::new().app_data(app_data.clone()).service(super::update))
                    .await;

            // Test missing body returns bad_request
            let req = test::TestRequest::post().uri("/users/123").to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

            // Test a missing UUID returns a bad_request error
            let req = test::TestRequest::post()
                .uri("/users/123")
                .insert_header(ContentType::json())
                .set_payload(r#"{"name": "Pinnochio"}"#)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

            // Test a UUID that doesn't match the modified user returns a forbidden error
            let req = test::TestRequest::post()
                .uri("/users/123")
                .insert_header(ContentType::json())
                .set_payload(r#"{"uuid": "im-the-wrong-uuid", "name": "Pinnochio"}"#)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::FORBIDDEN);

            // Test modifying a non-existant user returns 404
            let req = test::TestRequest::post()
                .uri("/users/879")
                .insert_header(ContentType::json())
                .set_payload(r#"{"uuid": "im-the-wrong-uuid", "name": "Pinnochio"}"#)
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        }

        #[actix_web::test]
        async fn update_user() {
            // Setup actix
            let app_data = web::Data::new(AppState::default());
            app_data.sessions.lock().unwrap().push(Session {
                uuid: String::from("testing-uuid-123"),
                id: 123,
                name: Some(String::from("Schmebulock the THIRD")),
            });
            let app =
                test::init_service(App::new().app_data(app_data.clone()).service(super::update))
                    .await;

            // Test modification of Name works fine
            let req = test::TestRequest::post()
                .uri("/users/123")
                .insert_header(ContentType::json())
                .set_payload(r#"{"uuid": "testing-uuid-123", "name": "Pinnochio"}"#)
                .to_request();
            let resp: super::User = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.id, "123");
            assert_eq!(resp.name, "Pinnochio");

            // Test modification of ID gets ignored
            let req = test::TestRequest::post()
                .uri("/users/123")
                .insert_header(ContentType::json())
                .set_payload(
                    r#"{"uuid": "testing-uuid-123", "id": "978", "name": "Gerrymander Himself"}"#,
                )
                .to_request();
            let resp: super::User = test::call_and_read_body_json(&app, req).await;
            assert_eq!(resp.id, "123");
            assert_eq!(resp.name, "Gerrymander Himself");
        }
    }
}
