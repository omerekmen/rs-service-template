use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/health" )]
#[tracing::instrument]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //shared::tracing_setup::init();

    HttpServer::new(|| {
        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(health)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
