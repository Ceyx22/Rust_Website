use actix_web::{get, HttpResponse, post, Responder};
use crate::pages;

#[get("/")]
pub async fn get_home() -> impl Responder{
    HttpResponse::Ok().body(pages::get_page(pages::Page::Home))
}

#[get("/projects")]
pub async fn get_projects() -> impl Responder{
    HttpResponse::Ok().body(pages::get_page(pages::Page::Project))
}

#[get("/about")]
pub async fn get_about() -> impl Responder{
    HttpResponse::Ok().body(pages::get_page(pages::Page::About))
}

#[post("/projects")]
pub async fn update_projects() -> impl Responder{
    pages::refresh().await;
    HttpResponse::Ok().body("UPDATED....")
}