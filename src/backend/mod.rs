use actix_web::{get, HttpResponse, Responder};


pub mod publishers;
pub mod app_config;


// pub async fn temp() -> impl Responder {
//     publishers::temp
// }
// pub async fn test() -> impl Responder {
//     publishers::test
// }

// // this function could be located in a different module
// pub fn scoped_config() {
//     cfg.service(
//         web::resource("/test")
//             .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
//             .route(web::head().to(HttpResponse::MethodNotAllowed)),
//     );
// }