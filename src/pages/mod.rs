mod home;
mod projects;
mod about;

use std::sync::Mutex;
use actix_web::Responder;
use lazy_static::lazy_static;
use tera::{Tera, Context};
use log::error;

lazy_static! {
    static ref TEMPLATES: Tera = {
        return match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                error!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
    };
}

pub enum Page {
    Home,
    Project,
    About
}

struct Pages{
    home: String,
    project: String,
    about: String,
}

lazy_static! {
    static ref PAGES: Mutex<Pages> = Mutex::new(Pages{
        home: TEMPLATES.render("home.html", &Context::new()).unwrap(),
        project: TEMPLATES.render("projects.html", &projects::get_context()).unwrap(),
        about: TEMPLATES.render("about.html", &Context::new()).unwrap(),
    });
}

pub fn get_page(page: Page) -> String{
    return match page {
        Page::Home => PAGES.lock().unwrap().home.clone(),
        Page::Project => PAGES.lock().unwrap().project.clone(),
        Page::About => PAGES.lock().unwrap().about.clone(),
    }
}

pub async fn refresh() {
    let ctx = projects::get_context();
    let mut pages = PAGES.lock().unwrap();
    pages.project = TEMPLATES.render("projects.html", &ctx).unwrap();
}
