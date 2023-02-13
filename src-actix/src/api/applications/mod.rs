use actix_web::{web};

mod _get;


pub fn mount(app: &mut web::ServiceConfig) {
    _get::mount(app);
}
