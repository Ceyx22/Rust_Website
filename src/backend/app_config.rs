use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::backend;
pub fn config(config: &mut web::ServiceConfig){
    config
        .service(web::scope("/home")
        .service(backend::publishers::temp)
        .service(backend::publishers::test));
}