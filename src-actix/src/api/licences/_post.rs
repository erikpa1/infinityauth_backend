use actix_easy_multipart::MultipartForm;
use actix_web::{web, post, Responder};

use azure_tools::myazure::tables::insert_entity;

use uuid::Uuid;

use actix_easy_multipart::text::Text;

use serde_json::{json, Value};
use azure_tools::{CONST_RK, CONST_PK};
use crate::STATUS_OK;


#[derive(MultipartForm)]
struct CreateLicenceForm {
    organization: Text<String>,
    application: Text<String>,
    licence_type: Text<String>,
    valid_to: Text<String>,

}


#[post("/api/organizations/add-organizations")]
async fn _add_licence(form: MultipartForm<CreateLicenceForm>) -> impl Responder {
    // let entities = get_entities("organizations").await;

    let organization_uid = &form.organization.as_str();
    let application_uid = &form.application.as_str();
    let licence_type = &form.licence_type.as_str();
    let valid_to = &form.valid_to.as_str();

    // let value = json!({
    //     CONST_PK!(): "organization",
    //     CONST_RK!(): Uuid::new_v4().to_string(),
    //     "Name": name,
    //     "ImagePath": image_path
    // });
    //
    // insert_entity("organizations", "organization", &value);

    return STATUS_OK!();
}

pub fn mount(app: &mut web::ServiceConfig) {
    app.service(_add_licence);
}