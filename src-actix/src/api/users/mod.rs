use actix_web::{web};

mod _get;
mod _post;


pub fn mount(app: &mut web::ServiceConfig) {
    _get::mount(app);
    _post::mount(app);
}
