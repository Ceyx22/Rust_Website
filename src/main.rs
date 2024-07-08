mod pages;
mod backend;

use actix_web::{middleware, App, HttpServer, Responder};
use middleware::Logger;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();
    let address = std::env::var("ADDRESS").unwrap_or("127.0.0.1".into());
    let port: u16 = std::env::var("PORT").unwrap_or("0207".into()).parse().unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(backend::config)
    })
    .bind((address, port))?
    .run()
    .await
}
