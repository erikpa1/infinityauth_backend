
use actix_web::{web};


#[macro_export]
macro_rules! STATUS_OK {
    () => {
        web::Json(json!({
        "status": true
    }))
    };
}