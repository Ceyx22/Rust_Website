use std::env;
use actix_web::http::StatusCode;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
}

pub struct MessageStruct{
    response_code: StatusCode,
    body: String,
    status_code: u16
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap()
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT").unwrap().parse::<u16>().unwrap()
}

impl MessageStruct {
    pub fn new(status_code: u16, body:String)-> Self{
        MessageStruct{
            status_code,
            body,
            response_code: StatusCode::from_u16(status_code).unwrap()
        }
    }
}