use crate::agents::multi_agent_system::{MultiAIAgentSystem, MultiAIAgentSystemTrait};
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let server = "127.0.0.1";

    println!("Starting server {server} on port {port}");

    let system: MultiAIAgentSystem = MultiAIAgentSystem::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(system.clone()))
            // .route("/audit", web::post().to(audit_contract))
            .service(health_check)
    })
    .bind((server, port))?
    .workers(2)
    .run()
    .await
}
