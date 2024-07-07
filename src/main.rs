mod pages;
mod backend;
mod utils;

use actix_web::{middleware, App, HttpServer, Responder};


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv::dotenv().ok();
    env_logger::init();

    let port = (*utils::constants::PORT).clone();
    let address = (*utils::constants::ADDRESS).clone();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(backend::app_config::config)
    })
    .bind((address, port))?
    .run()
    .await
}
