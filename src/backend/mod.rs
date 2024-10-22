use actix_web::{ web};
use actix_files::{Files};
mod publishers;

pub fn config(config: &mut web::ServiceConfig){
    config
        .service(web::scope("")
            .service(Files::new("/static", "./static"))
            .service(publishers::update_projects)
            .service(publishers::get_home)
            .service(publishers::get_projects)
            .service(publishers::get_about)

        );
}
