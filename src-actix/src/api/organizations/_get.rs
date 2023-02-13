use actix_web::{web, get, Responder};

use azure_tools::myazure::tables::get_entities;

use serde_json::{json, Value};

use crate::constants::tables::{OrganizationTable};

#[get("/api/organizations/get-organizations")]
async fn _get_organizations() -> impl Responder {
    let entities = get_entities("organizations").await;

    if let Ok(result) = entities {
        let convertedVector: Vec<Value> = result.iter().map(|value| {
            let uid = value.get(OrganizationTable::RK());
            let name = value.get(OrganizationTable::NAME());
            let image_path = value.get(OrganizationTable::IMAGE_PATH());

            return json!({
                "uid": uid,
                "name": name,
                "image_path": image_path,
            });
        }).collect();

        return web::Json(json!({
        "values": json!(convertedVector)
    }));
    } else {
        return web::Json(json!({
        "values": []
    }));
    }
}

pub fn mount(app: &mut web::ServiceConfig) {
    app.service(_get_organizations);
}