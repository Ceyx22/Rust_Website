use serde::{Deserialize, Serialize};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use tera::Context;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize, Debug)]
struct Project{
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JSON{
    data: Vec<Project>,
}

fn update_page() -> Vec<Project>{
    let data = ureq::get("https://raw.githubusercontent.com/Ceyx22/Rust_Website/master/project-files/projects.json").call().unwrap().into_string().expect("Unable to turn into string");
    // println!("{}", data);
    let json: JSON = serde_json::from_str(&data).unwrap();
    return json.data
}

pub fn get_context() -> Context{
    let mut ctx = Context::new();
    ctx.insert("projects", &update_page());
    return ctx;
}

pub fn verify(msg:&str, key:&str) -> Result<(), String>{
    let private_key = std::env::var("SECRETKEY").unwrap();
    let mut hasher = HmacSha256::new_from_slice(private_key.as_ref()).unwrap();
    hasher.update(msg.as_ref());
    return match hasher.verify_slice(&*hex::decode(key).unwrap()) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}