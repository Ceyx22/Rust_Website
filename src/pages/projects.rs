use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Serialize, Deserialize, Debug)]
struct Project{
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JSON{
    data: Vec<Project>,
}

fn update_page() -> Vec<Project>{
    let data = ureq::get("https://github.com/Ceyx22/Rust_Website/tree/master/project-files").call().unwrap().into_string().expect("Unable to turn into string");
    let json: JSON = serde_json::from_str(&data).unwrap();
    return json.data
}

pub fn get_context() -> Context{
    let mut ctx = Context::new();
    ctx.insert("projects", &update_page());
    return ctx;
}