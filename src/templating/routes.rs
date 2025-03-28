// use headless_chrome;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

mod utils;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    message: String,
}

#[derive(Deserialize)]
struct TemplateRenderInput {
    template_id: String,
    content: serde_json::Value,
    prerender: Option<bool>,
}

#[post("/render-pdf")]
pub async fn render_pdf(job: web::Json<TemplateRenderInput>) -> impl Responder {
    let template_id = &job.template_id;
    let context = &job.content;
    let content = utils::render_template_from_dir(template_id, context);
    let prerender = &job.prerender.unwrap_or(false);

    // if (prerender) {
    //     let browser = headless_chrome::Browser::default().unwrap();
    //     let tab = browser.new_tab().unwrap();
    //     let data = format!("data:text/html,{}", &content.to_owned());
    //     tab.navigate_to(&data).unwrap();
    //     tab.wait_until_navigated().unwrap();
    //     let pdf_content = tab.print_to_pdf(None).unwrap();
//}
    //  HttpResponse::Ok()
    //     .content_type("application/pdf")
    //     .body(pdf_content)
    HttpResponse::Ok()
        .content_type("text/html")
        .body(content)
}
