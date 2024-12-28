use chrono::prelude::*;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tera::Context;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    title: String,
    description: String,
    technologies: Vec<String>,
    featured: bool,
    images: Vec<String>,
    slug: String,
    date: DateTime<Utc>,
    show: bool,
    headers: Vec<String>,
    texts: Vec<String>,
    videos: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JSON {
    data: Vec<Project>,
}

fn update_page() -> Vec<Project> {
    let projects_url: String = std::env::var("PROJECTS_URL").unwrap();
    let data = ureq::get(&projects_url)
        .call()
        .unwrap()
        .into_string()
        .expect("Unable to turn into string");
    let json: JSON = serde_json::from_str(&data).unwrap();
    let mut filtered_projects: Vec<Project> = json
        .data
        .into_iter()
        .filter(|project| project.show)
        .collect();

    // Sort projects by title
    filtered_projects.sort_by_key(|x| x.date.clone());
    filtered_projects.reverse();
    return filtered_projects;
    // return json.data.sort_by_key(|x| x.date);
}
//TODO: Fix sort by date
pub fn get_context(get_featured: bool) -> Context {
    let gitraw_url: String = std::env::var("GITRAW_URL").unwrap();
    let mut ctx = Context::new();
    match get_featured {
        false => ctx.insert("projects", &update_page()),
        true => {
            let featured: Vec<_> = update_page()
                .into_iter()
                .filter(|project| project.featured)
                .collect();
            ctx.insert("featured_projects", &featured);
        }
    }
    ctx.insert("gitraw_url", &gitraw_url);
    return ctx;
}

// TODO: Refactor into get context with enum
pub(crate) fn get_project_ctx(slug: &str) -> Context {
    let gitraw_url: String = std::env::var("GITRAW_URL").unwrap();
    let youtube_url: String = std::env::var("YOUTUBE_URL").unwrap();
    let mut ctx = Context::new();

    let project = update_page()
        .into_iter()
        .find(|project| project.slug == slug)
        .unwrap();

    ctx.insert("project", &project);
    ctx.insert("gitraw_url", &gitraw_url);
    ctx.insert("youtube_url", &youtube_url);
    return ctx;
}

pub fn verify(msg: &str, key: &str) -> Result<(), String> {
    let private_key = std::env::var("SECRETKEY").unwrap();

    let mut hasher = HmacSha256::new_from_slice(private_key.as_ref()).unwrap();
    hasher.update(msg.as_ref());
    return match hasher.verify_slice(&*hex::decode(key).unwrap()) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    };
}
