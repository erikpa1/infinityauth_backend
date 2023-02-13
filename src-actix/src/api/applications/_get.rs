use actix_web::{web, get, Responder};

use azure_tools::myazure::tables::get_entities;

use serde_json::{json, Value};

use crate::constants::tables::{ApplicationTable};

#[get("/api/applications/get-applications")]
async fn _get_applications() -> impl Responder {
    let entities = get_entities("applications").await;

    if let Ok(result) = entities {
        let convertedVector: Vec<Value> = result.iter().map(|value| {
            let uid = value.get(ApplicationTable::RK());
            let name = value.get(ApplicationTable::NAME());
            let appType = value.get(ApplicationTable::TYPE());

            return json!({
                "uid": uid,
                "name": name,
                "type": appType
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
    app.service(_get_applications);
}