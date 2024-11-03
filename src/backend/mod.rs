mod publishers;
mod contact;
use actix_web::{web};
use actix_files::{Files};

pub fn config(config: &mut web::ServiceConfig){
    config.service(web::scope("")
        .service(Files::new("/static", "./static"))
        .service(publishers::update_projects)
        .service(contact::submit_contact)
        .service(publishers::get_home)
        .service(publishers::get_projects)
        .service(publishers::get_about)
        .service(publishers::get_latex)
        .service(contact::get_contact));
}
