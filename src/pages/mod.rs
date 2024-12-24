mod env_context;
mod projects;

use crate::pages::projects::get_project_ctx;
use lazy_static::lazy_static;
use log::error;
use std::sync::Mutex;
use tera::{Context, Tera};

pub enum Page {
    Home,
    Gallery,
    About,
    Latex,
    Confirmation,
    DynamicProject(String),
}

struct Pages {
    home: String,
    gallery: String,
    about: String,
    latex: String,
    confirmation: String,
}

lazy_static! {
    static ref TEMPLATES: Tera = {
        match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                error!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

lazy_static! {
    static ref PAGES: Mutex<Pages> = Mutex::new(Pages {
        home: TEMPLATES
            .render("home.html", &projects::get_context(true))
            .unwrap(),
        gallery: TEMPLATES
            .render("gallery.html", &projects::get_context(false))
            .unwrap(),
        about: TEMPLATES
            .render("about.html", &env_context::get_social_links())
            .unwrap(),
        latex: TEMPLATES.render("latex.html", &Context::new()).unwrap(),
        confirmation: TEMPLATES
            .render("confirmation.html", &Context::new())
            .unwrap(),
    });
}

pub fn get_page(page: Page) -> String {
    return match page {
        Page::Home => PAGES.lock().unwrap().home.clone(),
        Page::Gallery => PAGES.lock().unwrap().gallery.clone(),
        Page::About => PAGES.lock().unwrap().about.clone(),
        Page::Latex => PAGES.lock().unwrap().latex.clone(),
        Page::Confirmation => PAGES.lock().unwrap().confirmation.clone(),
        Page::DynamicProject(slug) => render_project(&slug),
    };
}

fn render_project(slug: &str) -> String {
    TEMPLATES
        .render("individual_project_base.html", &get_project_ctx(slug))
        .unwrap()
}

pub async fn refresh() {
    let ctx: Context = projects::get_context(false);
    let featured_ctx: Context = projects::get_context(true);
    let mut pages = PAGES.lock().unwrap();
    pages.gallery = TEMPLATES.render("gallery.html", &ctx).unwrap();
    pages.home = TEMPLATES.render("home.html", &featured_ctx).unwrap();
}

pub fn verify(msg: &str, key: &str) -> Result<(), String> {
    return projects::verify(msg, key);
}
