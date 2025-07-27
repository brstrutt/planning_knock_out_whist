use actix_web::{Responder, Result, get, web};
use serde::{Deserialize, Serialize};

// GET
#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
}

// LIST
#[get("/users")]
pub async fn list() -> Result<impl Responder> {
    Ok(web::Json(Vec::<User>::new()))
}

// CREATE
// UPDATE
// DELETE

#[cfg(test)]
mod tests {
    use crate::state::AppState;
    use actix_web::{App, test, web};
    use std::sync::Mutex;

    #[actix_web::test]
    async fn list() {
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
}
