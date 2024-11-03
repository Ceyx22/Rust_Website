mod projects;

use std::sync::Mutex;
use lazy_static::lazy_static;
use tera::{Tera, Context};
use log::error;

pub enum Page {
    Home,
    Project,
    About,
    Latex,
    Contact,
}

struct Pages{
    home: String,
    project: String,
    about: String,
    latex: String,
    contact: String,
}

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

lazy_static! {
    static ref PAGES: Mutex<Pages> = Mutex::new(Pages{
        home: TEMPLATES.render("home.html", &projects::get_featured_ctx()).unwrap(),
        project: TEMPLATES.render("projects.html", &projects::get_context()).unwrap(),
        about: TEMPLATES.render("about.html", &Context::new()).unwrap(),
        latex: TEMPLATES.render("latex.html", &Context::new()).unwrap(),
        contact: TEMPLATES.render("contact.html", &Context::new()).unwrap(),
    });
}

pub fn get_page(page: Page) -> String{
    return match page {
        Page::Home => PAGES.lock().unwrap().home.clone(),
        Page::Project => PAGES.lock().unwrap().project.clone(),
        Page::About => PAGES.lock().unwrap().about.clone(),
        Page::Latex => PAGES.lock().unwrap().latex.clone(),
        Page::Contact => PAGES.lock().unwrap().contact.clone(),
    }
}

pub async fn refresh() {
    let ctx = projects::get_context();
    let featured_ctx = projects::get_featured_ctx();
    let mut pages = PAGES.lock().unwrap();
    pages.project = TEMPLATES.render("projects.html", &ctx).unwrap();
    pages.home = TEMPLATES.render("home.html", &featured_ctx).unwrap();
}

pub fn verify(msg:&str, key:&str) -> Result<(), String> {
    return projects::verify(msg, key);
}