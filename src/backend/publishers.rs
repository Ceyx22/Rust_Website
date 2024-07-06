use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::pages;

#[get("/welcome")]
pub async fn temp() -> impl Responder {
    HttpResponse::Ok().body(pages::hello_template())
}

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("TEST!")
}
