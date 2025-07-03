use actix_web::{HttpResponse, Responder, post, web};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use serde::{Deserialize, Serialize};

mod utils;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    message: String,
}

#[derive(Deserialize)]
struct TemplateRenderInput {
    template_id: String,
    data: serde_json::Value,
    prerender: Option<bool>,
}

#[post("/render-template")]
pub async fn render_pdf(job: web::Json<TemplateRenderInput>) -> impl Responder {
    let template_id = &job.template_id;
    let context = &job.data;
    let html_string = utils::render_template_from_dir(template_id, context);
    let prerender = job.prerender.unwrap_or(false);

    println!("Rendering template: {}", template_id);
    if prerender {
        let pdf_bytes_result: Result<Vec<u8>, pyo3::PyErr> = Python::with_gil(|py| {
            let weasyprint_module = PyModule::import(py, "weasyprint")?;
            let html_class = weasyprint_module.getattr("HTML")?;

            // Create an HTML object from the string
            let html_obj = html_class.call1((PyString::new(py, &html_string),))?;

            // Render to PDF bytes
            let write_pdf_method = html_obj.getattr("write_pdf")?;
            let pdf_data = write_pdf_method.call0()?;

            // Convert the Python bytes object to a Rust Vec<u8>
            let bytes = pdf_data.downcast::<PyBytes>()?;
            Ok(bytes.as_bytes().to_vec())
        });

        match pdf_bytes_result {
            Ok(pdf_bytes) => {
                // Now you have the PDF data as a Vec<u8> in Rust
                println!("Successfully generated PDF with {} bytes.", pdf_bytes.len());

                // You can now save this vector to a file, send it over a network, etc.
                // For demonstration, let's save it to a file:
                std::fs::write("output.pdf", &pdf_bytes).expect("Failed to write PDF to file");
                HttpResponse::Ok()
                    .content_type("application/pdf")
                    .body(pdf_bytes)
            }
            Err(e) => {
                eprintln!("Failed to generate PDF: {:?}", e);
                HttpResponse::InternalServerError().body("Failed to generate PDF")
            }
        }
    } else {
        HttpResponse::Ok().content_type("text/html").body(html_string)
    }
}

// #[post("/render-template")]
// pub async fn render_pdf(job: web::Json<TemplateRenderInput>) -> impl Responder {
//     let template_id = &job.template_id;
//     let context = &job.data;
//     let content = utils::render_template_from_dir(template_id, context);
//     let prerender = job.prerender.unwrap_or(false);

//     println!("Rendering template: {}", template_id);
//     if prerender {
//         let launch_options = headless_chrome::LaunchOptionsBuilder::default()
//             .headless(true)
//             .sandbox(false)
//             .build()
//             .unwrap();
//         let browser = headless_chrome::Browser::new(launch_options).unwrap();
//         let tab = browser.new_tab().unwrap();
//         let data = format!("data:text/html,{}", &content.to_owned());
//         tab.navigate_to(&data).unwrap();
//         tab.wait_until_navigated().unwrap();
//         let pdf_content = tab.print_to_pdf(None).unwrap();
//         HttpResponse::Ok()
//             .content_type("application/pdf")
//             .body(pdf_content)
//     } else {
//         HttpResponse::Ok().content_type("text/html").body(content)
//     }
// }

// #[post("/render-template")]
// pub async fn render_pdf(job: web::Json<TemplateRenderInput>) -> impl Responder {
//     let template_id = &job.template_id;
//     let context = &job.data;
//     let content = utils::render_template_from_dir(template_id, context);
//     let prerender = job.prerender.unwrap_or(false);

//     println!("Rendering template: {}", template_id);
//     if prerender {
//         println!("Here we got 0");
//         let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");
//         println!("Here we got 1");
//         let mut pdfout = pdf_app.builder()
//             .orientation(Orientation::Portrait)
//             .margin(Size::Inches(1))
//             .title("Awesome Foo")
//             .build_from_html(&content)
//             .expect("failed to build pdf");

//         println!("Here we got 2");
//         pdfout.save("foo.pdf").expect("failed to save foo.pdf");
//         println!("Here we got 3");
//         let mut pdf_content = Vec::new();
//         pdfout
//             .read_to_end(&mut pdf_content)
//             .expect("failed to read PDF bytes");
//         HttpResponse::Ok()
//             .content_type("application/pdf")
//             .body(pdf_content)
//     } else {
//         HttpResponse::Ok().content_type("text/html").body(content)
//     }
// }

// fn main() -> PyResult<()> {
//     // Initialize the Python interpreter

//     let html_string = r#"
//         <!DOCTYPE html>
//         <html>
//         <head>
//             <title>My Document</title>
//             <style>
//                 body { font-family: sans-serif; }
//                 h1 { color: navy; }
//             </style>
//         </head>
//         <body>
//             <h1>Hello from WeasyPrint!</h1>
//             <p>This is a paragraph generated from an HTML string.</p>
//             <p>Date: 2025-07-02</p>
//         </body>
//         </html>
//     "#;

//     let pdf_bytes: Result<Vec<u8>, pyo3::PyErr> = Python::with_gil(|py| {
//         let weasyprint_module = PyModule::import(py, "weasyprint")?;
//         let html_class = weasyprint_module.getattr("HTML")?;

//         // Create an HTML object from the string
//         let html_obj = html_class.call1((PyString::new(py, html_string),))?;

//         // Render to PDF bytes
//         let write_pdf_method = html_obj.getattr("write_pdf")?;
//         let pdf_data = write_pdf_method.call0()?;

//         // Convert the Python bytes object to a Rust Vec<u8>
//         let bytes = pdf_data.downcast::<PyBytes>()?;
//         Ok(bytes.as_bytes().to_vec())
//     });
//     let pdf_bytes = pdf_bytes?;

//     // Now you have the PDF data as a Vec<u8> in Rust
//     println!("Successfully generated PDF with {} bytes.", pdf_bytes.len());

//     // You can now save this vector to a file, send it over a network, etc.
//     // For demonstration, let's save it to a file:
//     std::fs::write("output.pdf", &pdf_bytes)
//         .expect("Failed to write PDF to file");
//     println!("PDF saved as output.pdf");

//     Ok(())
// }
