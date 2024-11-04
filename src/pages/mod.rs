mod projects;
mod env_context;

use lazy_static::lazy_static;
use log::error;
use std::sync::Mutex;
use tera::{Context, Tera};
use crate::pages::projects::get_project_ctx;

pub enum Page {
    Home,
    Gallery,
    About,
    Latex,
    Contact,
    DynamicProject(String),
}

struct Pages {
    home: String,
    gallery: String,
    about: String,
    latex: String,
    contact: String,
    // project: String,
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
    static ref PAGES: Mutex<Pages> = Mutex::new(Pages {
        home: TEMPLATES
            .render("home.html", &projects::get_context(true))
            .unwrap(),
        gallery: TEMPLATES
            .render("projects.html", &projects::get_context(false))
            .unwrap(),
        about: TEMPLATES
            .render("about.html", &env_context::get_social_links())
            .unwrap(),
        latex: TEMPLATES.render("latex.html", &Context::new()).unwrap(),
        contact: TEMPLATES.render("contact.html", &Context::new()).unwrap(),
    });
}

pub fn get_page(page: Page) -> String {
    return match page {
        Page::Home => PAGES.lock().unwrap().home.clone(),
        Page::Gallery => PAGES.lock().unwrap().gallery.clone(),
        Page::About => PAGES.lock().unwrap().about.clone(),
        Page::Latex => PAGES.lock().unwrap().latex.clone(),
        Page::Contact => PAGES.lock().unwrap().contact.clone(),
        Page::DynamicProject(slug) => render_project(&slug)
    };
}

fn render_project(slug: &str) -> String{
    TEMPLATES.render("project_base.html", &get_project_ctx(slug)).unwrap()
}

pub async fn refresh() {
    let ctx:Context = projects::get_context(false);
    let featured_ctx:Context = projects::get_context(true);
    let mut pages = PAGES.lock().unwrap();
    pages.gallery = TEMPLATES.render("projects.html", &ctx).unwrap();
    pages.home = TEMPLATES.render("home.html", &featured_ctx).unwrap();
}

pub fn verify(msg: &str, key: &str) -> Result<(), String> {
    return projects::verify(msg, key);
}
