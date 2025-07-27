use actix_web::{HttpResponse, Responder, get};

// GET
// LIST
#[get("/users")]
pub async fn list() -> impl Responder {
    HttpResponse::Ok()
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
        let resp = test::call_service(&app, req).await;

        // Check the response
        print!("Status code: {}", resp.status());
        assert!(resp.status().is_success());
    }
}
