use tera::Context;

pub fn get_social_links() -> Context {
    let linkedin_url: String = std::env::var("LINKEDIN_URL").unwrap();
    let github_url: String = std::env::var("GITHUB_URL").unwrap();
    let mut ctx = Context::new();
    ctx.insert("linkedin_url", &linkedin_url);
    ctx.insert("github_url", &github_url);
    return ctx;
}