use actix_web::{Responder, web};
mod publishers;

pub fn config(config: &mut web::ServiceConfig){
    config
        .service(web::scope("")
            .service(publishers::update_projects)
            .service(publishers::get_home)
            .service(publishers::get_projects)
        );
}
