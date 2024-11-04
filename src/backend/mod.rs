mod contact;
mod publishers;
use actix_files::Files;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(Files::new("/static", "./static"))
            .service(publishers::update_projects)
            .service(publishers::get_home)
            .service(publishers::get_gallery)
            .service(publishers::get_about)
            .service(publishers::get_latex)
            .service(contact::get_contact)
            .service(contact::submit_contact)
            .service(publishers::get_project_page)

    );
}
