use sailfish::TemplateOnce;

#[derive(TemplateOnce)]  // automatically implement `TemplateOnce` trait
#[template(path = "hello.stpl")]  // specify the path to template
struct HelloTemplate {
    // data to be passed to the template
    messages: Vec<String>,
}
pub fn new_temp() -> String {
    let ctx = HelloTemplate {
        messages: vec![String::from("foo"), String::from("bar")],
    };
    return ctx.render_once().unwrap();
}
