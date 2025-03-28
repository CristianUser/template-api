use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[path = "templating/routes.rs"]
mod routes;

use routes::{render_pdf};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Health {
    status: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(Health { status: "ok".to_owned() })
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        let cors = Cors::permissive(); // Change this to configure your CORS settings

        App::new()
            .wrap(cors)
            .service(health)
            .service(render_pdf)
    })
    .bind(("0.0.0.0", 5000))?;

    println!("Listening on port 5000");
    server.run().await
}
