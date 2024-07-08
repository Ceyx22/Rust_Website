use actix_web::{get, HttpResponse, post, Responder};
use crate::pages;


// Get
#[get("/")]
pub async fn get_home() -> impl Responder{
    return HttpResponse::Ok().body(pages::get_page(pages::Page::Home));
}

#[get("/projects")]
pub async fn get_projects() -> impl Responder{
    return HttpResponse::Ok().body(pages::get_page(pages::Page::Project));
}

#[get("/about")]
pub async fn get_about() -> impl Responder{
    return HttpResponse::Ok().body(pages::get_page(pages::Page::About));
}

// Post
#[post("/projects")]
pub async fn update_projects() -> impl Responder{
    pages::refresh().await;
    return HttpResponse::Ok().body("UPDATED....");
}