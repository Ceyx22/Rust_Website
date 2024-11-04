mod backend;
mod pages;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let address: String = std::env::var("ADDRESS").unwrap_or("127.0.0.1".into());
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8000".into())
        .parse()
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(backend::config)
    })
    .bind((address, port))?
    .run()
    .await
}
