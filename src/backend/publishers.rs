use crate::pages;
use actix_web::web::Bytes;
use actix_web::{web, get, post, HttpRequest, HttpResponse, Responder};
use log::{info, warn};

#[get("/")]
pub async fn get_home() -> impl Responder {
    return HttpResponse::Ok().body(pages::get_page(pages::Page::Home));
}

#[get("/projects")]
pub async fn get_gallery() -> impl Responder {
    return HttpResponse::Ok().body(pages::get_page(pages::Page::Gallery));
}

#[get("/about")]
pub async fn get_about() -> impl Responder {
    return HttpResponse::Ok().body(pages::get_page(pages::Page::About));
}

#[get("/latex")]
pub async fn get_latex() -> impl Responder {
    return HttpResponse::Ok().body(pages::get_page(pages::Page::Latex));
}

// Post
#[post("/projects")]
pub async fn update_projects(req: HttpRequest, bytes: Bytes) -> impl Responder {
    let incoming = req.headers().get("X-Hub-Signature-256");
    if incoming.is_none() {
        warn!("Unauthorized attempt to update.");
        return HttpResponse::Unauthorized().body("Invalid Signature");
    }
    let str_inc = incoming.unwrap().to_str();
    if str_inc.is_err() {
        warn!("Unauthorized attempt to update.");
        return HttpResponse::Unauthorized().body("Invalid Signature");
    }
    let hash = str_inc.unwrap().strip_prefix("sha256=");
    if hash.is_none() {
        warn!("Unauthorized attempt to update.");
        return HttpResponse::Unauthorized().body("Invalid Signature");
    }

    let res = pages::verify(&*String::from_utf8_lossy(&bytes), hash.unwrap());

    if res.is_err() {
        return HttpResponse::Unauthorized().body(res.err().unwrap());
    }

    info!("Updated Portfolio");
    crate::pages::refresh().await;
    HttpResponse::Ok().body("Updated!")
}

#[get("/projects/{slug}")]
pub async fn get_project_page(slug: web::Path<String>) -> impl Responder {
    let page_content = pages::get_page(pages::Page::DynamicProject(slug.into_inner()));
    HttpResponse::Ok().body(page_content)
}
