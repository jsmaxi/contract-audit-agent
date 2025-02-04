use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use agents::{
    deduplication_agent::{FormatDeduplicationAgent, FormatDeduplicationAgentTrait},
    multi_agent_system::{MultiAIAgentSystem, MultiAIAgentSystemTrait},
};
use models::report::VulnerabilityReport;
use server::{
    request::AuditRequest,
    response::{AuditErrorResponse, AuditResponse},
};
use std::time::Instant;
mod agents;
mod config;
mod models;
mod server;

async fn audit_contract(
    request: web::Json<AuditRequest>,
    system: web::Data<MultiAIAgentSystem>,
) -> impl Responder {
    println!("Audit called");

    // Measure execution time
    let start = Instant::now();

    let request: AuditRequest = request.into_inner();

    if request.contract_code.trim().is_empty() || request.language.trim().is_empty() {
        let duration = start.elapsed();
        println!("Execution Time: {:?}", duration);
        return HttpResponse::BadRequest().json(AuditErrorResponse {
            error: "Request parameters cannot be empty".to_string(),
        });
    }

    // Analyze the contract in parallel
    let results = system.analyze_contract(&request.contract_code).await;

    // Print the intermediate results (uncomment if debugging)
    // for (agent_name, output) in results {
    //     println!("=== {} ===", agent_name);
    //     println!("{}", output);
    //     println!();
    // }

    // Deduplicate and format vulnerabilities list
    let fd_agent = FormatDeduplicationAgent::new();
    let values: Vec<&String> = results.values().collect();
    let json_dedup = fd_agent
        .format_and_deduplicate(values, system.client.clone())
        .await;

    // Print after deduplication (uncomment if debugging)
    // println!("Deduplicated: \n{:?}", json_dedup);

    // Parse into a report
    let report: VulnerabilityReport = FormatDeduplicationAgent::process_result(json_dedup);

    // Print a report (uncomment if debugging)
    println!("{:#?}", report);

    println!("Audit Done");

    let duration = start.elapsed();
    println!("Execution Time: {:?}", duration);

    HttpResponse::Ok().json(AuditResponse { report })
}

#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
    }))
}

async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome. Please call '/audit' endpoint with contract code string and contract language string.")
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/plain")
        .body("API route not found")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let server = "127.0.0.1";

    println!("Starting server {server} on port {port}");

    // Initialize the multi-AI agent system as app data
    // Note: Arc / Mutex for the future
    let system: MultiAIAgentSystem = MultiAIAgentSystem::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(system.clone()))
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(home))
            .route("/audit", web::post().to(audit_contract))
            .service(health_check)
            .default_service(web::route().to(not_found))
    })
    .bind((server, port))?
    .run()
    .await
}
