use serde::{Deserialize, Serialize};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use tera::Context;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize, Debug)]
struct Project{
    title: String,
    description: String,
    technologies: Vec<String>,
    featured: bool,
    image: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JSON{
    data: Vec<Project>,
}

fn update_page() -> Vec<Project>{
    let data = ureq::get("https://raw.githubusercontent.com/Ceyx22/Rust_Website/refs/heads/master/project-files/projects.json").call().unwrap().into_string().expect("Unable to turn into string");
    let json: JSON = serde_json::from_str(&data).unwrap();
    return json.data
}

pub fn get_context() -> Context{
    let mut ctx = Context::new();
    ctx.insert("projects", &update_page());
    return ctx;
}
pub fn get_featured_ctx() -> Context{
    let featured: Vec<_> = update_page()
        .into_iter()
        .filter(|project| project.featured)
        .collect();
    let mut ctx = Context::new();
    ctx.insert("featured_projects", &featured);
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