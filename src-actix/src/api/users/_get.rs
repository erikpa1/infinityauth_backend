use actix_web::{web, get, Responder};

use serde_json::{json};

#[get("/api/users/get")]
pub async fn _get_test_user() -> impl Responder {
    web::Json(json!({
        "myjson": "value"
    }))
}

pub fn mount(app: &mut web::ServiceConfig) {
    app.service(_get_test_user);

}