use actix_easy_multipart::MultipartForm;
use actix_web::{web, post, Responder};

use azure_tools::myazure::tables::insert_entity;


use uuid::Uuid;

use actix_easy_multipart::text::Text;

use serde_json::{json, Value};

use azure_tools::{CONST_RK, CONST_PK};
use crate::STATUS_OK;


#[derive(MultipartForm)]
struct CreateOrganizationParams {
    organization_name: Text<String>,
    image_path: Text<String>,
}

#[post("/api/organizations/add-organizations")]
async fn _add_ogranization(form: MultipartForm<CreateOrganizationParams>) -> impl Responder {
    // let entities = get_entities("organizations").await;

    let name = &form.organization_name.as_str();
    let image_path = &form.image_path.as_str();

    let value = json!({
        CONST_PK!(): "organization",
        CONST_RK!(): Uuid::new_v4().to_string(),
        "Name": name,
        "ImagePath": image_path
    });

    insert_entity("organizations", "organization", &value);

    return STATUS_OK!();
}

pub fn mount(app: &mut web::ServiceConfig) {
    app.service(_add_ogranization);
}