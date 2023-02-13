use actix_multipart::Multipart;
use actix_web::{web, post, Responder};
use actix_web::web::Form;
use serde::Deserialize;

use serde_json::{json, Value};

use actix_easy_multipart::text::Text;
use actix_easy_multipart::MultipartForm;

use azure_tools::myazure::tables;

#[derive(MultipartForm)]
struct LoginCredentials {
    user_name: Text<String>,
    password: Text<String>,

}


#[post("/api/users/try-login")]
// pub async fn _try_login_user(mut payload: Multipart) -> impl Responder {
pub async fn _try_login_user(login_data: MultipartForm<LoginCredentials>) -> impl Responder {
    println!("User name: {:?}", login_data.user_name);
    println!("User password: {:?}", login_data.password);

    if let Ok(result) = tables::get_entity("testusers", "root", &login_data.user_name).await {
        println!("{:?}", result);

        return web::Json(json!({
        "is_valid": false,
        "user_role": "admin",
        "user_data": json!({
            "user_name": "Erik",
            "user_surname": "Palencik",
        })
    }));
    } else {
        // return web::Json(json!({ "projects": json!([]) }));
    }

    return web::Json(json!({
        "is_valid": false,
    }));
}

pub fn mount(app: &mut web::ServiceConfig) {
    app.service(_try_login_user);
}