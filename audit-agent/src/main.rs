use actix_cors::Cors;
use actix_web::{http, middleware, web, /*App,*/ HttpResponse, /*HttpServer,*/ Responder,};
use actix_web::{/*get,*/ web::ServiceConfig};
use agents::{
    deduplication_agent::{FormatDeduplicationAgent, FormatDeduplicationAgentTrait},
    multi_agent_system::{MultiAIAgentSystem, MultiAIAgentSystemTrait},
};
use models::report::VulnerabilityReport;
use server::{
    request::AuditRequest,
    response::{AuditErrorResponse, AuditResponse},
};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use std::env;
use std::time::Instant;
mod agents;
mod config;
mod models;
mod server;

#[actix_web::post("/audit")]
async fn audit_contract(
    request: web::Json<AuditRequest>,
    system: web::Data<MultiAIAgentSystem>,
) -> impl Responder {
    println!("Audit called");

    // Measure execution time
    let start = Instant::now();

    let request: AuditRequest = request.into_inner();

    if request.contract_code.trim().is_empty()
        || request.language.trim().is_empty()
        || request.model.trim().is_empty()
    {
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

#[actix_web::get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome. Please call (post) '/audit' endpoint with contract code string and contract language string.")
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/plain")
        .body("API route not found")
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // let port = 8080;
    // let server = "127.0.0.1";
    // println!("Starting server {server} on port {port}");

    // Get secret from server
    let api_key: String = secrets
        .get("OPENAI_API_KEY")
        .expect("Secret API KEY is not set");
    // Set environment variable for GenAI
    std::env::set_var("OPENAI_API_KEY", api_key);

    // Initialize the multi-AI agent system as app data
    // Note: Arc / Mutex for the future
    let system: MultiAIAgentSystem = MultiAIAgentSystem::new();

    // Allowed caller
    let allowed_origin1 = "http://localhost:3000";
    let allowed_origin2 = "https://contract-audit-ui.vercel.app";
    let allowed_origin3 = "https://contract-audit-ui-production.up.railway.app";

    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::default()
            .allowed_origin(allowed_origin1)
            .allowed_origin(allowed_origin2)
            .allowed_origin(allowed_origin3)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .max_age(86400);

        cfg.service(
            web::scope("/")
                .wrap(cors) // Apply the CORS middleware
                .wrap(middleware::Compress::default()),
        )
        .app_data(web::Data::new(system.clone()))
        .service(home)
        .service(audit_contract)
        .service(health_check)
        .default_service(web::route().to(not_found));
    };

    Ok(config.into())

    // HttpServer::new(move || {
    //     let cors = Cors::default()
    //         .allowed_origin(allowed_origin)
    //         .allowed_methods(vec!["GET", "POST"])
    //         .allowed_headers(vec![
    //             http::header::AUTHORIZATION,
    //             http::header::CONTENT_TYPE,
    //         ])
    //         .max_age(3600);

    //     App::new()
    //         .app_data(web::Data::new(system.clone()))
    //         .wrap(middleware::Compress::default())
    //         .wrap(cors) // Apply the CORS middleware
    //         .route("/", web::get().to(home))
    //         .route("/audit", web::post().to(audit_contract))
    //         .service(health_check)
    //         .default_service(web::route().to(not_found))
    // })
    // .bind((server, port))?
    // .run()
    // .await
}
