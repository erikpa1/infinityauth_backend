mod users;
mod applications;
mod organizations;
mod licences;


use actix_web::{web};

pub fn mount(app: &mut web::ServiceConfig) {
    users::mount(app);
    applications::mount(app);
    organizations::mount(app);
    licences::mount(app);
}

